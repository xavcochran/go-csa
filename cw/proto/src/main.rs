use csv::{Writer, WriterBuilder};
use indexmap::IndexSet;
use protobuf::Message;
use std::collections::HashSet;
use std::fs::{File, OpenOptions};
use std::time::Instant;
#[allow(unknown_lints)]
#[allow(clippy::all)]
#[allow(unused_attributes)]
#[cfg_attr(rustfmt, rustfmt::skip)]
#[allow(dead_code)]
#[allow(missing_docs)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
#[allow(trivial_casts)]
#[allow(unused_results)]
#[allow(unused_mut)]
mod generated {
    include!("generated/numbers.rs");
}

use generated::NumberArray;

fn decode_numbers(bytes: &[u8]) -> Result<IndexSet<u32>, protobuf::Error> {
    let number_array = NumberArray::parse_from_bytes(bytes)?;
    let numbers: IndexSet<u32> = number_array.values().iter().cloned().collect();
    Ok(numbers)
}

fn encode_numbers(numbers: &[u32]) -> Result<Vec<u8>, protobuf::Error> {
    let mut message = NumberArray::new();
    message.set_values(numbers.to_vec());
    let bytes = message.write_to_bytes()?;
    Ok(bytes)
}


fn test(run: i32, wtr: &mut Writer<File>) -> Result<(), protobuf::Error> {
    let mut nums = Vec::new();

    for x in 0..512 as u32 {
        for y in 0..512 as u32 {
            nums.push((x << 9 | y) as u32);
        }
    }
    println!("{}", nums.len());
    let now = Instant::now();
    let encoded_data: Vec<u8> = encode_numbers(&nums)?;
    let elapsed_encode = now.elapsed();
    println!("{}", encoded_data.len());
    let now = Instant::now();
    match decode_numbers(&encoded_data) {
        Ok(number_set) => {
            let elapsed_decode = now.elapsed();
            println!(
                "Decoded {} unique numbers in {:.2?}",
                number_set.len(),
                elapsed_decode
            );
            wtr.write_record(&[
                "Decode",
                &format!("{:?}", run),
                &format!("{:.2?}", elapsed_decode),
            ])
            .unwrap();
            wtr.flush().unwrap();
        }
        Err(e) => {
            eprintln!("Failed to decode numbers: {}", e);
        }
    }
    Ok(())
}

fn main() -> Result<(), protobuf::Error> {
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("proto_results.csv")
        .unwrap();
    let mut wtr = WriterBuilder::new().has_headers(false).from_writer(file);
    wtr.write_record(&["Operation", "Run", "Time (seconds)"])
        .unwrap();
    for i in 0..2000 {
        test(i, &mut wtr).unwrap();
    }
    Ok(())
}
