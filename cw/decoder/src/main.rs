use bytes::{buf, BytesMut};
use indexmap::IndexSet;
use std::fmt::Error;
use std::time::Instant;
use std::{cell, fmt};
use std::{fs::read, vec};
use tokio::{io::AsyncReadExt, net::TcpStream};
// originally used standard hashset but doesnt have order
// index set retains order of insertion
// this increases decode time by about 30-40% but i believe it is a worthy tradeoff

const BYTE: usize = 8;
const HEADER_SIZE_BYTES: usize = 8;
const VERSION: usize = 0;
const FUNCTION_CALL: usize = 1;
const MESSAGE_ID: usize = 2;
const IMAGE_SIZE: usize = 4;
const LENGTH: usize = 6;
const CHECKSUM: usize = 10;

const PGM_LINE_SIZE: usize = 512;
const NUM_OF_U64_PER_PGM_LINE: usize = PGM_LINE_SIZE / 64;

#[derive(Debug)]
pub enum DecodeError {
    Io(std::io::Error),
    Other(String),
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecodeError::Io(e) => write!(f, "IO error: {}", e),
            DecodeError::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct Cell {
    xy: u32,
}
impl Cell {
    pub fn new(xy: u32) -> Self {
        Self { xy }
    }

    pub fn neighbours(xy: u32, cells: &IndexSet<u32>) -> u8 {
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
    image_size: u16,
    length: u32,
    checksum: u16,
}

impl Header {
    pub fn new() -> Self {
        Self {
            version: 0,
            fn_call: 0,
            msg_id: 0,
            image_size: 0,
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
            version: data[VERSION],       // first byte
            fn_call: data[FUNCTION_CALL], // second byte
            msg_id: ((data[MESSAGE_ID] as u16) << BYTE | (data[MESSAGE_ID + 1] as u16)), // 3rd & 4th byte
            image_size: ((data[IMAGE_SIZE] as u16) << BYTE | (data[IMAGE_SIZE + 1] as u16)), // 5th & 6th byte
            // 7th -> 10th byte
            length: || -> u32 {
                let mut buf: u32 = 0;
                for byte in &data[LENGTH..LENGTH + 3] {
                    let mut bitcount = 7;
                    buf |= (*byte as u32) << 31 - bitcount;
                    bitcount += byte;
                }
                return buf;
            }(),
            checksum: ((data[CHECKSUM] as u16) << BYTE | (data[CHECKSUM + 1] as u16)), // 11th & 12th byte
        }
    }

    fn decode_payload(
        &mut self,
        data: &[u8],
        coordinate_length: u32,
        offset: u32,
    ) -> IndexSet<u32> {
        let mut buffer: u32 = 0;
        let mut bit_count = 7;
        let size = self.header.length as usize;
        let mut cells = IndexSet::with_capacity(size);

        let coordinate_length_usize: usize = coordinate_length as usize;
        for byte in data {
            buffer |= (*byte as u32) << 31 - bit_count; // adds next byte to the buffer
            bit_count += BYTE;

            // while there is no space to shift, process first 18 bits
            while bit_count >= 24 {
                let extracted_value = (buffer & 0xFFFFC000) >> offset; // get first 18 bits then shift to right hand side

                cells.insert(extracted_value);

                buffer <<= coordinate_length; // shift buffer to the right by 18 bits
                bit_count -= coordinate_length_usize; // decrease bit count to account for bits just extracted
            }
        }
        return cells;
    }

    pub async fn decode(&mut self, mut stream: TcpStream) -> Result<IndexSet<u32>, DecodeError> {
        let mut buf = BytesMut::with_capacity(10);

        let coordinate_length = || -> u32 {
            let mask: u32 = 1;
            let mut size = 0;
            let image_size = self.header.image_size as u32;
            for i in 0..32 as u32 {
                if image_size & (mask << i) > 0 {
                    size = i;
                }
            }
            return size * 2;
        }();
        let offset = 32 - coordinate_length;

        match stream.read_buf(&mut buf).await {
            Ok(0) => {
                return Ok(IndexSet::new());
            }
            Ok(n) => {
                if n != 10 {
                    return Err(DecodeError::Other(format!(
                        "Length missmatch, expected headersize of 10, got {}",
                        n
                    )));
                } else {
                    self.decode_header(&buf);
                    return Ok(IndexSet::new());
                }
            }
            Err(e) => {
                return Err(DecodeError::Other(format!(
                    "Failed to read from socket; err = {:?}",
                    e
                )));
            }
        }
    }

    pub fn encode_payload(
        &self,
        cells: IndexSet<u32>,
        coordinate_length: usize,
    ) -> Vec<u8> {
        let mut buffer: u32 = 0;
        let mut bit_count: usize = coordinate_length -1;
        let mask: u32 = 0xFF000000;
        let capacity = cells.len() as f64 * (coordinate_length as f64 / 8.0);
        let mut data = Vec::with_capacity(capacity as usize);
        for cell in cells {
            buffer |= cell << 31 - bit_count;
            bit_count += coordinate_length;
            while bit_count >= 32 {
                let byte = buffer & mask;
                bit_count -= BYTE;
                buffer <<= BYTE;

                data.push((byte >> 24) as u8);
            }
        }
        while buffer != 0 {
            let byte = (buffer & 0xFF000000) >> 24;
            data.push(byte as u8);
            buffer <<= BYTE;
        }
        data
    }
}

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
        image_size: 512,
        length: world.len().clone() as u32,
        checksum: 0,
    };

    let mut packet = Packet { header };
    println!("{:?}", packet);
    let mut cells_processed: u32 = 0;

    let now = Instant::now();
    let cells = packet.decode_payload(&world, 18, 14);
    let elapsed = now.elapsed();
    for cell in &cells {
        let x = (cell & 0x3FF00) >> 9;
        let y = cell & 0x1FF;
        // println!("X: {}, {:032b}", x, x);

        // let live = Cell::neighbours(*cell, &cells);

        cells_processed += 1;
    }
    println!(
        "{} cells processed in {:.2?} seconds",
        cells_processed, elapsed
    );

    let now = Instant::now();
    let new_payload = packet.encode_payload(cells, 18);
    let elapsed = now.elapsed();
    println!("encoded cells processed in {:.2?} seconds", elapsed);
    println!("{:?}", packet);
    let mut cells_processed = 0;
    let now = Instant::now();
    let cells = packet.decode_payload(&new_payload, 18, 14);
    let elapsed = now.elapsed();
    for cell in &cells {
        let x = (cell & 0x3FF00) >> 9;
        let y = cell & 0x1FF;
        // println!("X: {}, {:032b}", x, x);
        // println!("Y: {}, {:032b}", y, y);

        // let live = Cell::neighbours(*cell, &cells);

        cells_processed += 1;
    }
    println!(
        "{} cells processed in {:.2?} seconds",
        cells_processed, elapsed
    );
}
