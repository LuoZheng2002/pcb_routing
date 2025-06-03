use std::{cmp::Ordering, collections::{BinaryHeap, HashMap, HashSet}};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LogArgs {
    pub message: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum MyResult<T, E> {
    Ok(T),
    Err(E),
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Color{
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ColorGrid{
    pub grid: Vec<Vec<Color>>,
}
