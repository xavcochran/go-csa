use csv::{Writer, WriterBuilder};
use indexmap::IndexSet;
use std::fs::{File, OpenOptions};
use std::{cell, mem};
use std::{collections::HashSet, time::Instant};
fn test(run: i32, wtr: &mut Writer<File>) {
    // Generate an array of numbers from 0 to 511
    let mut bytes = Vec::new();
    let now = Instant::now();
    for x in 0..512 as u16 {
        for y in 0..512 as u16 {
            bytes.push(((x >> 8) & 0xff) as u8);
            bytes.push((x & 0xff) as u8);
            bytes.push(((y >> 8) & 0xff) as u8);
            bytes.push((y & 0xff) as u8);
        }
    }
    let elapsed_encode = now.elapsed();

    println!("{:?}", bytes.len());
    let data = &bytes[..];

    // Print the JSON string
    let now = Instant::now();
    let mut cells: IndexSet<u32> = IndexSet::with_capacity(bytes.len()/(32/8));

    let mut buffer: u32 = 0;
    let mut bitcount: usize = 7;
    for byte in data {
        buffer |= (*byte as u32) << 31 - bitcount;
        bitcount += 8;

        while bitcount >= 32 {
            cells.insert(buffer);
            buffer = 0;
            bitcount -= 32;
        }
    }

    // Extract the vector of cells from the deserialized World struct
    let elapsed_decode = now.elapsed();

    
    wtr.write_record(&[
        "Decode",
        &format!("{:?}", run),
        &format!("{:.2?}", elapsed_decode),
    ])
    .unwrap();
    wtr.flush().unwrap();
}

fn main() {
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("u32_results.csv")
        .unwrap();
    let mut wtr = WriterBuilder::new().has_headers(false).from_writer(file);
    wtr.write_record(&["Operation", "Run", "Time (seconds)"])
        .unwrap();
    for i in 0..2000 {
        test(i, &mut wtr)
    }
}
