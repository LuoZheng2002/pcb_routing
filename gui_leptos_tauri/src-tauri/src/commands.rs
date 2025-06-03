use std::io::{Read, Write};

use shared::datatypes::{Color, ColorGrid, MyResult};

use crate::TCP_STREAM;

const FETCH_GRID: &str = "fetch_grid";

const USE_PYTHON_SERVER: bool = true;

fn fetch_from_python_server<T>() -> Result<T, String>
where
    T: serde::de::DeserializeOwned,
{
    println!("Sending request to Python server");
    let message = FETCH_GRID.to_string();
    // Send JSON message
    let serialized = serde_json::to_string(&message).map_err(|e| e.to_string())?;
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


fn fetch_grid_local() -> Result<ColorGrid, String> {
    todo!("Implement local grid fetching logic");
}

#[tauri::command]
pub fn fetch_grid() -> MyResult<ColorGrid, String> {
    if USE_PYTHON_SERVER {
        match fetch_from_python_server::<ColorGrid>() {
            Ok(grid) => MyResult::Ok(grid),
            Err(e) => MyResult::Err(e),
        }
    } else {
        match fetch_grid_local() {
            Ok(grid) => MyResult::Ok(grid),
            Err(e) => MyResult::Err(e),
        }
    }
}