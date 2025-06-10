use std::{cmp::Ordering, collections::{BTreeSet, BinaryHeap, HashMap, HashSet}};

use crate::grid::Point;


#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct Direction {
    pub x: i32,
    pub y: i32,
}

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
        let mut dist: HashMap<Point, f32> = HashMap::new();
        let mut prev: HashMap<Point, Point> = HashMap::new();

        #[derive(Debug, PartialEq)]
        struct State {
            cost: f32,
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
                    if next_cost < *dist.get(&next).unwrap_or(&f32::INFINITY) {
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
                    let next_cost = cost + (2.0f32).sqrt();
                    if next_cost < *dist.get(&next).unwrap_or(&f32::INFINITY) {
                        dist.insert(next, next_cost);
                        prev.insert(next, position);
                        heap.push(State { cost: next_cost, position: next });
                    }
                }
            }
        }

        // Reconstruct path
        let mut path = Vec::new();
        let mut current = self.end;
        while current != self.start {
            if let Some(&prev_point) = prev.get(&current) {
                path.push(Direction {
                    x: current.x as i32 - prev_point.x as i32,
                    y: current.y as i32 - prev_point.y as i32,
                });
                current = prev_point;
            } else {
                // No path found
                return Err("Dijkstra Algorithm Failed: No Path Found".to_string());
            }
        }
        path.reverse();
        let mut current = self.start;
        let covered: BTreeSet<Point> = path.iter().fold(BTreeSet::new(), |mut acc, dir| {
            current = Point {
                x: (current.x as i32 + dir.x) as usize,
                y: (current.y as i32 + dir.y) as usize,
            };
            acc.insert(current);
            acc
        });
        Ok(DijkstraResult {
            start: self.start,
            end: self.end,
            covered,
            directions: path,
            distance: *dist.get(&self.end).unwrap_or(&f32::INFINITY),
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
    pub directions: Vec<Direction>,
    pub covered: BTreeSet<Point>, // Points in the path
    pub distance: f32,
}