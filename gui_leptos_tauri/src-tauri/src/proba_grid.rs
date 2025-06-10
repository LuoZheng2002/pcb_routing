use std::{cell::RefCell, collections::{BTreeSet, HashMap, HashSet}, vec};

use shared::interface_types::{Color, ColorGrid};

use crate::grid::Point;

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
pub struct PadPairToRouteID(pub usize);
#[derive(Copy, Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct TraceID(pub usize);

#[derive(Clone)]
pub struct ProbaGridInput{
    pub width: usize,
    pub height: usize,
    pub nets: HashMap<NetID, NetInfo>,
    pub pads: HashMap<NetID, HashSet<Point>>, // NetID to list of pad coordinates
}

impl ProbaGridInput{
    pub fn to_color_grid(&self)->ColorGrid{
        let mut grid = vec![vec![Color{r: 255, g: 255, b: 255}; self.width]; self.height];
        for (net_id, net_info) in &self.nets {
            if let Some(pad_color) = &net_info.pad_color {
                if let Some(pads) = self.pads.get(net_id) {
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
        let prev_pads = std::mem::take(&mut self.pads);
        self.pads = prev_pads.into_iter()
            .map(|(net_id, pads)| {
                let mut new_pads = pads.clone();
                new_pads.remove(&point);
                (net_id, new_pads)
            })
            .filter(|(_, pads)| !pads.is_empty())
            .collect();
    }
    pub fn insert_pad(&mut self, net_id: NetID, point: Point, pad_color: Color, route_color: Color) {
        self.pads.entry(net_id).or_default().insert(point);
        self.nets.entry(net_id).or_insert(NetInfo {
            net_id: net_id.0,
            pad_character: None, // Assuming no character for pads in this context
            route_character: None, // Assuming no character for routes in this context
            pad_color: Some(pad_color),
            route_color: Some(route_color),
        });
    }
}



#[derive(Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct PadPairToRoute{
    pub net_id: NetID,
    pub pad_pair_id: PadPairToRouteID,
    pub start: Point, // Start point of the trace
    pub end: Point, // End point of the trace
}

#[derive(Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct Trace{
    pub net_id: NetID,
    pub pad_pair_id: PadPairToRouteID,
    pub trace_id: TraceID,
    pub start: Point, // Start point of the trace
    pub end: Point, // End point of the trace
    pub covered: Vec<Point>, // Points in the trace
}

pub struct TraceProbaInfo{
    pub net_id: NetID,
    pub pad_pair_id: PadPairToRouteID,
    pub trace_id: TraceID,
    pub prior_probability: f64, // Prior probability of the trace
    pub score: f64, // Score of the trace
    pub prior_anchor: f64, // equals to prior_probability * score
    pub old_posterior: RefCell<Option<f64>>,
    pub new_posterior: RefCell<Option<f64>>, // serve as a buffer for simultaneous updates
}

// pad pairs 


pub struct ProbaGridOutput{
    pub width: usize,
    pub height: usize,
    pub nets: HashMap<NetID, NetInfo>,
    pub pads: HashMap<NetID, HashSet<Point>>, // NetID to list of pad coordinates
    pub pad_pairs_to_route: HashMap<NetID, HashMap<PadPairToRouteID, PadPairToRoute>>, // NetID to PadPairToRouteID to PadPairToRoute
    pub visited_traces: HashMap<PadPairToRouteID, HashSet<Trace>>,
    pub traces: HashMap<PadPairToRouteID, HashMap<TraceID, Trace>>,
    pub trace_proba_infos: HashMap<TraceID, TraceProbaInfo>,
    pub trace_collision_adjacency: HashMap<TraceID, HashSet<TraceID>>, 
}

impl ProbaGridOutput{
    pub fn to_color_grid(&self)->ColorGrid{
        let mut grid = vec![vec![Color{r: 255, g: 255, b: 255}; self.width]; self.height];
        for (net_id, net_info) in &self.nets {
            if let Some(pad_color) = &net_info.pad_color {
                if let Some(pads) = self.pads.get(net_id) {
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
        for (net_id, pad_pairs) in &self.pad_pairs_to_route {
            let net_info = self.nets.get(net_id).unwrap();
            let route_color = net_info.route_color.clone().unwrap();                
            for (_, pad_pair) in pad_pairs {
                let traces_map = self.traces.get(&pad_pair.pad_pair_id).unwrap();
                for trace in traces_map.values() {
                    for point in &trace.covered {
                        let original_color = grid[point.y][point.x].clone();
                        let trace_info = self.trace_proba_infos.get(&trace.trace_id).unwrap();
                        let opacity: f64 = if let Some(old_posterior) = trace_info.old_posterior.borrow().as_ref() {
                            *old_posterior
                        } else {
                            trace_info.prior_anchor
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
        input: ProbaGridInput,
    },
    Initialized{
        output: ProbaGridOutput,
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