use std::{cell::RefCell, collections::{BTreeSet, HashMap, HashSet}, num::NonZeroUsize};

use rand::distr::{weighted::WeightedIndex};
use rand::prelude::*;

use crate::{dijkstra::DijkstraModel, grid::{Point, PointPair}, hyperparameters::{HALF_PROBABILITY_RAW_SCORE, LENGTH_PENALTY_RATE, MAX_GENERATION_ATTEMPTS, MAX_TRACES_PER_ITERATION, OPPORTUNITY_COST_WEIGHT, SCORE_WEIGHT, TURN_PENALTY_RATE}, proba_grid::{IterationNum, NetID, PadPair, PadPairID, ProbaGrid, ProbaGridProblem, TraceID, TraceInfo}};



// pub fn first_iteration_prior(
//     input: ProbaGridProblem,
// ) -> Result<ProbaGrid, String> {
//     // Initialize the output structure
//     let ProbaGridProblem {
//         width,
//         height,
//         nets,
//         net_to_pads: pads,
//     } = input;
//     let mut next_pair_id = 0;
//     let mut get_next_pair_id = || {
//         let id = PadPairID(next_pair_id);
//         next_pair_id += 1;
//         id
//     };
//     let mut pad_pairs_to_route:HashMap<NetID, HashMap<PadPairID, PadPair>> = pads
//         .iter()
//         .map(|(net_id, pad_set)| {
//             let mut pairs = HashMap::new();
//             let points_vec: Vec<_> = pad_set.iter().cloned().collect();
//             for i in 0..points_vec.len() {
//                 for j in (i + 1)..points_vec.len() {
//                     let point_pair = PointPair::new(points_vec[i], points_vec[j]);                    
//                     let start = point_pair.start();
//                     let end = point_pair.end();
//                     assert_ne!(start, end, "Pad pair start and end points must be different");
//                     let pad_pair_id = get_next_pair_id();
//                     let route = PadPair {
//                         net_id: net_id.clone(),
//                         pad_pair_id: pad_pair_id.clone(),
//                         start,
//                         end,
//                     };
//                     pairs.insert(pad_pair_id, route);
//                 }
//             }
//             (net_id.clone(), pairs)
//         }).collect();
    
//     // first iteration: get all directly connected pad pairs
//     let mut visited_traces: HashMap<PadPairID, HashSet<TraceInfo>> = HashMap::new();
//     let mut traces: HashMap<PadPairID, HashMap<TraceID, TraceInfo>> = HashMap::new();
//     let mut trace_proba_infos: HashMap<TraceID, TraceProbaInfo> = HashMap::new();
//     let mut next_trace_id = 0;
//     let mut get_next_trace_id = || {
//         let id = TraceID(next_trace_id);
//         next_trace_id += 1;
//         id
//     };
//     for pad_pairs in pad_pairs_to_route.values_mut() {
//         for (pad_pair_id, pair) in pad_pairs.iter_mut() {
//             let trace_id = get_next_trace_id();
//             let dijkstra_model = DijkstraModel {
//                 width,
//                 height,
//                 obstacles: pads.iter()
//                     .filter(|(net_id, _)| **net_id != pair.net_id)
//                     .flat_map(|(_, points)| points.iter().cloned())
//                     .collect(),
//                 diagonal_obstacles: HashSet::new(), // No diagonal obstacles in the first iteration
//                 start: pair.start,
//                 end: pair.end,
//             };
//             let result = dijkstra_model.run().map_err(|e| format!("Dijkstra's algorithm failed: {}", e))?;
//             let covered_vec = result.covered.iter().cloned().collect::<Vec<_>>();
//             let trace = TraceInfo {
//                 net_id: pair.net_id,
//                 pad_pair_id: *pad_pair_id,
//                 trace_id,
//                 start: pair.start,
//                 end: pair.end,
//                 covered: covered_vec, // Initially just the start and end points
//             };
//             let trace_set = visited_traces.entry(pad_pair_id.clone()).or_default();
//             if trace_set.insert(trace.clone()){
//                 let prior_probability = 1.0; // Initial prior probability for the first iteration is 100%
//                 let score = calculate_trace_score(&result.covered, &result.directions);
//                 let prior_anchor = prior_probability * score; // Initial prior anchor
//                 let trace_proba_info = TraceProbaInfo {
//                     net_id: pair.net_id,
//                     pad_pair_id: *pad_pair_id,
//                     trace_id,
//                     prior_probability,
//                     score, // Initial score
//                     prior_anchor,
//                     old_posterior: RefCell::new(None), // No old posterior in the first iteration
//                     new_posterior: RefCell::new(None), // No new posterior in the first iteration
//                 };
//                 traces.entry(pad_pair_id.clone())
//                     .or_default()
//                     .insert(trace_id.clone(), trace);
//                 trace_proba_infos.insert(trace_id.clone(), trace_proba_info);
//                 println!("Trace created for pad pair ID: {:?}, trace ID: {:?}", pad_pair_id, trace_id);
//             }else{
//                 panic!("Trace already exists for pad pair ID: {:?}", pad_pair_id);
//             }
//         }
//     }
//     let mut trace_collision_adjacency: HashMap<TraceID, HashSet<TraceID>> = HashMap::new();
//     let traces_indexed_from_net: HashMap<NetID, HashMap<TraceID, TraceInfo>> = traces.values()
//         .flat_map(|trace_map| {
//             trace_map.values()
//                 .map(|trace| (trace.net_id, trace.trace_id, trace.clone()))
//         })
//         .fold(HashMap::new(), |mut acc, (net_id, trace_id, trace)| {
//             acc.entry(net_id).or_default().insert(trace_id, trace);
//             acc
//         });
//     let traces_vec = traces_indexed_from_net
//         .into_values()
//         .collect::<Vec<_>>();
//     // permutate sets according to the net ids
//     for i in 0..traces_vec.len() {
//         for j in (i + 1)..traces_vec.len() {
//             let trace_set_a = &traces_vec[i];
//             let trace_set_b = &traces_vec[j];
//             for (trace_id_a, trace_a) in trace_set_a {
//                 for (trace_id_b, trace_b) in trace_set_b {
//                     let trace_a_covered = &trace_a.covered;
//                     let trace_b_covered = &trace_b.covered;
//                     let mut collide = false;
//                     for point in trace_a_covered {
//                         if trace_b_covered.contains(point) {
//                             collide = true;
//                             break;
//                         }
//                     }
//                     if collide{
//                         trace_collision_adjacency
//                             .entry(trace_id_a.clone())
//                             .or_default()
//                             .insert(trace_id_b.clone());
//                         trace_collision_adjacency
//                             .entry(trace_id_b.clone())
//                             .or_default()
//                             .insert(trace_id_a.clone());
//                     }
//                 }
//             }
//         }
//     }
//     println!("Trace proba info count: {}", trace_proba_infos.len());


//     // Prepare the output structure
//     let output = ProbaGrid {
//         width,
//         height,
//         nets,
//         net_to_pads: pads,
//         net_to_pad_pairs: pad_pairs_to_route,
//         visited_traces,
//         traces,
//         trace_proba_infos,
//         trace_collision_adjacency,
//     };
//     Ok(output)
// }

pub fn update_posterior(grid: &mut ProbaGrid) -> Result<(), String> {
    // // Update the posterior probabilities based on the prior anchor and collision adjacency
    // let ProbaGrid {
    //     width,
    //     height, 
    //     nets, 
    //     net_to_pads, 
    //     net_to_pad_pairs, 
    //     pad_pairs, 
    //     visited_traces, 
    //     pad_pair_to_traces, 
    //     traces, 
    //     trace_collision_adjacency, 
    //     next_iteration 
    // } = grid;
    for (trace_id, trace_info) in grid.traces.iter() {
        let adjacent_traces = grid.trace_collision_adjacency.get(trace_id)
            .ok_or_else(|| format!("Trace ID {:?} not found in trace_collision_adjacency", trace_id))?;
        let mut proba_product = 1.0;
        for adjacent_trace_id in adjacent_traces{
            let adjacent_trace_info = grid.traces.get(adjacent_trace_id)
                .ok_or_else(|| format!("Adjacent Trace ID {:?} not found in traces", adjacent_trace_id))?;
            // get num traces in the same iteration
            let num_traces_in_the_same_iteration = grid.get_num_traces_in_the_same_iteration(*adjacent_trace_id);
            let probability_normalized = adjacent_trace_info.get_posterior_normalized_with_fallback(num_traces_in_the_same_iteration);
            let one_minus_proba = 1.0 - probability_normalized;
            assert!(one_minus_proba > 0.0, "One minus probability must be greater than 0");
            proba_product *= one_minus_proba;
        }
        let target_posterior = proba_product;
        assert!(target_posterior >= 0.0 && target_posterior <= 1.0, "Target posterior must be between 0 and 1");
        // get num traces in the same iteration
        let num_traces_in_the_same_iteration = grid.get_num_traces_in_the_same_iteration(*trace_id);
        let current_posterior = trace_info.get_posterior_normalized_with_fallback(num_traces_in_the_same_iteration);
        let opportunity_cost = target_posterior / current_posterior;
        let score = trace_info.get_score();
        let score_weight = *SCORE_WEIGHT.lock().unwrap();
        let opportunity_cost_weight = *OPPORTUNITY_COST_WEIGHT.lock().unwrap();
        let posterior_unnormalized = 1.0*f64::powf(score, score_weight) * f64::powf(opportunity_cost, opportunity_cost_weight);
        let posterior_normalized = trace_info.get_normalized_prior_probability(num_traces_in_the_same_iteration) * posterior_unnormalized;
        let mut temp_posterior = trace_info.temp_posterior.borrow_mut();
        *temp_posterior = Some(posterior_normalized);
    }
    // convert the temp posterior to the final posterior
    for (_pad_pair_id, trace_ids) in grid.pad_pair_to_traces.iter(){
        // trace_infos in the same pad pair
        let trace_infos = trace_ids.iter()
            .flat_map(|(_, trace_ids)| trace_ids.iter())
            .map(|trace_id|{
                grid.traces.get(trace_id)
                    .expect(format!("Trace ID {:?} not found in traces", trace_id).as_str())
            })
            .collect::<Vec<_>>();
        let current_total_probability: f64 = trace_infos.iter()
            .map(|trace_info| {
                let temp_posterior = trace_info.temp_posterior.borrow();
                *temp_posterior.as_ref()
                    .expect("Temporary posterior must be set before updating the final posterior")
            })
            .sum();
        assert!(current_total_probability > 0.0, "Total probability must be greater than 0");
        let mut target_total_probability = 0.0;
        for iteration_num in (1..grid.next_iteration.get()).map(|i| NonZeroUsize::new(i).unwrap()) {
            let prior_probability = crate::hyperparameters::ITERATION_TO_PRIOR_PROBABILITY
                .get(&iteration_num)
                .ok_or_else(|| format!("Iteration {:?} not found in ITERATION_TO_PRIOR_PROBABILITY", iteration_num))?;
            target_total_probability += *prior_probability;
        }
        assert!(target_total_probability < 1.0, "Total prior probability must be less than 1.0, but got {}", target_total_probability);
        let normalization_factor = target_total_probability/ current_total_probability;
        for trace_info in trace_infos {
            let mut temp_posterior = trace_info.temp_posterior.borrow_mut();
            {
                let temp_posterior = *temp_posterior.as_ref()
                .expect("Temporary posterior must be set before updating the final posterior");
                let mut posterior_normalized = trace_info.posterior_normalized.borrow_mut();
                *posterior_normalized = Some(temp_posterior * normalization_factor);
            }
            // reset the temporary posterior
            *temp_posterior = None;
        }
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
    let pad_pair_to_traces: HashMap<PadPairID, HashMap<IterationNum, HashSet<TraceID>>> = pad_pairs
        .iter()
        .map(|(pad_pair_id, _)| {
             (*pad_pair_id, HashMap::new())
        }).collect();
    let grid = ProbaGrid {
        width,
        height,
        nets,
        net_to_pads,
        net_to_pad_pairs,
        pad_pairs,
        visited_traces: BTreeSet::new(),
        traces: HashMap::new(),
        pad_pair_to_traces,
        trace_collision_adjacency: HashMap::new(),
        next_iteration: NonZeroUsize::new(1).unwrap(), // Start with the first iteration
        trace_id_generator: Box::new((0..).map(TraceID)),
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
        nets: _,
        net_to_pads: _,
        net_to_pad_pairs,
        pad_pairs,
        visited_traces,
        traces,
        pad_pair_to_traces,
        trace_collision_adjacency,
        next_iteration,
        trace_id_generator,
    } = grid;
    // sample new traces for each net
    let mut new_traces: Vec<TraceInfo> = Vec::new();
    for (net_id, pad_pair_ids) in net_to_pad_pairs.iter() {
        println!("Sampling new traces for net ID: {:?}", net_id);
        let mut num_generated_traces: HashMap<PadPairID, usize> = pad_pair_ids
            .iter()
            .map(|pad_pair_id| (*pad_pair_id, 0))
            .collect();
        let mut generation_attempts: usize = 0;
        // while there is a pad pair that has less than MAX_TRACES traces
        while num_generated_traces.values().any(|&count| count < MAX_TRACES_PER_ITERATION)
        && generation_attempts < MAX_GENERATION_ATTEMPTS
        {
            println!("Generation attempt: {}", generation_attempts + 1);
            generation_attempts += 1;
            // randomly generate a trace for each pad pair of other nets (in a rare case the trace will not be generated)
            let obstacle_traces: HashMap<PadPairID, Option<TraceID>> = net_to_pad_pairs.iter()
                .filter(|(other_net_id, _)| *other_net_id != net_id)
                .flat_map(|(_, pad_pair_ids)| {
                    pad_pair_ids.iter().map::<Result<(PadPairID, Option<TraceID>),String>, _>(|pad_pair_id| {
                        let candidate_trace_ids = pad_pair_to_traces.get(pad_pair_id).unwrap();
                        // use flat map to merge a hashmap into a vector, removing the keys
                        let candidate_trace_ids = candidate_trace_ids.iter()
                            .flat_map(|(_, trace_ids)|trace_ids.iter())
                            .cloned()
                            .collect::<Vec<_>>();

                        let mut sum_probability = 0.0;
                        let mut probabilities: Vec<f64> = Vec::new();
                        // get the sum probability and all the probabilities of the candidate traces
                        for candidate_trace_id in candidate_trace_ids.iter(){
                            let candidate_trace = traces.get(candidate_trace_id).unwrap();
                            // we need a normalized fallback probability
                            let posterior_normalized = candidate_trace.posterior_normalized.borrow();
                            let posterior_normalized = posterior_normalized.as_ref()
                                .ok_or_else(|| format!("Posterior normalized for trace ID {:?} is None. Call update posterior before calling sample new traces", candidate_trace_id))
                                .map_err(|e| e)?;
                            sum_probability += *posterior_normalized;
                            probabilities.push(*posterior_normalized);
                        }
                        let mut assumed_sum_probability = 0.0;
                        for iteration in (1..next_iteration.get()).map(|i| NonZeroUsize::new(i).unwrap()) {
                            let prior_probability = crate::hyperparameters::ITERATION_TO_PRIOR_PROBABILITY
                                .get(&iteration)
                                .ok_or_else(|| format!("Iteration {:?} not found in ITERATION_TO_PRIOR_PROBABILITY", iteration))?;
                            assumed_sum_probability += *prior_probability;
                        }
                        assert!(f64::abs(sum_probability - assumed_sum_probability) < 1e-6, 
                            "Sum of probabilities {} does not match assumed sum probability {}", 
                            sum_probability, assumed_sum_probability);
                        probabilities.push(1.0 - sum_probability); // add the probability of not choosing any trace
                        let dist = WeightedIndex::new(probabilities)
                            .map_err(|e| format!("Failed to create WeightedIndex: {}", e))?;
                        let mut rng = rand::rng();
                        let index = dist.sample(&mut rng);
                        let chosen_trace_id = if index < candidate_trace_ids.len() {
                            Some(candidate_trace_ids[index])
                        } else {
                            None // No trace chosen
                        };
                        Ok((*pad_pair_id, chosen_trace_id))
                    })
                })
                .collect::<Result<HashMap<_, _>, _>>()
                .map_err(|e|e)?;
            // create a Dijkstra model that contains all the obstacles from other nets
            // this can be reused for all pad pairs in this net
            let mut obstacles: HashSet<Point> = HashSet::new();
            let mut diagonal_obstacles: HashSet<Point> = HashSet::new();
            for (_, trace_id) in obstacle_traces.iter() {
                if let Some(trace_id) = trace_id {
                    let trace_info = traces.get(trace_id).ok_or_else(|| format!("Trace ID {:?} not found in traces", trace_id))?;
                    obstacles.extend(trace_info.trace_path.covered.iter().cloned());
                    diagonal_obstacles.extend(trace_info.trace_path.diagonal_covered.iter().cloned());
                }
            }
            let dijkstra_model = DijkstraModel {
                width: *width,
                height: *height,
                obstacles,
                diagonal_obstacles,
                start: Point { x: 0, y: 0 }, // Placeholder, will be set for each pad pair
                end: Point { x: 0, y: 0 }, // Placeholder, will be set for each pad pair
            };
            for pad_pair_id in pad_pair_ids.iter() {
                if num_generated_traces.get(pad_pair_id).unwrap() >= &MAX_TRACES_PER_ITERATION {
                    continue; // Skip if the maximum number of traces for this pad pair is reached
                }
                let mut dijkstra_model_copy = dijkstra_model.clone();
                let pad_pair = pad_pairs.get(pad_pair_id).ok_or_else(|| format!("PadPairID {:?} not found in net_to_pad_pairs", pad_pair_id))?;
                dijkstra_model_copy.start = pad_pair.start;
                dijkstra_model_copy.end = pad_pair.end;
                let result = dijkstra_model_copy.run();
                let result = match result {
                    Ok(res) => res,
                    Err(e) => {
                        println!("Dijkstra's algorithm failed for pad pair ID {:?}: {}", pad_pair_id, e);
                        continue; // Skip this pad pair if Dijkstra's algorithm fails
                    }
                };
                let trace_path = result.trace_path;
                if visited_traces.contains(&trace_path) {
                    println!("Trace path already visited, skipping");
                    continue; // Skip if the trace path has already been visited
                }else{
                    visited_traces.insert(trace_path.clone());
                    // cannot add the new traces directly to the current container
                    let trace_id = trace_id_generator.next().unwrap();
                    let trace_info = TraceInfo {
                        net_id: *net_id,
                        pad_pair_id: *pad_pair_id,
                        trace_id,
                        start: pad_pair.start,
                        end: pad_pair.end,
                        trace_path,
                        trace_directions: result.trace_directions,
                        trace_length: result.distance,
                        iteration: *next_iteration,
                        prior_probability_cache: RefCell::new(None), // No prior probability cache in the first iteration
                        posterior_normalized: RefCell::new(None), // No posterior normalized in the first iteration
                        score_cache: RefCell::new(None), // No score cache in the first iteration
                        temp_posterior: RefCell::new(None), // No temporary posterior unnormalized in the first iteration
                    };

                    // traces.insert(trace_id, trace_info);
                    // pad_pair_to_traces.get_mut(pad_pair_id).unwrap()
                    //     .entry(IterationNum(*next_iteration))
                    //     .or_default()
                    //     .insert(trace_id);
                    new_traces.push(trace_info);


                    let num = num_generated_traces.get_mut(pad_pair_id)
                        .unwrap();
                    *num += 1; // Increment the number of generated traces for this pad pair
                }
            }
        }       
    }
    for trace_info in new_traces.iter() {
        let pad_pair_id = trace_info.pad_pair_id;
        let trace_id = trace_info.trace_id;
        // Insert the trace into the pad_pair_to_traces map
        let old = traces.insert(trace_id, trace_info.clone());
        assert!(old.is_none(), "Trace ID {:?} already exists in traces", trace_id);
        pad_pair_to_traces.get_mut(&pad_pair_id).unwrap()
            .entry(IterationNum(*next_iteration))
            .or_default()
            .insert(trace_id);
    }
    // update trace collision adjacency
    *trace_collision_adjacency = traces.iter()
        .map(|(trace_id, _)| {
            (trace_id.clone(), HashSet::new())
        }).collect();
    let net_to_traces: HashMap<NetID, HashMap<TraceID, &TraceInfo>> = net_to_pad_pairs
        .iter()
        .map(|(net_id, pad_pair_ids)| {
            let trace_id_to_info: HashMap<TraceID, &TraceInfo> = pad_pair_ids
                .iter()
                .flat_map(|pad_pair_id| {
                    pad_pair_to_traces.get(pad_pair_id).unwrap()
                        .iter()
                        .flat_map(|(_, trace_ids)| trace_ids.iter())
                        .cloned()                        
                })
                .map(|trace_id| {
                    let trace_info = traces.get(&trace_id)
                        .expect("Trace ID not found in traces");
                    (trace_id, trace_info)
                })
                .collect();
            (*net_id, trace_id_to_info)
        })
        .collect();

    let traces_vec = net_to_traces
        .into_values()
        .collect::<Vec<_>>();
    for i in 0..traces_vec.len() {
        for j in (i + 1)..traces_vec.len() {
            let trace_set_a = &traces_vec[i];
            let trace_set_b = &traces_vec[j];
            for (trace_id_a, trace_a) in trace_set_a {
                for (trace_id_b, trace_b) in trace_set_b {
                    let collide = trace_a.trace_path.collides_with(&trace_b.trace_path);
                    if collide{
                        trace_collision_adjacency.get_mut(trace_id_a).unwrap()
                            .insert(trace_id_b.clone());
                        trace_collision_adjacency
                            .get_mut(trace_id_b).unwrap()
                            .insert(trace_id_a.clone());
                    }
                }
            }
        }
    }
    // Increment the iteration number for the next round of sampling
    *next_iteration = NonZeroUsize::new(next_iteration.get() + 1).unwrap();
    Ok(())
}