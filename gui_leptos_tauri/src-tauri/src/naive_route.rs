use std::cmp::Reverse;
use std::collections::BTreeSet;
use std::collections::BinaryHeap;
use std::collections::HashSet;

use crate::dijkstra::*;
use crate::grid::*;
use crate::prim_mst::prim_mst;
use crate::proba_grid::TracePath;
use ordered_float::OrderedFloat;

pub fn naive_route(mut unrouted_grid: Grid) -> Result<Grid, String> {
    unrouted_grid.traces.clear();
    unrouted_grid.diagonal_traces.clear();
    let prepare_dijkstra_model_unrouted = |net: Net, start: Point, end: Point| {
        let other_pads = unrouted_grid.pads_except(&net);
        DijkstraModel {
            width: unrouted_grid.width,
            height: unrouted_grid.height,
            obstacles: other_pads,
            diagonal_obstacles: HashSet::new(), // no diagonal obstacles in the unrouted grid
            start,
            end,
        }
    };
    // prepare all the pairs of pads to route
    let pad_pairs: Vec<(OrderedFloat<f64>, Net, PointPair)> = unrouted_grid
        .pads
        .iter()
        .flat_map(|(net, points)| {
            // permutate all pairs and calculate their distance using Dijkstra's algorithm
            let mut pairs = vec![];
            let points_vec: Vec<Point> = points.iter().cloned().collect();
            for i in 0..points_vec.len() {
                for j in (i + 1)..points_vec.len() {
                    let point1 = points_vec[i];
                    let point2 = points_vec[j];
                    let dijkstra_model =
                        prepare_dijkstra_model_unrouted(net.clone(), point1, point2);
                    let DijkstraResult { distance, .. } =
                        dijkstra_model.run().unwrap_or(DijkstraResult {
                            start: point1,
                            end: point2,
                            trace_directions: vec![],
                            distance: f64::INFINITY,
                            trace_path: TracePath {
                                covered: BTreeSet::new(),
                                diagonal_covered: BTreeSet::new(),
                            },
                        });
                    pairs.push((
                        OrderedFloat(distance),
                        net.clone(),
                        PointPair::new(point1, point2),
                    ));
                }
            }

            // prim's algorithm
            let pairs = prim_mst(pairs);
            pairs
        })
        .collect();
    let mut priority_queue: BinaryHeap<_> = pad_pairs.into_iter().map(Reverse).collect();

    fn prepare_dijkstra_model(grid: &Grid, net: &Net, start: Point, end: Point) -> DijkstraModel {
        let other_pads = grid.pads_except(net);
        let other_routes = grid.routes_except(net);
        let other_diagonal_routes = grid.diagonal_routes_except(net);
        let mut obstacles = other_pads;
        obstacles.extend(other_routes);
        DijkstraModel {
            width: grid.width,
            height: grid.height,
            obstacles,
            diagonal_obstacles: other_diagonal_routes,
            start,
            end,
        }
    }
    let mut grid = unrouted_grid.clone();
    while let Some(Reverse((OrderedFloat(_distance), net, point_pair))) = priority_queue.pop() {
        println!(
            "Routing net: {:?}, from {:?} to {:?}",
            net,
            point_pair.start(),
            point_pair.end()
        );
        // construct dijkstra model for the current pair of pads
        let dijkstra_model =
            prepare_dijkstra_model(&grid, &net, point_pair.start(), point_pair.end());
        // run dijkstra's algorithm
        let DijkstraResult {
            trace_directions, ..
        } = dijkstra_model.run()?;
        // add the route to the grid
        let mut current_point = point_pair.start();
        grid.traces
            .entry(net.clone())
            .or_default()
            .insert(current_point);
        for direction in trace_directions {
            let last_point = current_point;
            current_point = Point {
                x: (current_point.x as i32 + direction.x) as usize,
                y: (current_point.y as i32 + direction.y) as usize,
            };
            assert!(
                current_point.x < grid.width && current_point.y < grid.height,
                "Point out of bounds"
            );
            grid.traces.get_mut(&net).unwrap().insert(current_point);
            if direction.x != 0 && direction.y != 0 {
                // if the direction is diagonal, we also add the diagonal trace
                let diagonal_trace_point = Point {
                    x: last_point.x.min(current_point.x),
                    y: last_point.y.min(current_point.y),
                };
                grid.diagonal_traces
                    .entry(net.clone())
                    .or_default()
                    .insert(diagonal_trace_point);
            }
        }
    }
    assert!(priority_queue.is_empty());
    Ok(grid)
}
