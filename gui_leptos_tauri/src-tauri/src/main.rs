// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::net::TcpStream;

fn main() {
    gui_leptos_tauri_lib::run()
}
