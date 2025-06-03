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


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Color{
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ColorGrid{
    pub grid: Vec<Vec<Color>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NewGridArgs{
    pub rows: usize,
    pub cols: usize,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClickCellArgs{
    pub x: usize,
    pub y: usize,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}