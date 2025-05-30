use std::{cmp::Reverse, collections::{BinaryHeap, HashSet}};

use image::{ImageBuffer, Pixel, Rgba};
use ordered_float::OrderedFloat;

use crate::datatypes::{DijkstraModel, DijkstraResult, Grid, Net, Pad, Point};



pub fn grid_to_string(grid: &Grid) -> Vec<Vec<char>>{
    let width = grid.width;
    let height = grid.height;
    let mut grid_string: Vec<Vec<char>> = vec![vec![' '; width as usize]; height as usize];
    for (net, points) in &grid.pads {
        let net_char = net.pad_c;
        for point in points {
            assert!(point.x < width && point.y < height, "Point out of bounds");
            grid_string[point.y as usize][point.x as usize] = net_char;
        }
    }
    for (net, points) in &grid.traces {
        let route_char = net.route_c;
        for point in points {
            assert!(point.x < width && point.y < height, "Point out of bounds");
            grid_string[point.y as usize][point.x as usize] = route_char;
        }
    }
    grid_string
}

pub fn print_grid_string(grid_string: &Vec<Vec<char>>) {
    let width = grid_string[0].len();
    let horizontal_wall = "#".repeat(width + 2);
    println!("{}", horizontal_wall);
    for row in grid_string {
        println!("#{}#", row.iter().collect::<String>());
    }
    println!("{}", horizontal_wall);
}

pub fn print_grid(grid: &Grid) {
    let grid_string = grid_to_string(grid);
    print_grid_string(&grid_string);
}

pub fn naive_route(unrouted_grid: Grid) -> Grid { 
    let prepare_dijkstra_model_unrouted = |net: Net, start: Point, end: Point|{
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
    let pad_pairs: Vec<(OrderedFloat<f32>, Net, Point, Point)> = unrouted_grid.pads.iter()
        .flat_map(|(net, points)| {
            // permutate all pairs and calculate their distance using Dijkstra's algorithm
            let mut pairs = vec![];
            let points_vec: Vec<Point> = points.iter().cloned().collect();
            for i in 0..points_vec.len() {
                for j in (i + 1)..points_vec.len() {
                    let point1 = points_vec[i];
                    let point2 = points_vec[j];
                    let dijkstra_model = prepare_dijkstra_model_unrouted(net.clone(), point1, point2);
                    let DijkstraResult{distance, ..} = dijkstra_model.run();
                    pairs.push((OrderedFloat(distance), net.clone(), point1, point2));
                }
            }
            pairs
        })
        .collect();
    let mut priority_queue: BinaryHeap<_> = pad_pairs.into_iter()
        .map(Reverse)
        .collect();
    
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
    while let Some(Reverse((OrderedFloat(_distance), net, start, end))) = priority_queue.pop() {
        // construct dijkstra model for the current pair of pads
        let dijkstra_model = prepare_dijkstra_model(&grid, &net, start, end);
        // run dijkstra's algorithm
        let DijkstraResult { start: _, directions, distance: _ } = dijkstra_model.run();
        // add the route to the grid
        let mut current_point = start;
        grid.traces.entry(net.clone()).or_default().insert(current_point);
        for direction in directions {
            let last_point = current_point;
            current_point = Point {
                x: (current_point.x as i32 + direction.x) as u32,
                y: (current_point.y as i32 + direction.y) as u32,
            };
            assert!(current_point.x < grid.width && current_point.y < grid.height, "Point out of bounds");
            grid.traces.get_mut(&net).unwrap().insert(current_point);
            if direction.x != 0 && direction.y != 0 {
                // if the direction is diagonal, we also add the diagonal trace
                let diagonal_trace_point = Point {
                    x: last_point.x.min(current_point.x),
                    y: last_point.y.min(current_point.y),
                };
                grid.diagonal_traces.entry(net.clone()).or_default().insert(diagonal_trace_point);
            }
        }
    }
    assert!(priority_queue.is_empty());
    grid
}