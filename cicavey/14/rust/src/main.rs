extern crate bit_vec;
use bit_vec::BitVec;

extern crate modulo;
use modulo::Mod;

fn get_bit(bvs: &Vec<BitVec>, c: (usize, usize)) -> bool {
    bvs[c.1].get(c.0).unwrap()
}

fn set_bit(bvs: &mut Vec<BitVec>, c: (usize, usize)) {
    bvs[c.1].set(c.0, true);
}

fn main() {
    let base = "nbysizxe";
    let bvs: Vec<BitVec> = (0..128)
        .map(|s| format!("{}-{}", base, s))
        .map(|s| knot_hash(&s))
        .map(|kh| BitVec::from_bytes(&kh)).collect();
    // Count all bits
    let part1: usize = bvs.iter().map(|bv| bv.iter().filter(|x| *x).count())
        .sum();
    println!("{:?}", part1);

    // Flood fill ...
    let mut dst_bvs: Vec<BitVec> = (0..128).map(|_| BitVec::from_elem(128, false)).collect();
    // Iterate over bits, if not already marked, flood fill from each and mark consumed bits in target
    let mut rc: usize = 0;

    // for i in 0..10 {
    //     println!("S:{:?}", bvs[i]);    
    // }

    for y in 0..128 {
        for x in 0..128 {
            // Is source bit even set?
            if !get_bit(&bvs, (x,y)) {
                continue;
            }
            
            // Bit already set in dst, skip
            // Already filled
            if get_bit(&dst_bvs, (x, y)) {
                continue;
            }

            // Begin filling from here. Mark bits set in dst
            // Fill queue with positions (ignoring off bounds)
            let mut q = vec![(x,y)];
            while !q.is_empty() {
                let cur = q.pop().unwrap();
                let (cx, cy) = cur;

                //println!("{:?} {:?}", cur, q);

                // Mark cur in dst
                if !get_bit(&dst_bvs, cur) {
                    set_bit(&mut dst_bvs, cur);

                    // Enqueue all valid neighbors (within grid)
                    // N
                    if cy > 0 { // N exists
                        let nc = (cx, cy-1);
                        if get_bit(&bvs, nc) {
                            q.push(nc);
                        }
                    }
                    if cy < 127 {
                        let nc = (cx, cy+1);
                        if get_bit(&bvs, nc) {
                            q.push(nc);
                        }
                    }
                    if cx > 0 {
                        let nc = (cx-1, cy);
                        if get_bit(&bvs, nc) {
                            q.push(nc);
                        }
                    }
                    if cx < 127 {
                        let nc = (cx+1, cy);
                        if get_bit(&bvs, nc) {
                            q.push(nc);
                        }
                    }
                }
            }

            rc +=1;
        }

        //break;
    }

    // for i in 0..10 {
    //     println!("D:{:?}", dst_bvs[i]);    
    // }

    println!("{:?}", rc);
}

fn knot_hash(key : &str) -> [u8;16] {
     let mut lens: Vec<usize> = key.bytes().map(|c| c as usize).collect();
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
    //let t: Vec<String> = data.chunks(16).map(xor).map(to_hex).collect();
    let b: Vec<u8> = data.chunks(16).map(xor).map(|u| u as u8).collect();
    let mut out: [u8;16] = [0; 16];
    out.clone_from_slice(&b);
    return out;
}

fn xor(a: &[usize]) -> usize {
    let mut acc = a[0];
    for i in 1..a.len() {
        acc ^= a[i];
    }
    return acc;
}