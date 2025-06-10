use std::{cell::RefCell, collections::{BTreeSet, HashMap, HashSet}};

use shared::interface_types::Color;

use crate::grid::Point;


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

pub struct ProbaGridInput{
    pub width: usize,
    pub height: usize,
    pub nets: HashMap<NetID, NetInfo>,
    pub pads: HashMap<NetID, HashSet<Point>>, // NetID to list of pad coordinates
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