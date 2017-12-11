use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;

extern crate modulo;
use modulo::Mod;

fn main() {
    let part1 = false;
    let file = File::open("../input.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();

    // part 1
    if part1 {
        let lens: Vec<usize> = contents.split(",").map(|s| s.parse().unwrap()).collect();
        let data_size = 256;
        let mut data = vec![0; data_size];
        for i in 0..data_size {
            data[i] = i as usize;
        }
        let mut idx: usize = 0;
        let mut skip: usize = 0;
        for len in lens {
            let r = data.clone(); // create a read-only clone to do the reverse indexing
            // Reverse iterate
            let start = (idx + len - 1) % data_size;
            for i in 0..len {
                let dst_idx = (idx + i) % data_size;
                let src_idx = (start as i32 - i as i32).modulo(data_size as i32) as usize;
                data[dst_idx] = r[src_idx];
            }
            idx += (len + skip) % data_size;
            skip += 1;
        }
        println!("{}", data[0] * data[1]);
    } else {
        // part 2
        let mut lens: Vec<usize> = contents.bytes().map(|c| c as usize).collect();
        let mut std: Vec<usize> = vec![17, 31, 73, 47, 23];
        lens.append(&mut std);
        
        let data_size = 256;
        let mut data = vec![0; data_size];
        for i in 0..data_size {
            data[i] = i as usize;
        }

        let mut idx: usize = 0;
        let mut skip: usize = 0;
        for _ in 0..64 {
            for len in &lens {
                let r = data.clone(); // create a read-only clone to do the reverse indexing
                // Reverse iterate
                let start = (idx + len - 1) % data_size;
                for i in 0..*len {
                    let dst_idx = (idx + i) % data_size;
                    let src_idx = (start as i32 - i as i32).modulo(data_size as i32) as usize;
                    data[dst_idx] = r[src_idx];
                }
                idx += (len + skip) % data_size;
                skip += 1;
            }
        }
        let t: Vec<String> = data.chunks(16).map(xor).map(to_hex).collect();
        
        println!("{}", t.join(""));
    }
}

const CHARS: &'static [u8] = b"0123456789abcdef";

fn to_hex(data: usize) -> String {
        let mut v = Vec::with_capacity(2);
        v.push(CHARS[(data >> 4) as usize]);
        v.push(CHARS[(data & 0xf) as usize]);

        unsafe {
            String::from_utf8_unchecked(v)
        }
}

fn xor(a: &[usize]) -> usize {
    let mut acc = a[0];
    for i in 1..a.len() {
        acc ^= a[i];
    }
    return acc;
}