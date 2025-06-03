
pub mod datatypes;
pub mod functions;





// use std::collections::HashMap;

// use functions::{naive_route, print_grid, print_grid_string};
// use lazy_static::lazy_static;



// use datatypes::{Grid, Net, Point};

// fn main() {
//     println!("Hello, world!");
//     let pads = [
//         (Net{pad_c: 'A', route_c: 'a'}, [Point{x: 3, y: 3}, Point{x: 9, y: 3}].into()),
//         (Net{pad_c: 'B', route_c: 'b'}, [Point{x: 5, y: 1}, Point{x: 5, y: 5}].into()),
//         (Net{pad_c: 'C', route_c: 'c'}, [Point{x: 7, y: 1}, Point{x: 7, y: 5}].into()),
//     ].into();
//     let grid = Grid{
//         pads,
//         traces: HashMap::new(),
//         diagonal_traces: HashMap::new(),
//         width: 12,
//         height: 8,
//     };
//     print_grid(&grid);
//     let routed_grid = naive_route(grid);
//     print_grid(&routed_grid);


//     let pads = [
//         (Net{pad_c: 'A', route_c: 'a'}, [Point{x: 3, y: 3}, Point{x: 9, y: 3}].into()),
//         (Net{pad_c: 'B', route_c: 'b'}, [Point{x: 5, y: 1}, Point{x: 7, y: 5}].into()),
//         (Net{pad_c: 'C', route_c: 'c'}, [Point{x: 7, y: 1}, Point{x: 5, y: 5}].into()),
//     ].into();
//     let grid = Grid{
//         pads,
//         traces: HashMap::new(),
//         diagonal_traces: HashMap::new(),
//         width: 12,
//         height: 8,
//     };
//     print_grid(&grid);
//     let routed_grid = naive_route(grid);
//     print_grid(&routed_grid);
// }

