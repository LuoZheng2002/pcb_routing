use std::collections::{BTreeSet, HashMap, HashSet};

use shared::interface_types::{Color, ColorGrid};

#[derive(Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub enum Net {
    Character {
        pad_c: char,
        route_c: char,
    },
    Color {
        pad_color: Color,
        route_color: Color,
    },
}

#[derive(Debug, Clone, PartialEq, Hash, Eq, Copy, PartialOrd, Ord)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}
#[derive(Debug, Clone, PartialEq, Hash, Eq, Copy, PartialOrd, Ord)]
pub struct PointPair {
    start: Point,
    end: Point,
}

impl PointPair {
    pub fn new(point1: Point, point2: Point) -> Self {
        if point1 < point2 {
            PointPair {
                start: point1,
                end: point2,
            }
        } else {
            PointPair {
                start: point2,
                end: point1,
            }
        }
    }
    pub fn start(&self) -> Point {
        self.start
    }
    pub fn end(&self) -> Point {
        self.end
    }
}

// #[derive(Debug, Clone, PartialEq, Hash, Eq)]
// pub struct Pad{
//     pub net: Net,
//     pub point: Point,
// }

#[derive(Debug, Clone)]
pub struct Grid {
    pub pads: HashMap<Net, BTreeSet<Point>>,
    pub traces: HashMap<Net, HashSet<Point>>,
    pub diagonal_traces: HashMap<Net, HashSet<Point>>, // the point is at the top left corner of the diagonal trace
    pub width: usize,
    pub height: usize,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Grid {
            pads: HashMap::new(),
            traces: HashMap::new(),
            diagonal_traces: HashMap::new(),
            width,
            height,
        }
    }
    pub fn pads_except(&self, net: &Net) -> HashSet<Point> {
        self.pads
            .iter()
            .filter(|(n, _)| **n != *net)
            .flat_map(|(_, points)| points.iter())
            .cloned()
            .collect()
    }
    pub fn routes_except(&self, net: &Net) -> HashSet<Point> {
        self.traces
            .iter()
            .filter(|(n, _)| **n != *net)
            .flat_map(|(_, points)| points.iter())
            .cloned()
            .collect()
    }
    pub fn diagonal_routes_except(&self, net: &Net) -> HashSet<Point> {
        self.diagonal_traces
            .iter()
            .filter(|(n, _)| **n != *net)
            .flat_map(|(_, points)| points.iter())
            .cloned()
            .collect()
    }
    fn to_char_matrix(&self) -> Vec<Vec<char>> {
        let width = self.width;
        let height = self.height;
        let mut grid_string: Vec<Vec<char>> = vec![vec![' '; width as usize]; height as usize];
        for (net, points) in &self.pads {
            if let Net::Character { pad_c, route_c } = net {
                let net_char = pad_c;
                for point in points {
                    assert!(point.x < width && point.y < height, "Point out of bounds");
                    grid_string[point.y as usize][point.x as usize] = *net_char;
                }
            } else {
                panic!("Unsupported Net type for pads: {:?}", net);
            }
        }
        for (net, points) in &self.traces {
            if let Net::Character { pad_c, route_c } = net {
                let route_char = route_c;
                for point in points {
                    assert!(point.x < width && point.y < height, "Point out of bounds");
                    grid_string[point.y as usize][point.x as usize] = *route_char;
                }
            } else {
                panic!("Unsupported Net type for traces: {:?}", net);
            }
        }
        grid_string
    }

    fn build_grid_string(char_matrix: &Vec<Vec<char>>) -> String {
        let width = char_matrix[0].len();
        let horizontal_wall = "#".repeat(width + 2);
        let mut result = String::new();

        result.push_str(&horizontal_wall);
        result.push('\n');

        for row in char_matrix {
            result.push('#');
            result.push_str(&row.iter().collect::<String>());
            result.push('#');
            result.push('\n');
        }

        result.push_str(&horizontal_wall);
        result.push('\n');

        result
    }
    pub fn print(&self) {
        let char_matrix = self.to_char_matrix();
        let result = Self::build_grid_string(&char_matrix);
        println!("{}", result);
    }
    pub fn to_string(&self) -> String {
        let char_matrix = self.to_char_matrix();
        Self::build_grid_string(&char_matrix)
    }
    pub fn from_string(s: &str) -> Self {
        let mut lines = s.lines().collect::<Vec<&str>>();
        let first_line = lines[0];
        let width = first_line.len() as usize - 2; // subtract 2 for the walls
        assert!(
            lines.len() >= 3,
            "Grid must have at least 3 lines (top wall, bottom wall, and one row of data)"
        );
        lines.pop(); // remove the last line (bottom wall)
        lines.remove(0); // remove the first line (top wall)
        let height = lines.len() as usize;
        let mut pads: HashMap<Net, BTreeSet<Point>> = HashMap::new();
        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c != ' ' && c != '#' {
                    let point = Point {
                        x: x as usize - 1,
                        y: y as usize,
                    };
                    let net = Net::Character {
                        pad_c: c,
                        route_c: c.to_ascii_lowercase(),
                    }; // assuming pad and route characters are the same
                    pads.entry(net).or_default().insert(point);
                }
            }
        }
        Grid {
            pads,
            traces: HashMap::new(),
            diagonal_traces: HashMap::new(),
            width,
            height,
        }
    }
    pub fn remove_pad(&mut self, point: Point) {
        let prev_pads = std::mem::take(&mut self.pads);
        self.pads = prev_pads
            .into_iter()
            .filter_map(|(net, points)| {
                let mut new_points = points.clone();
                new_points.remove(&point);
                if new_points.is_empty() {
                    None // remove the net if it has no pads left
                } else {
                    Some((net, new_points))
                }
            })
            .collect();
    }
    pub fn insert_pad(&mut self, net: Net, point: Point) {
        self.pads.entry(net).or_default().insert(point);
    }
    pub fn to_color_grid(&self) -> ColorGrid {
        let mut color_grid = vec![
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
        for (net, points) in &self.pads {
            if let Net::Color {
                pad_color,
                route_color: _,
            } = net
            {
                for point in points {
                    assert!(
                        point.x < self.width && point.y < self.height,
                        "Point out of bounds"
                    );
                    color_grid[point.y][point.x] = pad_color.clone();
                }
            } else {
                panic!("Unsupported Net type for pads: {:?}", net);
            }
        }
        for (net, points) in &self.traces {
            if let Net::Color {
                pad_color: _,
                route_color,
            } = net
            {
                for point in points {
                    assert!(
                        point.x < self.width && point.y < self.height,
                        "Point out of bounds"
                    );
                    color_grid[point.y][point.x] = route_color.clone();
                }
            } else {
                panic!("Unsupported Net type for traces: {:?}", net);
            }
        }
        ColorGrid { grid: color_grid }
    }
}
