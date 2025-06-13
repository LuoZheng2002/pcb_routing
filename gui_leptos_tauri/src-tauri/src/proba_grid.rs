use std::{cell::RefCell, collections::{BTreeSet, HashMap, HashSet}, num::NonZeroUsize, vec};

use shared::interface_types::{Color, ColorGrid};

use crate::{grid::Point, hyperparameters::ITERATION_TO_PRIOR_PROBABILITY};

#[derive(Debug, Clone)]
pub struct NetInfo{
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




#[derive(Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct PadPair{
    pub net_id: NetID,
    pub pad_pair_id: PadPairID,
    pub start: Point, // Start point of the trace
    pub end: Point, // End point of the trace
}

#[derive(Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct TracePath(pub BTreeSet<Point>); // A set of points that the trace covers


#[derive(Debug, Clone)]
pub struct TraceInfo{
    pub net_id: NetID,
    pub pad_pair_id: PadPairID,
    pub trace_id: TraceID,
    pub start: Point, // Start point of the trace
    pub end: Point, // End point of the trace
    pub trace_path: TracePath, // The path of the trace
    pub iteration: NonZeroUsize, // The iteration that the trace belongs to, starting from 1
    // probability information:
    pub score: f64, // Score of the trace, between 0 and 1
    // pub old_posterior_unnormalized: RefCell<Option<f64>>, // to be accessed in the next iteration
    pub old_posterior_normalized: RefCell<Option<f64>>, // to be accessed in the next iteration
    pub new_posterior_unnormalized: RefCell<Option<f64>>, // serve as a buffer for simultaneous updates
}

impl TraceInfo{
    pub fn get_prior_probability(&self)->f64{
        ITERATION_TO_PRIOR_PROBABILITY.get(&self.iteration)
            .cloned()
            .unwrap_or_else(|| panic!("No prior probability for iteration {:?}", self.iteration))
    }
}


#[derive(Clone)]
pub struct ProbaGridProblem{
    pub width: usize,
    pub height: usize,
    pub nets: HashMap<NetID, NetInfo>,
    pub net_to_pads: HashMap<NetID, HashSet<Point>>, // NetID to list of pad coordinates
}

impl ProbaGridProblem{
    pub fn to_color_grid(&self)->ColorGrid{
        let mut grid = vec![vec![Color{r: 255, g: 255, b: 255}; self.width]; self.height];
        for (net_id, net_info) in &self.nets {
            if let Some(pad_color) = &net_info.pad_color {
                if let Some(pads) = self.net_to_pads.get(net_id) {
                    for pad in pads {
                        if pad.x < self.width && pad.y < self.height {
                            grid[pad.y][pad.x] = pad_color.clone();
                        }
                    }
                }
            }else{
                panic!("NetInfo for NetID {:?} does not have a pad_color", net_id);
            }
        }
        ColorGrid { grid}
    }
    pub fn remove_pad(&mut self, point: Point) {
        let prev_pads = std::mem::take(&mut self.net_to_pads);
        self.net_to_pads = prev_pads.into_iter()
            .map(|(net_id, pads)| {
                let mut new_pads = pads.clone();
                new_pads.remove(&point);
                (net_id, new_pads)
            })
            .filter(|(_, pads)| !pads.is_empty())
            .collect();
    }
    pub fn insert_pad(&mut self, net_id: NetID, point: Point, pad_color: Color, route_color: Color) {
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



pub struct ProbaGrid{
    pub width: usize,
    pub height: usize,
    pub nets: HashMap<NetID, NetInfo>,
    pub net_to_pads: HashMap<NetID, HashSet<Point>>, // NetID to list of pad coordinates
    // output
    pub net_to_pad_pairs: HashMap<NetID, HashSet<PadPairID>>, // NetID to PadPairToRouteID to PadPairToRoute
    pub pad_pairs: HashMap<PadPairID, PadPair>, // PadPairToRouteID to PadPairToRoute
    pub visited_traces: BTreeSet<TracePath>,
    pub pad_pair_to_traces: HashMap<PadPairID, HashSet<TraceID>>, // PadPairID to TraceID
    pub traces: HashMap<TraceID, TraceInfo>, // TraceID to Trace
    pub trace_collision_adjacency: HashMap<TraceID, HashSet<TraceID>>,
    
    pub next_iteration: NonZeroUsize, // The next iteration to be processed, starting from 1
}

impl ProbaGrid{
    pub fn to_color_grid(&self)->ColorGrid{
        let mut grid = vec![vec![Color{r: 255, g: 255, b: 255}; self.width]; self.height];
        for (net_id, net_info) in &self.nets {
            if let Some(pad_color) = &net_info.pad_color {
                if let Some(pads) = self.net_to_pads.get(net_id) {
                    for pad in pads {
                        if pad.x < self.width && pad.y < self.height {
                            grid[pad.y][pad.x] = pad_color.clone();
                        }
                    }
                }
            }else{
                panic!("NetInfo for NetID {:?} does not have a pad_color", net_id);
            }
        }
        // iterate through the traces and set the route color
        for (net_id, pad_pairs) in &self.net_to_pad_pairs {
            let net_info = self.nets.get(net_id).unwrap();
            let route_color = net_info.route_color.clone().unwrap();                
            for pad_pair_id in pad_pairs {
                let trace_ids = self.pad_pair_to_traces.get(pad_pair_id).unwrap();
                for trace_id in trace_ids {
                    let trace = self.traces.get(trace_id).unwrap();
                    for point in &trace.trace_path.0 {
                        let original_color = grid[point.y][point.x].clone();
                        let opacity: f64 = if let Some(old_posterior) = trace.old_posterior_normalized.borrow().as_ref() {
                            *old_posterior
                        } else {
                            trace.get_prior_probability()
                        };
                        let new_color = Color {
                            r: (route_color.r as f64 * opacity + original_color.r as f64 * (1.0-opacity)) as u8,
                            g: (route_color.g as f64 * opacity + original_color.g as f64 * (1.0-opacity)) as u8,
                            b: (route_color.b as f64 * opacity + original_color.b as f64 * (1.0-opacity)) as u8,
                        };
                        grid[point.y][point.x] = new_color;
                    }
                }
            }
        }
        ColorGrid { grid}
    }
}

pub enum ProbaGridState{
    Uninitialized{
        input: ProbaGridProblem,
    },
    Initialized{
        output: ProbaGrid,
    }
}

impl ProbaGridState{
    pub fn to_color_grid(&self)->ColorGrid{
        match self {
            ProbaGridState::Uninitialized { input } => input.to_color_grid(),
            ProbaGridState::Initialized { output } => output.to_color_grid(),
        }
    }
}