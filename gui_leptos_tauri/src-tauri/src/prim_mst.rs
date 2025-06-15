use std::{cmp::Reverse, collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap}};

use ordered_float::OrderedFloat;

use crate::grid::{Net, Point, PointPair};




pub fn prim_mst(edges: Vec<(OrderedFloat<f64>, Net, PointPair)>) -> Vec<(OrderedFloat<f64>, Net, PointPair)> {
    assert!(edges.len() >=1, "At least 1 edge is required to form a minimum spanning tree.");
    let (_, net, _) = edges[0].clone();
    let mut index_to_point: BTreeSet<Point> = BTreeSet::new();
    for (_, _net, point_pair) in &edges {
        index_to_point.insert(point_pair.start().clone());
        index_to_point.insert(point_pair.end().clone());
    }
    let index_to_point: Vec<Point> = index_to_point.into_iter().collect();
    let point_to_index: HashMap<Point, i32> = index_to_point
        .iter()
        .enumerate()
        .map(|(i, point)| (*point, i as i32))
        .collect();
    let edges: Vec<(OrderedFloat<f64>, Net, i32, i32)> = edges.into_iter()
        .map(|(weight, net, point_pair)| {
            let start_index = point_to_index[&point_pair.start()];
            let end_index = point_to_index[&point_pair.end()];
            (weight, net, start_index, end_index)
        })
        .collect();
    let undirected_graph: Vec<Vec<(OrderedFloat<f64>, i32)>> = {
        let mut graph = vec![vec![]; point_to_index.len()];
        for (weight, net, start, end) in &edges {
            graph[*start as usize].push((weight.clone(), *end));
            graph[*end as usize].push((weight.clone(), *start));
        }
        graph
    };
    let mut mst_edges: Vec<(OrderedFloat<f64>, Net, i32, i32)> = vec![];
    let mut visited = std::collections::HashSet::new();
    // let edges = edges.into_iter()
    //     .map(|(weight, net, start, end)| (weight, Option<(Net, Point, Point)>::Some((net, start, end))))
    //     .collect::<Vec<_>>();
    let mut priority_queue: BinaryHeap<Reverse<(OrderedFloat<f64>, i32, i32)>> = BinaryHeap::new();
    priority_queue.push(Reverse((OrderedFloat(0.0), 0, -1))); // Start with a dummy edge
    while let Some(Reverse((weight, u, v))) = priority_queue.pop() {
        if visited.contains(&u) {
            continue; // Skip if already visited
        }
        visited.insert(u);
        if v != -1 {
            mst_edges.push((weight, net.clone(), u, v));
        }
        for &(edge_weight, neighbor) in &undirected_graph[u as usize] {
            if !visited.contains(&neighbor) {
                priority_queue.push(Reverse((edge_weight, neighbor, u)));
            }
        }
    }
    // let mst_edges = mst_edges.into_iter()
    //     .map(|(weight, net, start, end)|{
    //         let start_point = index_to_point[start as usize].clone();
    //         let end_point = index_to_point[end as usize].clone();
    //         ((start_point, end_point), (weight, net))
    //     })
    //     .collect::<BTreeMap<(Point, Point), _>>();
    // let mst_edges = mst_edges.into_iter()
    //     .map(|((start, end), (weight, net))| (weight, net, start, end))
    //     .collect::<Vec<_>>();
    let mst_edges: Vec<(OrderedFloat<f64>, Net, PointPair)> = mst_edges.into_iter()
        .map(|(weight, net, start, end)| {
            let start_point = index_to_point[start as usize].clone();
            let end_point = index_to_point[end as usize].clone();
            (weight, net, PointPair::new(start_point, end_point))
        })
        .collect();
    assert!(mst_edges.len() >= 1, "Minimum spanning tree must have at least one edge.");
    mst_edges
}