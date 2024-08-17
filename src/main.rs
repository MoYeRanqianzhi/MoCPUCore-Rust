mod core;
mod pc;
mod registers;

use std::env::args;
use std::fs::read;
use std::io::Read;
use core::MoCPU;

fn main() {
    let args: Vec<String> = args().collect();
    let mut code: Vec<[u8; 4]> = Vec::new();

    for chunk in read(&args[1]).expect("无法读取的文件").chunks(4) {
        code.push(<[u8; 4]>::try_from(chunk).unwrap())
    }

    let mut m = MoCPU::std();
    m.run(decode(&code))
}

fn decode(raw: &Vec<[u8; 4]>) -> [[u8; 4]; 256] {
    let mut code: [[u8; 4]; 256] = [[0; 4]; 256];

    let raw = if raw.len() <= 256 { raw } else { &raw[..256] };

    for (index, item) in raw.iter().enumerate() {
        code[index] = *item;
    }
    code
}