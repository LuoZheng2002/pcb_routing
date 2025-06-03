use std::collections::{HashMap, HashSet};

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
    pub pads: HashMap<Net, HashSet<Point>>,
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
    fn to_string(&self) -> Vec<Vec<char>>{
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

    fn print_grid_string(grid_string: &Vec<Vec<char>>) {
        let width = grid_string[0].len();
        let horizontal_wall = "#".repeat(width + 2);
        println!("{}", horizontal_wall);
        for row in grid_string {
            println!("#{}#", row.iter().collect::<String>());
        }
        println!("{}", horizontal_wall);
    }
    pub fn print(&self) {
        let grid_string = self.to_string();
        Self::print_grid_string(&grid_string);
    }
}

