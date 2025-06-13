use std::{cell::RefCell, collections::{BTreeSet, HashMap, HashSet}, num::NonZeroUsize};

use crate::{dijkstra::{DijkstraModel, Direction}, grid::{Point, PointPair}, hyperparameters::{HALF_PROBABILITY_RAW_SCORE, LENGTH_PENALTY_RATE, TURN_PENALTY_RATE}, proba_grid::{NetID, PadPair, PadPairID, ProbaGridProblem, ProbaGrid, TraceInfo, TraceID, TraceProbaInfo}};



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
    assert!(score >= 0.0 && score <= 1.0, "Score must be between 0 and 1, got: {}", score);
    score
} 


pub fn first_iteration_prior(
    input: ProbaGridProblem,
) -> Result<ProbaGrid, String> {
    // Initialize the output structure
    let ProbaGridProblem {
        width,
        height,
        nets,
        net_to_pads: pads,
    } = input;
    let mut next_pair_id = 0;
    let mut get_next_pair_id = || {
        let id = PadPairID(next_pair_id);
        next_pair_id += 1;
        id
    };
    let mut pad_pairs_to_route:HashMap<NetID, HashMap<PadPairID, PadPair>> = pads
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
                    let route = PadPair {
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
    let mut visited_traces: HashMap<PadPairID, HashSet<TraceInfo>> = HashMap::new();
    let mut traces: HashMap<PadPairID, HashMap<TraceID, TraceInfo>> = HashMap::new();
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
            let trace = TraceInfo {
                net_id: pair.net_id,
                pad_pair_id: *pad_pair_id,
                trace_id,
                start: pair.start,
                end: pair.end,
                covered: covered_vec, // Initially just the start and end points
            };
            let trace_set = visited_traces.entry(pad_pair_id.clone()).or_default();
            if trace_set.insert(trace.clone()){
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
                println!("Trace created for pad pair ID: {:?}, trace ID: {:?}", pad_pair_id, trace_id);
            }else{
                panic!("Trace already exists for pad pair ID: {:?}", pad_pair_id);
            }
        }
    }
    let mut trace_collision_adjacency: HashMap<TraceID, HashSet<TraceID>> = HashMap::new();
    let traces_indexed_from_net: HashMap<NetID, HashMap<TraceID, TraceInfo>> = traces.values()
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
    println!("Trace proba info count: {}", trace_proba_infos.len());


    // Prepare the output structure
    let output = ProbaGrid {
        width,
        height,
        nets,
        net_to_pads: pads,
        net_to_pad_pairs: pad_pairs_to_route,
        visited_traces,
        traces,
        trace_proba_infos,
        trace_collision_adjacency,
    };
    Ok(output)
}

pub fn update_posterior(output: &mut ProbaGrid, coefficient: f64) -> Result<(), String> {
    // Update the posterior probabilities based on the prior anchor and collision adjacency
    let ProbaGrid{
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
            let old_posterior = trace_proba_info.old_posterior.borrow().clone();
            let proba = if let Some(old_posterior) = old_posterior {
                println!("Using old posterior for trace ID {:?}: {:?}", collision_id, old_posterior);
                old_posterior
            } else {
                println!("Old posterior for trace ID {:?} is None, using prior anchor", collision_id);
                trace_proba_info.prior_anchor
            };
            let one_minus_proba = 1.0 - proba;
            assert!(one_minus_proba > 0.0, "One minus probability must be greater than 0");
            proba_product *= one_minus_proba;
        }
        let evidence_proba = proba_product;
        assert!(evidence_proba >= 0.0 && evidence_proba <= 1.0, "Evidence probability must be between 0 and 1");
        // let posterior = f64::sqrt(prior_anchor * evidence_proba);
        let posterior = f64::powf(prior_anchor, 1.0 - coefficient) * f64::powf(evidence_proba, coefficient);
        let mut new_posterior = target_trace_info.new_posterior.borrow_mut();
        *new_posterior = Some(posterior);
    }
    for trace_proba_info in trace_proba_infos.values_mut() {
        let mut old_posterior = trace_proba_info.old_posterior.borrow_mut();
        let mut new_posterior = trace_proba_info.new_posterior.borrow_mut();
        assert!(new_posterior.is_some(), "New posterior must be set before updating old posterior");
        *old_posterior = new_posterior.clone();
        *new_posterior = None; // Clear the new posterior for the next iteration
        println!("Updated trace ID: {:?}, posterior: {:?}", trace_proba_info.trace_id, old_posterior);
    }
    Ok(())
}

pub fn initialize_proba_grid(input: ProbaGridProblem) -> Result<ProbaGrid, String> {
    let ProbaGridProblem {
        width,
        height,
        nets,
        net_to_pads,
    } = input;

    let mut pad_pair_id_generator = (0..).map(PadPairID);
    let mut pad_pairs: HashMap<PadPairID, PadPair> = HashMap::new();
    let net_to_pad_pairs = net_to_pads
        .iter()
        .map(|(net_id, pad_set)| {
            let mut pairs_set = HashSet::new();
            let points_vec: Vec<_> = pad_set.iter().cloned().collect();
            for i in 0..points_vec.len() {
                for j in (i + 1)..points_vec.len() {
                    let point_pair = PointPair::new(points_vec[i], points_vec[j]);
                    let start = point_pair.start();
                    let end = point_pair.end();
                    assert_ne!(start, end, "Pad pair start and end points must be different");
                    let pad_pair_id = pad_pair_id_generator.next().unwrap();
                    let pad_pair = PadPair {
                        net_id: *net_id,
                        pad_pair_id,
                        start,
                        end,
                    };
                    pairs_set.insert(pad_pair_id);
                    pad_pairs.insert(pad_pair_id, pad_pair);
                }
            }
            (net_id.clone(), pairs_set)
        })
        .collect();
    
    let grid = ProbaGrid {
        width,
        height,
        nets,
        net_to_pads,
        net_to_pad_pairs,
        pad_pairs,
        visited_traces: BTreeSet::new(),
        traces: HashMap::new(),
        pad_pair_to_traces: HashMap::new(),
        trace_collision_adjacency: HashMap::new(),
        next_iteration: NonZeroUsize::new(1).unwrap(), // Start with the first iteration
    };
    Ok(grid)
}

// sample, for each pair_to_route, consider all pair_to_route from other nets, 
// and each pair_to_route can choose one trace from all its registered traces, or it can choose nothing (can be a trace that hasn't been registered)
// other trace rate: ?
// 
// there must be a sequence
// our good wish ...

// ratio: anchor is 1, and can be pulled by score and opportunity cost (probability determined by other traces / its current allocated probability)
// and then all normalized to ...

// how to pull in between different iterations? -> Determined by the last iteration.
// the average score, and average opportunity cost

// straight: 70% (pull: score, opportunity cost), detour once: 30%*70%, detour twice: ...
// all traces belonging to "detour once" will be grouped together and has a total probability of 1-sum of straight probability
// the sum probability will be allocated based on score, 
pub fn sample_new_traces(grid: &mut ProbaGrid)-> Result<(), String>{
    let ProbaGrid {
        width,
        height,
        nets,
        net_to_pads,
        net_to_pad_pairs,
        pad_pairs,
        visited_traces,
        traces,
        pad_pair_to_traces,
        trace_collision_adjacency,
        next_iteration,
    } = grid;
    for (net_id, pad_pair_ids) in net_to_pad_pairs.iter() {
        // randomly generate a trace for each pad pair of other nets (in a rare case the trace will not be generated)
        let obstacle_traces: HashMap<PadPairID, Option<TraceID>> = net_to_pad_pairs.iter()
            .filter(|(other_net_id, _)| *other_net_id != net_id)
            .flat_map(|(_, pad_pair_ids)| {
                pad_pair_ids.iter().map(|pad_pair_id| {
                    let candidate_trace_ids = pad_pair_to_traces.get(pad_pair_id).unwrap();
                    let mut sum_probability = 0.0;
                    for candidate_trace_id in candidate_trace_ids.iter(){
                        let candidate_trace = traces.get(candidate_trace_id).unwrap();
                        
                    }
                    // candidate traces have traces of iterations from 1

                    // how to make the probability of not choosing any trace to be variable?
                    // don't make them to be variable

                    // we need the trace's probability

                    let trace_id = TraceID(next_iteration.get() as usize);
                    (pad_pair_id.clone(), Some(trace_id))
                })
            })
            .collect();
        for pad_pair_id in pad_pair_ids.iter() {
            let pad_pair = pad_pairs.get(pad_pair_id).ok_or_else(|| format!("PadPairID {:?} not found in net_to_pad_pairs", pad_pair_id))?;
            // let trace_id = TraceID(next_iteration.get() as usize);
            let dijkstra_model = DijkstraModel {
                width: *width,
                height: *height,
                obstacles: net_to_pads.iter()
                    .filter(|(net_id, _)| **net_id != pad_pair.net_id)
                    .flat_map(|(_, points)| points.iter().cloned())
                    .collect(),
                diagonal_obstacles: HashSet::new(), // No diagonal obstacles in the first iteration
                start: pad_pair.start,
                end: pad_pair.end,
            };
        }
    }
    todo!()
}