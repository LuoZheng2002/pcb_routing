use std::{collections::HashMap, fs, path::Path};

use gui_leptos_tauri_lib::{grid::Net, grid::Point, grid::Grid, naive_route::naive_route};



#[test]
fn test_naive_route() {

    let test_files = vec![
        "../../test_data/test_naive_route1.txt",
        "../../test_data/test_naive_route2.txt",
        "../../test_data/test_naive_route3.txt",
        "../../test_data/test_naive_route4.txt",
    ];
    for test_file in test_files {
        let content = fs::read_to_string(test_file)
            .expect("Failed to read file");
        let content = content.replace("\r\n", "\n"); // Normalize line endings
        let input_and_output: Vec<&str> = content.split("input:\n").collect();
        assert_eq!(input_and_output.len(), 2);
        let input_and_output = input_and_output[1].split("output:\n").collect::<Vec<&str>>();
        assert_eq!(input_and_output.len(), 2);
        let input = input_and_output[0].trim();
        let expected_output = input_and_output[1].trim();
        let grid = Grid::from_string(input);
        let routed_grid = naive_route(grid).unwrap();
        let output = routed_grid.to_string();
        let output = output.trim();
        assert_eq!(output, expected_output, "Output does not match expected. Output:\n{}", output);        
    }   
}