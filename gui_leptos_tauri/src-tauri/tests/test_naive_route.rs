use std::{collections::HashMap, fs, path::Path};

use gui_leptos_tauri_lib::{grid::Net, grid::Point, grid::Grid, naive_route::naive_route};



#[test]
fn test_naive_route() {
    let pads = [
        (Net{pad_c: 'A', route_c: 'a'}, [Point{x: 3, y: 3}, Point{x: 9, y: 3}].into()),
        (Net{pad_c: 'B', route_c: 'b'}, [Point{x: 5, y: 1}, Point{x: 5, y: 5}].into()),
        (Net{pad_c: 'C', route_c: 'c'}, [Point{x: 7, y: 1}, Point{x: 7, y: 5}].into()),
    ].into();
    let grid = Grid{
        pads,
        traces: HashMap::new(),
        diagonal_traces: HashMap::new(),
        width: 12,
        height: 8,
    };
    let test_problem_solution_pairs: Vec<(Grid, String)> = vec![
        (grid, "../../test_data/test_naive_route1.txt".to_string()),

    ];
    for (grid, expected_path) in test_problem_solution_pairs {
        let expected = fs::read_to_string(expected_path)
            .expect("Failed to read expected output file");
        let mut output = grid.to_string();
        // this is the core algorithm that we are testing
        let routed_grid = naive_route(grid);
        output.push_str(&routed_grid.to_string());
        let output = output.replace("\r\n", "\n"); // Normalize line endings
        let expected = expected.replace("\r\n", "\n"); // Normalize line endings
        assert_eq!(output, expected, "Output does not match expected. Output:\n{}", output);    
    }       
}