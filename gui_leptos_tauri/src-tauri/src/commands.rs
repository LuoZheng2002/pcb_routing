use std::{
    io::{Read, Write},
    vec,
};

use shared::interface_types::{ClickCellArgs, Color, ColorGrid, MyResult, NewGridArgs};

use crate::{
    grid::{Grid, Net, Point},
    hyperparameters::{OPPORTUNITY_COST_WEIGHT, SCORE_WEIGHT},
    naive_route::naive_route,
    proba_grid::{NetID, ProbaGridProblem, ProbaGridState},
    solve_proba_grid::{initialize_proba_grid, sample_new_traces, update_posterior},
    tauri_state::TAURI_STATE,
    TCP_STREAM,
};

const USE_PYTHON_SERVER: bool = false;

fn call_python_server<In, Out>(function_name: &str, input_args: In) -> Result<Out, String>
where
    In: serde::Serialize,
    Out: serde::de::DeserializeOwned,
{
    println!("Sending request to Python server");
    // Send JSON message
    let serialized =
        serde_json::to_string(&(function_name, input_args)).map_err(|e| e.to_string())?;
    let mut tcp_stream = TCP_STREAM.lock().unwrap();
    println!("before writing");
    tcp_stream
        .write_all(serialized.as_bytes())
        .map_err(|e| e.to_string())?;
    println!("after writing");

    // Read response
    let mut buffer = [0u8; 512];
    println!("before reading");

    match tcp_stream.read(&mut buffer) {
        Ok(size) => {
            let response = String::from_utf8_lossy(&buffer[..size]);
            println!("Received response: {}", response);
            serde_json::from_str(&response).map_err(|e| e.to_string())
        }
        Err(e) => Err(e.to_string()),
    }
}

fn naive_new_grid_local(rows: usize, cols: usize) -> Result<ColorGrid, String> {
    println!("Creating new grid locally ");
    let mut tauri_state = TAURI_STATE.lock().unwrap();
    tauri_state.naive_grid = Grid::new(rows, cols);
    let grid = tauri_state.naive_grid.to_color_grid();
    Ok(grid)
}

#[tauri::command]
pub fn naive_new_grid(rows: usize, cols: usize) -> MyResult<ColorGrid, String> {
    if USE_PYTHON_SERVER {
        match call_python_server::<NewGridArgs, ColorGrid>("new_grid", NewGridArgs { rows, cols }) {
            Ok(grid) => MyResult::Ok(grid),
            Err(e) => MyResult::Err(e),
        }
    } else {
        match naive_new_grid_local(rows, cols) {
            Ok(grid) => MyResult::Ok(grid),
            Err(e) => MyResult::Err(e),
        }
    }
}

fn naive_click_cell_local(x: usize, y: usize, r: u8, g: u8, b: u8) -> Result<ColorGrid, String> {
    let mut tauri_state = TAURI_STATE.lock().unwrap();
    let grid = &mut tauri_state.naive_grid;
    let width = grid.width;
    let height = grid.height;
    if x >= width || y >= height {
        return Err(format!(
            "Point ({}, {}) is out of bounds for grid size {}x{}",
            x, y, width, height
        ));
    }
    if let (255, 255, 255) = (r, g, b) {
        // If the color is white, remove the cell
        grid.remove_pad(Point { x, y });
    } else {
        // Otherwise, set the color
        grid.insert_pad(
            Net::Color {
                pad_color: Color { r, g, b },
                route_color: Color {
                    r: u32::clamp((r as u32 + 255) / 2, 0, 255) as u8,
                    g: u32::clamp((g as u32 + 255) / 2, 0, 255) as u8,
                    b: u32::clamp((b as u32 + 255) / 2, 0, 255) as u8,
                },
            },
            Point { x, y },
        );
    }
    let grid = grid.to_color_grid();
    Ok(grid)
}

#[tauri::command]
pub fn naive_click_cell(x: usize, y: usize, r: u8, g: u8, b: u8) -> MyResult<ColorGrid, String> {
    if USE_PYTHON_SERVER {
        match call_python_server::<ClickCellArgs, ColorGrid>(
            "click_cell",
            ClickCellArgs { x, y, r, g, b },
        ) {
            Ok(grid) => MyResult::Ok(grid),
            Err(e) => MyResult::Err(e),
        }
    } else {
        match naive_click_cell_local(x, y, r, g, b) {
            Ok(grid) => MyResult::Ok(grid),
            Err(e) => MyResult::Err(e),
        }
    }
}

fn proba_click_cell_local(x: usize, y: usize, r: u8, g: u8, b: u8) -> Result<ColorGrid, String> {
    let mut tauri_state = TAURI_STATE.lock().unwrap();
    let grid = &mut tauri_state.proba_grid;
    let grid = match grid {
        ProbaGridState::Uninitialized { input } => input,
        _ => return Err("Proba grid is already initialized".to_string()),
    };
    let width = grid.width;
    let height = grid.height;
    if x >= width || y >= height {
        return Err(format!(
            "Point ({}, {}) is out of bounds for grid size {}x{}",
            x, y, width, height
        ));
    }
    let color_to_net_id = vec![
        (255, 0, 0),   // Red
        (0, 255, 0),   // Green
        (0, 0, 255),   // Blue
        (255, 255, 0), // Yellow
        (255, 0, 255), // Orange
        (0, 255, 255), // Purple
    ]
    .into_iter()
    .enumerate()
    .map(|(i, (r, g, b))| ((r, g, b), NetID(i)))
    .collect::<std::collections::HashMap<_, _>>();
    if let (255, 255, 255) = (r, g, b) {
        // If the color is white, remove the cell
        grid.remove_pad(Point { x, y });
    } else {
        // Otherwise, set the color
        let net_id = color_to_net_id
            .get(&(r, g, b))
            .ok_or_else(|| format!("Color ({}, {}, {}) is not recognized", r, g, b))?;
        let pad_color = Color { r, g, b };
        let route_color = pad_color.clone();
        grid.insert_pad(*net_id, Point { x, y }, pad_color, route_color);
    }
    let grid = grid.to_color_grid();
    Ok(grid)
}

#[tauri::command]
pub fn proba_click_cell(x: usize, y: usize, r: u8, g: u8, b: u8) -> MyResult<ColorGrid, String> {
    if USE_PYTHON_SERVER {
        match call_python_server::<ClickCellArgs, ColorGrid>(
            "click_cell",
            ClickCellArgs { x, y, r, g, b },
        ) {
            Ok(grid) => MyResult::Ok(grid),
            Err(e) => MyResult::Err(e),
        }
    } else {
        match proba_click_cell_local(x, y, r, g, b) {
            Ok(grid) => MyResult::Ok(grid),
            Err(e) => MyResult::Err(e),
        }
    }
}

fn naive_do_route_local() -> Result<ColorGrid, String> {
    let mut tauri_state = TAURI_STATE.lock().unwrap();
    let old_grid = tauri_state.naive_grid.clone();
    tauri_state.naive_grid = naive_route(old_grid)?;
    let color_grid = tauri_state.naive_grid.to_color_grid();
    Ok(color_grid)
}

#[tauri::command]
pub fn naive_do_route() -> MyResult<ColorGrid, String> {
    if USE_PYTHON_SERVER {
        match call_python_server::<(), ColorGrid>("naive_route", ()) {
            Ok(grid) => MyResult::Ok(grid),
            Err(e) => MyResult::Err(e),
        }
    } else {
        match naive_do_route_local() {
            Ok(grid) => MyResult::Ok(grid),
            Err(e) => MyResult::Err(e),
        }
    }
}

fn proba_clear_local(rows: usize, cols: usize) -> Result<ColorGrid, String> {
    println!("Creating new grid locally ");
    let mut tauri_state = TAURI_STATE.lock().unwrap();
    tauri_state.proba_grid = ProbaGridState::Uninitialized {
        input: ProbaGridProblem {
            width: rows,
            height: cols,
            nets: std::collections::HashMap::new(),
            net_to_pads: std::collections::HashMap::new(),
        },
    };
    let grid = tauri_state.proba_grid.to_color_grid();
    Ok(grid)
}

#[tauri::command]
pub fn proba_clear(rows: usize, cols: usize) -> MyResult<ColorGrid, String> {
    if USE_PYTHON_SERVER {
        match call_python_server::<(), ColorGrid>("naive_route", ()) {
            Ok(grid) => MyResult::Ok(grid),
            Err(e) => MyResult::Err(e),
        }
    } else {
        match proba_clear_local(rows, cols) {
            Ok(grid) => MyResult::Ok(grid),
            Err(e) => MyResult::Err(e),
        }
    }
}

fn proba_init_local() -> Result<ColorGrid, String> {
    let mut tauri_state = TAURI_STATE.lock().unwrap();
    let grid = match &tauri_state.proba_grid {
        ProbaGridState::Uninitialized { input } => input,
        _ => return Err("Proba grid is already initialized".to_string()),
    };
    let grid_output = initialize_proba_grid(grid.clone())?;
    tauri_state.proba_grid = ProbaGridState::Initialized {
        output: grid_output,
    };
    let color_grid = tauri_state.proba_grid.to_color_grid();
    Ok(color_grid)
}

#[tauri::command]
pub fn proba_init() -> MyResult<ColorGrid, String> {
    if USE_PYTHON_SERVER {
        match call_python_server::<(), ColorGrid>("naive_route", ()) {
            Ok(grid) => MyResult::Ok(grid),
            Err(e) => MyResult::Err(e),
        }
    } else {
        match proba_init_local() {
            Ok(grid) => MyResult::Ok(grid),
            Err(e) => MyResult::Err(e),
        }
    }
}

fn proba_update_posterior_local(
    scoreWeight: f64,
    opportunityCostWeight: f64,
) -> Result<ColorGrid, String> {
    *SCORE_WEIGHT.lock().unwrap() = scoreWeight;
    *OPPORTUNITY_COST_WEIGHT.lock().unwrap() = opportunityCostWeight;
    let mut tauri_state = TAURI_STATE.lock().unwrap();
    let grid = match &mut tauri_state.proba_grid {
        ProbaGridState::Initialized { output } => output,
        _ => return Err("Proba grid is not initialized".to_string()),
    };
    update_posterior(grid)?;
    let color_grid = grid.to_color_grid();
    Ok(color_grid)
}

#[tauri::command]
pub fn proba_update_posterior(
    scoreWeight: f64,
    opportunityCostWeight: f64,
) -> MyResult<ColorGrid, String> {
    if USE_PYTHON_SERVER {
        match call_python_server::<(), ColorGrid>("naive_route", ()) {
            Ok(grid) => MyResult::Ok(grid),
            Err(e) => MyResult::Err(e),
        }
    } else {
        match proba_update_posterior_local(scoreWeight, opportunityCostWeight) {
            Ok(grid) => MyResult::Ok(grid),
            Err(e) => MyResult::Err(e),
        }
    }
}

fn proba_next_net_local() -> Result<ColorGrid, String> {
    // let mut tauri_state = TAURI_STATE.lock().unwrap();
    // let old_grid = tauri_state.grid.clone();
    // tauri_state.grid = naive_route(old_grid)?;
    // let color_grid = tauri_state.grid.to_color_grid();
    // Ok(color_grid)
    todo!()
}

#[tauri::command]
pub fn proba_next_net() -> MyResult<ColorGrid, String> {
    if USE_PYTHON_SERVER {
        match call_python_server::<(), ColorGrid>("naive_route", ()) {
            Ok(grid) => MyResult::Ok(grid),
            Err(e) => MyResult::Err(e),
        }
    } else {
        match proba_next_net_local() {
            Ok(grid) => MyResult::Ok(grid),
            Err(e) => MyResult::Err(e),
        }
    }
}

fn proba_next_pair_local() -> Result<ColorGrid, String> {
    // let mut tauri_state = TAURI_STATE.lock().unwrap();
    // let old_grid = tauri_state.grid.clone();
    // tauri_state.grid = naive_route(old_grid)?;
    // let color_grid = tauri_state.grid.to_color_grid();
    // Ok(color_grid)
    todo!()
}

#[tauri::command]
pub fn proba_next_pair() -> MyResult<ColorGrid, String> {
    if USE_PYTHON_SERVER {
        match call_python_server::<(), ColorGrid>("naive_route", ()) {
            Ok(grid) => MyResult::Ok(grid),
            Err(e) => MyResult::Err(e),
        }
    } else {
        match proba_next_pair_local() {
            Ok(grid) => MyResult::Ok(grid),
            Err(e) => MyResult::Err(e),
        }
    }
}

fn proba_sample_local() -> Result<ColorGrid, String> {
    let mut tauri_state = TAURI_STATE.lock().unwrap();
    let grid = match &mut tauri_state.proba_grid {
        ProbaGridState::Initialized { output } => output,
        _ => return Err("Proba grid is not initialized".to_string()),
    };
    sample_new_traces(grid)?;
    let color_grid = grid.to_color_grid();
    Ok(color_grid)
}

#[tauri::command]
pub fn proba_sample() -> MyResult<ColorGrid, String> {
    if USE_PYTHON_SERVER {
        match call_python_server::<(), ColorGrid>("naive_route", ()) {
            Ok(grid) => MyResult::Ok(grid),
            Err(e) => MyResult::Err(e),
        }
    } else {
        match proba_sample_local() {
            Ok(grid) => MyResult::Ok(grid),
            Err(e) => MyResult::Err(e),
        }
    }
}
