use std::{cell, mem};
use std::{collections::HashSet, time::Instant};
use indexmap::IndexSet;
fn main() {
    // Generate an array of numbers from 0 to 511
    let mut bytes = Vec::new();
    for x in 0..512 as u32 {
        for y in 0..512 as u32 {
            bytes.push(((x >> 24) & 0xff) as u8);
            bytes.push(((x >> 16) & 0xff) as u8);
            bytes.push(((x >> 8) & 0xff) as u8);
            bytes.push((x & 0xff) as u8);
            bytes.push(((y >> 24) & 0xff) as u8);
            bytes.push(((y >> 16) & 0xff) as u8);
            bytes.push(((y >> 8) & 0xff) as u8);
            bytes.push((y & 0xff) as u8);
        }
    }

    println!("{:?}", bytes.len());
    let data = &bytes[..];

    // Print the JSON string
    let now = Instant::now();

    let mut cells: IndexSet<u64> = IndexSet::with_capacity(512 * 512);
    let mut buffer: u64 = 0;
    let mut bitcount: usize = 7;
    for byte in data {
        buffer |= (*byte as u64) << 63 - bitcount;
        bitcount += 8;

        while bitcount >= 64 {
            cells.insert(buffer);
            buffer = 0;
            bitcount -= 64;
        }
    }

    // Extract the vector of cells from the deserialized World struct
    let elapsed = now.elapsed();

    println!("{:?}", mem::size_of::<String>());
    let mut cells_processed = 0;
    for _ in cells {
        cells_processed += 1;
    }
    println!(
        "{} cells processed in {:.2?} seconds",
        cells_processed, elapsed
    );
}
