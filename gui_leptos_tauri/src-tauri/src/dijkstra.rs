use std::{cmp::Ordering, collections::{BTreeSet, BinaryHeap, HashMap, HashSet}};

use crate::{grid::Point, proba_grid::{Direction, TracePath}};

#[derive(Debug, Clone)]
pub struct DijkstraModel{
    pub width: usize,
    pub height: usize,
    pub obstacles: HashSet<Point>,
    pub diagonal_obstacles: HashSet<Point>, // obstacles that are diagonal traces
    pub start: Point,
    pub end: Point,
}

impl DijkstraModel {
    pub fn run(&self) -> Result<DijkstraResult, String> {
        let mut heap = BinaryHeap::new();
        let mut dist: HashMap<Point, f64> = HashMap::new();
        let mut prev: HashMap<Point, Point> = HashMap::new();

        #[derive(Debug, PartialEq)]
        struct State {
            cost: f64,
            position: Point,
        }

        impl Eq for State {}

        impl PartialOrd for State {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                other.cost.partial_cmp(&self.cost)
            }
        }

        impl Ord for State {
            fn cmp(&self, other: &Self) -> Ordering {
                self.partial_cmp(other).unwrap()
            }
        }

        heap.push(State { cost: 0.0, position: self.start });
        dist.insert(self.start, 0.0);

        let cardinal_dirs: [(i32, i32); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];
        let diagonal_dirs: [(i32, i32); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];

        while let Some(State { cost, position }) = heap.pop() {
            if position == self.end {
                break;
            }
            // skip if we already found a shorter path
            if let Some(&d) = dist.get(&position) {
                if cost > d {
                    continue;
                }
            }
            // Cardinal moves
            for &(dx, dy) in &cardinal_dirs {
                if let Some(next) = self.offset_point(position, dx, dy) {
                    if self.obstacles.contains(&next) {
                        continue;
                    }
                    let next_cost = cost + 1.0;
                    if next_cost < *dist.get(&next).unwrap_or(&f64::INFINITY) {
                        dist.insert(next, next_cost);
                        prev.insert(next, position);
                        heap.push(State { cost: next_cost, position: next });
                    }
                }
            }
            // Diagonal moves
            for &(dx, dy) in &diagonal_dirs {
                if let Some(next) = self.offset_point(position, dx, dy) {
                    // top-left corner of the diagonal
                    let top_left = self.offset_point(position, dx.min(0), dy.min(0)).unwrap();
                    if self.obstacles.contains(&next) || self.diagonal_obstacles.contains(&top_left) {
                        continue;
                    }
                    let next_cost = cost + (2.0f64).sqrt();
                    if next_cost < *dist.get(&next).unwrap_or(&f64::INFINITY) {
                        dist.insert(next, next_cost);
                        prev.insert(next, position);
                        heap.push(State { cost: next_cost, position: next });
                    }
                }
            }
        }

        // Reconstruct path
        let mut trace_directions = Vec::new();
        let mut covered = BTreeSet::new();
        let mut diagonal_covered = BTreeSet::new();
        let mut current = self.end;
        covered.insert(current);
        while current != self.start {
            if let Some(&prev_point) = prev.get(&current) {
                let direction = Direction {
                    x: current.x as i32 - prev_point.x as i32,
                    y: current.y as i32 - prev_point.y as i32,
                };
                if direction.x != 0 && direction.y != 0 {
                    // if the direction is diagonal, we also add the diagonal trace
                    let diagonal_trace_point = Point {
                        x: prev_point.x.min(current.x),
                        y: prev_point.y.min(current.y),
                    };
                    diagonal_covered.insert(diagonal_trace_point);
                }
                trace_directions.push(direction);
                current = prev_point;
                covered.insert(current);
            } else {
                // No path found
                return Err("Dijkstra Algorithm Failed: No Path Found".to_string());
            }
        }
        trace_directions.reverse();
        let mut current = self.start;
        let trace_path: BTreeSet<Point> = trace_directions.iter().fold(BTreeSet::new(), |mut acc, dir| {
            current = Point {
                x: (current.x as i32 + dir.x) as usize,
                y: (current.y as i32 + dir.y) as usize,
            };
            acc.insert(current);
            acc
        });
        let trace_path = TracePath { 
            covered, 
            diagonal_covered
        };
        Ok(DijkstraResult {
            start: self.start,
            end: self.end,
            trace_path,
            trace_directions,
            distance: *dist.get(&self.end).unwrap_or(&f64::INFINITY),
        })
    }

    fn offset_point(&self, point: Point, dx: i32, dy: i32) -> Option<Point> {
        let nx = point.x as i32 + dx;
        let ny = point.y as i32 + dy;
        if nx >= 0 && ny >= 0 && (nx as usize) < self.width && (ny as usize) < self.height {
            Some(Point { x: nx as usize, y: ny as usize })
        } else {
            None
        }
    }
}

pub struct DijkstraResult{
    pub start: Point,
    pub end: Point, 
    pub trace_path: TracePath,
    pub trace_directions: Vec<Direction>,
    pub distance: f64,
}