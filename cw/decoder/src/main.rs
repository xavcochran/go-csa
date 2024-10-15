use bytes::{buf, BytesMut};
use std::cell;
use std::time::Instant;
use std::{collections::HashSet, fs::read, vec};
use tokio::{io::AsyncReadExt, net::TcpStream};

const BYTE: usize = 8;
const HEADER_SIZE_BYTES: usize = 8;
const VERSION: usize = 0;
const FUNCTION_CALL: usize = 1;
const MESSAGE_ID: usize = 2;
const LENGTH: usize = 4;
const CHECKSUM: usize = 6;

const PGM_LINE_SIZE: usize = 512;
const NUM_OF_U64_PER_PGM_LINE: usize = PGM_LINE_SIZE / 64;

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct Cell {
    xy: u32,
}
impl Cell {
    pub fn new(xy: u32) -> Self {
        Self { xy }
    }

    pub fn neighbours(xy: u32, cells: &HashSet<u32>) -> u8 {
        let mut live_neighbours = 0;
        let neighbour_positions = vec![
            xy + 512, // right
            xy - 512, // left
            xy + 1,   // up
            xy - 1,   // down
            xy + 513, // right up
            xy + 511, // right down
            xy - 511, // left down
            xy - 513, // left up
        ];

        for &pos in &neighbour_positions {
            if cells.contains(&pos) {
                live_neighbours += 1;
            }
        }

        live_neighbours
    }
}

#[derive(Debug, Clone)]
struct Header {
    version: u8,
    fn_call: u8,
    msg_id: u16,
    length: u32,
    checksum: u16,
}

impl Header {
    pub fn new() -> Self {
        Self {
            version: 0,
            fn_call: 0,
            msg_id: 0,
            length: 0,
            checksum: 0,
        }
    }
}

#[derive(Debug, Clone)]
struct Packet {
    header: Header,
}

impl Packet {
    fn decode_header(&mut self, data: &[u8]) {
        self.header = Header {
            version: data[VERSION], // first byte
            fn_call: data[FUNCTION_CALL], // second byte
            msg_id: ((data[MESSAGE_ID] as u16) << BYTE | (data[MESSAGE_ID + 1] as u16)), // 3rd & 4th byte
            length: ((data[LENGTH] as u32) << BYTE | (data[LENGTH + 3] as u32)), // 5th -> 8th byte
            checksum: ((data[CHECKSUM] as u16) << BYTE | (data[CHECKSUM + 1] as u16)), // 9th & 10th byte
        }
    }

    fn decode_payload(&mut self, data: &[u8]) -> HashSet<u32> {
        let mut buffer: u32 = 0;
        let mut bit_count = 7;
        let size = self.header.length as usize;
        let mut cells = HashSet::with_capacity(size);

        for byte in data {
            buffer |= (*byte as u32) << 31 - bit_count; // adds next byte to the buffer
            bit_count += BYTE;

            // while there is no space to shift, process first 18 bits
            while bit_count >= 24 {
                let extracted_value = (buffer & 0xFFFFC000) >> 14; // get first 18 bits then shift to right hand side

                cells.insert(extracted_value);

                buffer <<= 18; // shift buffer to the right by 18 bits
                bit_count -= 18; // decrease bit count to account for bits just extracted
            }
        }
        return cells;
    }


    pub async fn decode(&mut self, mut stream: TcpStream) -> HashSet<u32> {
        let mut buf = BytesMut::with_capacity(64);

        match stream.read_buf(&mut buf).await {
            Ok(0) => {
                return HashSet::new();
            }
            Ok(n) => {
                if n != 64 {
                    println!("there has been an size miss match streaming this line of data");
                    return HashSet::new();
                } else {
                    self.decode_header(&buf);
                    return self.decode_payload(&buf);
                }
            }
            Err(e) => {
                println!("Failed to read from socket; err = {:?}", e);
                return HashSet::new();
            }
        }
    }
}

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let mut world: Vec<u8> = Vec::with_capacity(512 * 512);
    let mut buffer: u32 = 0;
    let mut bit_count: usize = 17;
    let mask: u32 = 0xFF000000;
    for x in 0..512 as u32 {
        for y in 0..512 as u32 {
            let new_num: u32 = x << 9 as u32 | y;
            let shifted_num = new_num << 31 - bit_count;
            buffer = buffer | shifted_num;

            bit_count += 18;

            while bit_count >= 32 {
                let byte = buffer & mask;
                bit_count -= BYTE;
                buffer <<= BYTE;

                world.push((byte >> 24) as u8);
            }
        }
    }

    while buffer != 0 {
        let byte = (buffer & 0xFF000000) >> 24;
        world.push(byte as u8);
        buffer <<= BYTE;
    }

    println!("size {}", world.len());

    let header = Header {
        version: 0,
        fn_call: 0,
        msg_id: 0,
        length: world.len().clone() as u32,
        checksum: 0,
    };

    let mut packet = Packet { header };
    println!("{:?}", packet);
    let mut cells_processed = 0;
    let now = Instant::now();
    let cells = packet.decode_payload(&world);
    let elapsed = now.elapsed();
    for cell in &cells {
        let x = (cell & 0x3FF00) >> 9;
        let y = cell & 0x1FF;
        if x == 511 && y == 511 {
            println!("X: {}, {:032b}", x, x);

            println!("Y: {}, {:032b}\n", y, y);
        }
        // let live = Cell::neighbours(*cell, &cells);

        cells_processed += 1;
    }
    println!(
        "{} cells processed in {:.2?} seconds",
        cells_processed, elapsed
    );
}
