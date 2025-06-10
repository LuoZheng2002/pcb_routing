use std::{cell::RefCell, collections::{BTreeSet, HashMap, HashSet}};

use crate::{dijkstra::{DijkstraModel, Direction}, grid::{Point, PointPair}, hyperparameters::{HALF_PROBABILITY_RAW_SCORE, LENGTH_PENALTY_RATE, TURN_PENALTY_RATE}, proba_grid::{NetID, PadPairToRoute, PadPairToRouteID, ProbaGridInput, ProbaGridOutput, Trace, TraceID, TraceProbaInfo}};



pub fn calculate_trace_score(covered: &BTreeSet<Point>, directions: &Vec<Direction>) -> f64{
    let total_length = covered.len();
    let mut turns = 0;
    let mut last_direction = directions.first().cloned().unwrap_or(Direction { x: 0, y: 0 });
    if directions.len() > 0{
        for direction in directions.iter().skip(1) {
            if direction.x != last_direction.x || direction.y != last_direction.y {
                turns += 1;
            }
            last_direction = direction.clone();
        }
    }
    let score_raw = total_length as f64* LENGTH_PENALTY_RATE + turns as f64 * TURN_PENALTY_RATE;
    let k = f64::ln(2.0)/ HALF_PROBABILITY_RAW_SCORE;
    let score = f64::exp(-k * score_raw);
    score
} 


pub fn first_iteration_prior(
    input: ProbaGridInput,
) -> Result<ProbaGridOutput, String> {
    // Initialize the output structure
    let ProbaGridInput {
        width,
        height,
        nets,
        pads,
    } = input;
    let mut next_pair_id = 0;
    let mut get_next_pair_id = || {
        let id = PadPairToRouteID(next_pair_id);
        next_pair_id += 1;
        id
    };
    let mut pad_pairs_to_route:HashMap<NetID, HashMap<PadPairToRouteID, PadPairToRoute>> = pads
        .iter()
        .map(|(net_id, pad_set)| {
            let mut pairs = HashMap::new();
            let points_vec: Vec<_> = pad_set.iter().cloned().collect();
            for i in 0..points_vec.len() {
                for j in (i + 1)..points_vec.len() {
                    let point_pair = PointPair::new(points_vec[i], points_vec[j]);                    
                    let start = point_pair.start();
                    let end = point_pair.end();
                    assert_ne!(start, end, "Pad pair start and end points must be different");
                    let pad_pair_id = get_next_pair_id();
                    let route = PadPairToRoute {
                        net_id: net_id.clone(),
                        pad_pair_id: pad_pair_id.clone(),
                        start,
                        end,
                    };
                    pairs.insert(pad_pair_id, route);
                }
            }
            (net_id.clone(), pairs)
        }).collect();
    
    // first iteration: get all directly connected pad pairs
    let mut visited_traces: HashMap<PadPairToRouteID, HashSet<Trace>> = HashMap::new();
    let mut traces: HashMap<PadPairToRouteID, HashMap<TraceID, Trace>> = HashMap::new();
    let mut trace_proba_infos: HashMap<TraceID, TraceProbaInfo> = HashMap::new();
    let mut next_trace_id = 0;
    let mut get_next_trace_id = || {
        let id = TraceID(next_trace_id);
        next_trace_id += 1;
        id
    };
    for pad_pairs in pad_pairs_to_route.values_mut() {
        for (pad_pair_id, pair) in pad_pairs.iter_mut() {
            let trace_id = get_next_trace_id();
            let dijkstra_model = DijkstraModel {
                width,
                height,
                obstacles: pads.iter()
                    .filter(|(net_id, _)| **net_id != pair.net_id)
                    .flat_map(|(_, points)| points.iter().cloned())
                    .collect(),
                diagonal_obstacles: HashSet::new(), // No diagonal obstacles in the first iteration
                start: pair.start,
                end: pair.end,
            };
            let result = dijkstra_model.run().map_err(|e| format!("Dijkstra's algorithm failed: {}", e))?;
            let covered_vec = result.covered.iter().cloned().collect::<Vec<_>>();
            let trace = Trace {
                net_id: pair.net_id,
                pad_pair_id: *pad_pair_id,
                trace_id,
                start: pair.start,
                end: pair.end,
                covered: covered_vec, // Initially just the start and end points
            };
            let trace_set = visited_traces.entry(pad_pair_id.clone()).or_default();
            if trace_set.insert(trace.clone()){
                // If the trace was not already visited, insert it into the traces map
                let trace_id = get_next_trace_id();
                let prior_probability = 1.0; // Initial prior probability for the first iteration is 100%
                let score = calculate_trace_score(&result.covered, &result.directions);
                let prior_anchor = prior_probability * score; // Initial prior anchor
                let trace_proba_info = TraceProbaInfo {
                    net_id: pair.net_id,
                    pad_pair_id: *pad_pair_id,
                    trace_id,
                    prior_probability,
                    score, // Initial score
                    prior_anchor,
                    old_posterior: RefCell::new(None), // No old posterior in the first iteration
                    new_posterior: RefCell::new(None), // No new posterior in the first iteration
                };
                traces.entry(pad_pair_id.clone())
                    .or_default()
                    .insert(trace_id.clone(), trace);
                trace_proba_infos.insert(trace_id.clone(), trace_proba_info);
            }else{
                panic!("Trace already exists for pad pair ID: {:?}", pad_pair_id);
            }
        }
    }
    let mut trace_collision_adjacency: HashMap<TraceID, HashSet<TraceID>> = HashMap::new();
    let traces_indexed_from_net: HashMap<NetID, HashMap<TraceID, Trace>> = traces.values()
        .flat_map(|trace_map| {
            trace_map.values()
                .map(|trace| (trace.net_id, trace.trace_id, trace.clone()))
        })
        .fold(HashMap::new(), |mut acc, (net_id, trace_id, trace)| {
            acc.entry(net_id).or_default().insert(trace_id, trace);
            acc
        });
    let traces_vec = traces_indexed_from_net
        .into_values()
        .collect::<Vec<_>>();
    // permutate sets according to the net ids
    for i in 0..traces_vec.len() {
        for j in (i + 1)..traces_vec.len() {
            let trace_set_a = &traces_vec[i];
            let trace_set_b = &traces_vec[j];
            for (trace_id_a, trace_a) in trace_set_a {
                for (trace_id_b, trace_b) in trace_set_b {
                    let trace_a_covered = &trace_a.covered;
                    let trace_b_covered = &trace_b.covered;
                    let mut collide = false;
                    for point in trace_a_covered {
                        if trace_b_covered.contains(point) {
                            collide = true;
                            break;
                        }
                    }
                    if collide{
                        trace_collision_adjacency
                            .entry(trace_id_a.clone())
                            .or_default()
                            .insert(trace_id_b.clone());
                        trace_collision_adjacency
                            .entry(trace_id_b.clone())
                            .or_default()
                            .insert(trace_id_a.clone());
                    }
                }
            }
        }
    }

    // Prepare the output structure
    let output = ProbaGridOutput {
        width,
        height,
        nets,
        pads,
        pad_pairs_to_route,
        visited_traces,
        traces,
        trace_proba_infos,
        trace_collision_adjacency,
    };
    Ok(output)
}

pub fn update_posterior(output: &mut ProbaGridOutput) -> Result<(), String> {
    // Update the posterior probabilities based on the prior anchor and collision adjacency
    let ProbaGridOutput{
        trace_proba_infos,
        trace_collision_adjacency,
        ..  
    } = output;
    for (trace_id, collision_set) in trace_collision_adjacency.iter() {
        let target_trace_info = trace_proba_infos.get(trace_id)
            .ok_or_else(|| format!("Trace ID {:?} not found in trace_proba_infos", trace_id))?;
        let prior_anchor = target_trace_info.prior_anchor;
        let mut proba_product = 1.0;
        for collision_id in collision_set {
            let trace_proba_info = trace_proba_infos.get(collision_id)
                .ok_or_else(|| format!("Collision Trace ID {:?} not found in trace_proba_infos", collision_id))?;
            let old_posterior = trace_proba_info.new_posterior.borrow().clone();
            let proba = if let Some(old_posterior) = old_posterior {
                old_posterior
            } else {
                trace_proba_info.prior_anchor
            };
            let one_minus_proba = 1.0 - proba;
            assert!(one_minus_proba > 0.0, "One minus probability must be greater than 0");
            proba_product *= one_minus_proba;
        }
        let evidence_proba = 1.0-proba_product;
        assert!(evidence_proba >= 0.0 && evidence_proba <= 1.0, "Evidence probability must be between 0 and 1");
        let posterior = f64::sqrt(prior_anchor * evidence_proba);
        let mut new_posterior = target_trace_info.new_posterior.borrow_mut();
        *new_posterior = Some(posterior);
    }
    for trace_proba_info in trace_proba_infos.values_mut() {
        let mut old_posterior = trace_proba_info.old_posterior.borrow_mut();
        let mut new_posterior = trace_proba_info.new_posterior.borrow_mut();
        *old_posterior = new_posterior.clone();
        *new_posterior = None; // Clear the new posterior for the next iteration
    }
    Ok(())
}