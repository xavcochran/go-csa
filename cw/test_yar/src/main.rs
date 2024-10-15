use serde::{Serialize, Deserialize};
use serde_json::{to_string, from_str};
use std::{collections::HashSet, time::Instant};
use std::mem

#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq)]
struct Cell {
    x: u32,
    y: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct World {
    cells: HashSet<Cell>,
}

fn main() {
    // Generate an array of numbers from 0 to 511
    let mut cells: HashSet<Cell> = HashSet::with_capacity(512 * 512);

    for x in 0..512 {
        for y in 0..512 {
            cells.insert(Cell { x, y });
        }
    }

    let world = World { cells };

    // Serialize the world to a JSON string
    let json = to_string(&world).unwrap();

    // Print the JSON string
    let now = Instant::now();

    
    // Deserialize the JSON string back into a World struct
    let deserialized_world: World = from_str(&json).unwrap();
    
    // Extract the vector of cells from the deserialized World struct
    let elapsed = now.elapsed();
    let deserialized_cells = deserialized_world.cells;
    
    println!("{}", deserialized_cells.len());
    println!(
        "cells processed in {:.2?} seconds",
        elapsed
    );
}