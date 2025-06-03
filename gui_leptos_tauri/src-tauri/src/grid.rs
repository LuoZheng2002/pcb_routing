use std::collections::{BTreeSet, HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Hash, Eq, PartialOrd, Ord)]
pub struct Net{
    pub pad_c: char,
    pub route_c: char,
}

#[derive(Debug, Clone, PartialEq, Hash, Eq, Copy, PartialOrd, Ord)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct Pad{
    pub net: Net,
    pub point: Point,
}

#[derive(Debug, Clone)]
pub struct Grid{
    pub pads: HashMap<Net, BTreeSet<Point>>,
    pub traces: HashMap<Net, HashSet<Point>>,
    pub diagonal_traces: HashMap<Net, HashSet<Point>>, // the point is at the top left corner of the diagonal trace
    pub width: u32,
    pub height: u32,
}

impl Grid{
    pub fn pads_except(&self, net: &Net) -> HashSet<Point> {
        self.pads.iter()
            .filter(|(n, _)| **n != *net)
            .flat_map(|(_, points)| points.iter())
            .cloned()
            .collect()
    }
    pub fn routes_except(&self, net: &Net) -> HashSet<Point> {
        self.traces.iter()
            .filter(|(n, _)| **n != *net)
            .flat_map(|(_, points)| points.iter())
            .cloned()
            .collect()
    }
    pub fn diagonal_routes_except(&self, net: &Net) -> HashSet<Point> {
        self.diagonal_traces.iter()
            .filter(|(n, _)| **n != *net)
            .flat_map(|(_, points)| points.iter())
            .cloned()
            .collect()
    }
    fn to_char_matrix(&self) -> Vec<Vec<char>>{
        let width = self.width;
        let height = self.height;
        let mut grid_string: Vec<Vec<char>> = vec![vec![' '; width as usize]; height as usize];
        for (net, points) in &self.pads {
            let net_char = net.pad_c;
            for point in points {
                assert!(point.x < width && point.y < height, "Point out of bounds");
                grid_string[point.y as usize][point.x as usize] = net_char;
            }
        }
        for (net, points) in &self.traces {
            let route_char = net.route_c;
            for point in points {
                assert!(point.x < width && point.y < height, "Point out of bounds");
                grid_string[point.y as usize][point.x as usize] = route_char;
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
}

