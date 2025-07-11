use core::num;
use std::{
    cell::RefCell,
    collections::{BTreeSet, HashMap, HashSet},
    num::NonZeroUsize,
    vec,
};

use shared::interface_types::{Color, ColorGrid};

use crate::{
    grid::Point,
    hyperparameters::{
        HALF_PROBABILITY_RAW_SCORE, ITERATION_TO_PRIOR_PROBABILITY, LENGTH_PENALTY_RATE,
        TURN_PENALTY_RATE,
    },
};

#[derive(Debug, Clone)]
pub struct NetInfo {
    pub net_id: usize,
    pub pad_character: Option<char>,
    pub route_character: Option<char>,
    pub pad_color: Option<Color>,
    pub route_color: Option<Color>,
}

#[derive(Copy, Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct NetID(pub usize);
#[derive(Copy, Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct PadPairID(pub usize);
#[derive(Copy, Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct TraceID(pub usize);

#[derive(Debug, Clone)]
pub struct PadPair {
    pub net_id: NetID,
    pub pad_pair_id: PadPairID,
    pub start: Point, // Start point of the trace
    pub end: Point,   // End point of the trace
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct Direction {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct TracePath {
    pub covered: BTreeSet<Point>,          // The points covered by the trace
    pub diagonal_covered: BTreeSet<Point>, // points in the diagonal
}

impl TracePath {
    pub fn collides_with(&self, other: &TracePath) -> bool {
        // Check if the covered points intersect
        !self.covered.is_disjoint(&other.covered)
            || !self.diagonal_covered.is_disjoint(&other.diagonal_covered)
    }
}

pub struct PostProcessInput {
    pub width: usize,
    pub height: usize,
    pub nets: HashMap<NetID, NetInfo>,
    pub net_to_pads: HashMap<NetID, HashSet<Point>>, // NetID to list of pad coordinates
    // output
    pub net_to_pad_pairs: HashMap<NetID, HashSet<PadPairID>>, // NetID to PadPairToRouteID to PadPairToRoute
    pub pad_pairs: HashMap<PadPairID, PadPair>,               // PadPairToRouteID to PadPairToRoute
    pub pad_pair_to_traces: HashMap<PadPairID, HashSet<TraceID>>, // PadPairID to TraceID
    pub traces: HashMap<TraceID, TraceInfo>,                  // TraceID to Trace
}

#[derive(Debug, Clone)]
pub struct TraceInfo {
    pub net_id: NetID,
    pub pad_pair_id: PadPairID,
    pub trace_id: TraceID,
    pub start: Point,                     // Start point of the trace
    pub end: Point,                       // End point of the trace
    pub trace_path: TracePath,            // The path of the trace
    pub trace_directions: Vec<Direction>, // The directions of the trace
    pub trace_length: f64,                // The length of the trace
    pub iteration: NonZeroUsize, // The iteration that the trace belongs to, starting from 1
    // probability information:
    pub prior_probability_cache: RefCell<Option<f64>>, // Prior probability of the trace, between 0 and 1
    pub score_cache: RefCell<Option<f64>>,             // Score of the trace, between 0 and 1
    // pub old_posterior_unnormalized: RefCell<Option<f64>>, // to be accessed in the next iteration
    pub posterior_normalized: RefCell<Option<f64>>, // to be accessed in the next iteration
    pub temp_posterior: RefCell<Option<f64>>,       // serve as a buffer for simultaneous updates
}

impl TraceInfo {
    fn calculate_score(&self) -> f64 {
        // calculate turns
        let mut turns = 0;
        let mut last_direction = self
            .trace_directions
            .first()
            .cloned()
            .unwrap_or(Direction { x: 0, y: 0 });
        if self.trace_directions.len() > 0 {
            for direction in self.trace_directions.iter().skip(1) {
                if direction.x != last_direction.x || direction.y != last_direction.y {
                    turns += 1;
                }
                last_direction = direction.clone();
            }
        }
        let score_raw = self.trace_length * LENGTH_PENALTY_RATE + turns as f64 * TURN_PENALTY_RATE;
        let k = f64::ln(2.0) / HALF_PROBABILITY_RAW_SCORE;
        let score = f64::exp(-k * score_raw);
        assert!(
            score >= 0.0 && score <= 1.0,
            "Score must be between 0 and 1, got: {}",
            score
        );
        score
    }
    pub fn get_score(&self) -> f64 {
        // let mut score_cache = self.score_cache.borrow_mut();
        // *score_cache.get_or_insert_with(||{
        //     self.calculate_score()
        // })
        // we do not use cache until there is performance issue
        self.calculate_score()
    }
    fn calculate_normalized_prior_probability(
        &self,
        num_traces_in_the_same_iteration: usize,
    ) -> f64 {
        let sum_probability = ITERATION_TO_PRIOR_PROBABILITY
            .get(&self.iteration)
            .cloned()
            .unwrap_or_else(|| panic!("No prior probability for iteration {:?}", self.iteration));
        sum_probability / (num_traces_in_the_same_iteration as f64)
    }
    /// this prior probability is not normalized
    pub fn get_normalized_prior_probability(&self, num_traces_in_the_same_iteration: usize) -> f64 {
        // let mut prior_probability_cache = self.prior_probability_cache.borrow_mut();
        // *prior_probability_cache.get_or_insert_with(||{
        //     self.calculate_prior_probability()
        // })
        // we do not use cache until there is performance issue
        self.calculate_normalized_prior_probability(num_traces_in_the_same_iteration)
    }

    // pub fn calculate_fallback_posterior_unnormalized(&self, num_traces_in_the_same_iteration: usize)->f64{
    //     // calculate the posterior probability of the trace
    //     // this is used when there is no old posterior probability available
    //     let prior_probability = self.get_prior_probability();
    //     let normalized_prior_probability = prior_probability / (num_traces_in_the_same_iteration as f64);

    //     let score = self.get_score();
    //     let posterior_unnormalized = prior_probability * score;
    //     // we use the number of traces in the same iteration to normalize the posterior probability
    //     let posterior_normalized = posterior_unnormalized / (num_traces_in_the_same_iteration as f64);
    //     assert!(posterior_normalized >= 0.0 && posterior_normalized <= 1.0, "Posterior normalized must be between 0 and 1, got: {}", posterior_normalized);
    //     posterior_normalized
    // }
    pub fn get_posterior_normalized_with_fallback(
        &self,
        num_traces_in_the_same_iteration: usize,
    ) -> f64 {
        let posterior_normalized = self.posterior_normalized.borrow();
        if let Some(old_posterior) = posterior_normalized.as_ref() {
            *old_posterior
        } else {
            self.get_normalized_prior_probability(num_traces_in_the_same_iteration)
        }
    }
    // call stack: want to sample traces that block the way -> call get_posterior_normalized for other traces
    // -> use num_traces_in_the_same_iteration to get the normalized prior probability
    // -> use the normalized prior probability as the normalized posterior probability

    // want to sample traces -> should already have the posterior_normalized
}

#[derive(Clone)]
pub struct ProbaGridProblem {
    pub width: usize,
    pub height: usize,
    pub nets: HashMap<NetID, NetInfo>,
    pub net_to_pads: HashMap<NetID, HashSet<Point>>, // NetID to list of pad coordinates
}

impl ProbaGridProblem {
    pub fn to_color_grid(&self) -> ColorGrid {
        let mut grid = vec![
            vec![
                Color {
                    r: 255,
                    g: 255,
                    b: 255
                };
                self.width
            ];
            self.height
        ];
        for (net_id, net_info) in &self.nets {
            if let Some(pad_color) = &net_info.pad_color {
                if let Some(pads) = self.net_to_pads.get(net_id) {
                    for pad in pads {
                        if pad.x < self.width && pad.y < self.height {
                            grid[pad.y][pad.x] = pad_color.clone();
                        }
                    }
                }
            } else {
                panic!("NetInfo for NetID {:?} does not have a pad_color", net_id);
            }
        }
        ColorGrid { grid }
    }
    pub fn remove_pad(&mut self, point: Point) {
        let prev_pads = std::mem::take(&mut self.net_to_pads);
        self.net_to_pads = prev_pads
            .into_iter()
            .map(|(net_id, pads)| {
                let mut new_pads = pads.clone();
                new_pads.remove(&point);
                (net_id, new_pads)
            })
            .filter(|(_, pads)| !pads.is_empty())
            .collect();
    }
    pub fn insert_pad(
        &mut self,
        net_id: NetID,
        point: Point,
        pad_color: Color,
        route_color: Color,
    ) {
        self.net_to_pads.entry(net_id).or_default().insert(point);
        self.nets.entry(net_id).or_insert(NetInfo {
            net_id: net_id.0,
            pad_character: None, // Assuming no character for pads in this context
            route_character: None, // Assuming no character for routes in this context
            pad_color: Some(pad_color),
            route_color: Some(route_color),
        });
    }
}

#[derive(Copy, Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct IterationNum(pub NonZeroUsize);

pub struct ProbaGrid {
    pub width: usize,
    pub height: usize,
    pub nets: HashMap<NetID, NetInfo>,
    pub net_to_pads: HashMap<NetID, HashSet<Point>>, // NetID to list of pad coordinates
    // output
    pub net_to_pad_pairs: HashMap<NetID, HashSet<PadPairID>>, // NetID to PadPairToRouteID to PadPairToRoute
    pub pad_pairs: HashMap<PadPairID, PadPair>,               // PadPairToRouteID to PadPairToRoute
    pub visited_traces: BTreeSet<TracePath>,
    pub pad_pair_to_traces: HashMap<PadPairID, HashMap<IterationNum, HashSet<TraceID>>>, // PadPairID to TraceID
    pub traces: HashMap<TraceID, TraceInfo>, // TraceID to Trace
    pub trace_collision_adjacency: HashMap<TraceID, HashSet<TraceID>>,

    pub next_iteration: NonZeroUsize, // The next iteration to be processed, starting from 1
    pub trace_id_generator: Box<dyn Iterator<Item = TraceID> + Send + 'static>, // A generator for TraceID, starting from 0
}

impl ProbaGrid {
    pub fn get_num_traces_in_the_same_iteration(&self, trace_id: TraceID) -> usize {
        let trace_info = self.traces.get(&trace_id).unwrap();
        let pad_pair_id = trace_info.pad_pair_id;
        let iteration_num = trace_info.iteration;
        let num = self
            .pad_pair_to_traces
            .get(&pad_pair_id)
            .unwrap()
            .get(&IterationNum(iteration_num))
            .unwrap()
            .len();
        assert!(
            num > 0,
            "There should be at least one trace in the same iteration"
        );
        num
    }

    pub fn to_color_grid(&self) -> ColorGrid {
        let mut grid = vec![
            vec![
                Color {
                    r: 255,
                    g: 255,
                    b: 255
                };
                self.width
            ];
            self.height
        ];
        for (net_id, net_info) in &self.nets {
            if let Some(pad_color) = &net_info.pad_color {
                if let Some(pads) = self.net_to_pads.get(net_id) {
                    for pad in pads {
                        if pad.x < self.width && pad.y < self.height {
                            grid[pad.y][pad.x] = pad_color.clone();
                        }
                    }
                }
            } else {
                panic!("NetInfo for NetID {:?} does not have a pad_color", net_id);
            }
        }
        // iterate through the traces and set the route color
        for (net_id, pad_pairs) in &self.net_to_pad_pairs {
            let net_info = self.nets.get(net_id).unwrap();
            let route_color = net_info.route_color.clone().unwrap();
            for pad_pair_id in pad_pairs {
                // trace_ids is a set of TraceID that combines the traces in each iteration
                // use flat map
                let trace_ids = self
                    .pad_pair_to_traces
                    .get(pad_pair_id)
                    .unwrap()
                    .iter()
                    .flat_map(|(_, trace_ids)| trace_ids.iter())
                    .cloned()
                    .collect::<Vec<_>>();
                for trace_id in trace_ids {
                    let trace = self.traces.get(&trace_id).unwrap();
                    for point in &trace.trace_path.covered {
                        let original_color = grid[point.y][point.x].clone();
                        let num_traces_in_the_same_iteration =
                            self.get_num_traces_in_the_same_iteration(trace_id);
                        let opacity: f64 = trace.get_posterior_normalized_with_fallback(
                            num_traces_in_the_same_iteration,
                        );
                        let new_color = Color {
                            r: (route_color.r as f64 * opacity
                                + original_color.r as f64 * (1.0 - opacity))
                                as u8,
                            g: (route_color.g as f64 * opacity
                                + original_color.g as f64 * (1.0 - opacity))
                                as u8,
                            b: (route_color.b as f64 * opacity
                                + original_color.b as f64 * (1.0 - opacity))
                                as u8,
                        };
                        grid[point.y][point.x] = new_color;
                    }
                }
            }
        }
        ColorGrid { grid }
    }
}

pub enum ProbaGridState {
    Uninitialized { input: ProbaGridProblem },
    Initialized { output: ProbaGrid },
}

impl ProbaGridState {
    pub fn to_color_grid(&self) -> ColorGrid {
        match self {
            ProbaGridState::Uninitialized { input } => input.to_color_grid(),
            ProbaGridState::Initialized { output } => output.to_color_grid(),
        }
    }
}
