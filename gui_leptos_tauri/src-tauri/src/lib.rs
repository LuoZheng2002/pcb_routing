use std::{net::TcpStream, sync::Mutex};

pub mod commands;
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use lazy_static::lazy_static;

use crate::commands::fetch_grid;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            fetch_grid,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// add a lazy static variable for TCP stream
lazy_static! {
    pub static ref TCP_STREAM: Mutex<TcpStream> = {
        println!("Connecting to Python server");
        let stream = TcpStream::connect("127.0.0.1:4000").unwrap();
        println!("Connected to Python server");
        Mutex::new(stream)
    };
}
