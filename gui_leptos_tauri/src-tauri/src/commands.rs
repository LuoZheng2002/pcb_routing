use std::{io::{Read, Write}, vec};

use shared::interface_types::{ClickCellArgs, Color, ColorGrid, MyResult, NewGridArgs};

use crate::{grid::{Grid, Net, Point}, naive_route::naive_route, tauri_state::TAURI_STATE, TCP_STREAM};

const USE_PYTHON_SERVER: bool = false;

fn call_python_server<In, Out>(function_name: &str, input_args: In) -> Result<Out, String>
where
    In: serde::Serialize,
    Out: serde::de::DeserializeOwned,
{
    println!("Sending request to Python server");
    // Send JSON message
    let serialized = serde_json::to_string(&(function_name, input_args)).map_err(|e| e.to_string())?;
    let mut tcp_stream = TCP_STREAM.lock().unwrap();
    println!("before writing");
    tcp_stream.write_all(serialized.as_bytes()).map_err(|e| e.to_string())?;
    println!("after writing");
    
    // Read response
    let mut buffer = [0u8; 512];
    println!("before reading");
    
    match tcp_stream.read(&mut buffer) {
        Ok(size) => {
            let response = String::from_utf8_lossy(&buffer[..size]);
            println!("Received response: {}", response);
            serde_json::from_str(&response).map_err(|e| e.to_string())
        },
        Err(e) => Err(e.to_string()),
    }
}


fn new_grid_local(rows: usize, cols: usize) -> Result<ColorGrid, String> {
    println!("Creating new grid locally ");
    let mut tauri_state = TAURI_STATE.lock().unwrap();
    tauri_state.grid = Grid::new(rows, cols);
    let grid = tauri_state.grid.to_color_grid();
    Ok(grid)
}

#[tauri::command]
pub fn new_grid(rows: usize, cols: usize) -> MyResult<ColorGrid, String> {
    if USE_PYTHON_SERVER {
        match call_python_server::<NewGridArgs,ColorGrid>("new_grid", NewGridArgs { rows, cols }) {
            Ok(grid) => MyResult::Ok(grid),
            Err(e) => MyResult::Err(e),
        }
    } else {
        match new_grid_local(rows, cols) {
            Ok(grid) => MyResult::Ok(grid),
            Err(e) => MyResult::Err(e),
        }
    }
}

fn click_cell_local(
    x: usize,
    y: usize,
    r: u8,
    g: u8,
    b: u8,
) -> Result<ColorGrid, String> {
    let mut tauri_state = TAURI_STATE.lock().unwrap();
    let grid = &mut tauri_state.grid;
    let width = grid.width;
    let height = grid.height;
    if x >= width || y >= height {
        return Err(format!("Point ({}, {}) is out of bounds for grid size {}x{}", x, y, width, height));
    }
    if let (255, 255, 255) = (r, g, b) {
        // If the color is white, remove the cell
        grid.remove_pad(Point { x, y});
    } else {
        // Otherwise, set the color
        grid.insert_pad(Net::Color { pad_color: Color { r, g, b }, route_color: Color{
            r: u32::clamp((r as u32 + 255)/2, 0, 255) as u8,
            g: u32::clamp((g as u32 + 255)/2, 0, 255) as u8,
            b: u32::clamp((b as u32 + 255)/2, 0, 255) as u8,
        } }, Point { x, y });
    }
    let grid = grid.to_color_grid();
    Ok(grid)
}

#[tauri::command]
pub fn click_cell(
    x: usize,
    y: usize,
    r: u8,
    g: u8,
    b: u8,
) -> MyResult<ColorGrid, String> {
    if USE_PYTHON_SERVER {
        match call_python_server::<ClickCellArgs, ColorGrid>(
            "click_cell",
            ClickCellArgs {
                x,
                y,
                r,
                g,
                b,
            },
        ) {
            Ok(grid) => MyResult::Ok(grid),
            Err(e) => MyResult::Err(e),
        }
    } else {
        match click_cell_local(x, y, r, g, b) {
            Ok(grid) => MyResult::Ok(grid),
            Err(e) => MyResult::Err(e),
        }
    }
}



fn do_naive_route_local() -> Result<ColorGrid, String> {
    let mut tauri_state = TAURI_STATE.lock().unwrap();
    let old_grid = tauri_state.grid.clone();
    tauri_state.grid = naive_route(old_grid)?;
    let color_grid = tauri_state.grid.to_color_grid();
    Ok(color_grid)
}

#[tauri::command]
pub fn do_naive_route() -> MyResult<ColorGrid, String> {
    if USE_PYTHON_SERVER {
        match call_python_server::<(), ColorGrid>("naive_route", ()) {
            Ok(grid) => MyResult::Ok(grid),
            Err(e) => MyResult::Err(e),
        }
    } else {
        match do_naive_route_local() {
            Ok(grid) => MyResult::Ok(grid),
            Err(e) => MyResult::Err(e),
        }
    }
}