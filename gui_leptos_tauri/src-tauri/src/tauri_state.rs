use std::{collections::HashMap, sync::Mutex};

use lazy_static::lazy_static;

use crate::{grid::Grid, proba_grid::{ProbaGridInput, ProbaGridState}};

pub struct TauriState{
    pub naive_grid: Grid,
    pub proba_grid: ProbaGridState,
}

lazy_static!{
    pub static ref TAURI_STATE: Mutex<TauriState> = Mutex::new(TauriState {
        naive_grid: Grid::new(10, 10), // Initialize with a default grid size
        proba_grid: ProbaGridState::Uninitialized { input: ProbaGridInput{width: 10, height: 10, nets: HashMap::new(), pads: HashMap::new()} }
    });
}