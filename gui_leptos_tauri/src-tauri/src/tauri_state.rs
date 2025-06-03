use std::sync::Mutex;

use lazy_static::lazy_static;

use crate::grid::Grid;

pub struct TauriState{
    pub grid: Grid,
}

lazy_static!{
    pub static ref TAURI_STATE: Mutex<TauriState> = Mutex::new(TauriState {
        grid: Grid::new(10, 10), // Initialize with a default grid size
    });
}