use std::io::Result;

pub fn solve<L: Iterator<Item=Result<String>>>(input: &mut L) -> () {
    let line = input.next().unwrap().unwrap();
    let part_1_lengths = line.split(',').map(|s| s.parse::<u8>().unwrap()).collect();

    let mut list: Vec<u8> = (0..255).collect();
    list.push(255); // 0..256 overflows the u8 back to 0, resulting in an empty list

    run_round(&mut list, 0, 0, &part_1_lengths);

    println!("Result of list[0] * list[1]: {}", list[0] as usize * list[1] as usize);

    // reset for part two
    list = (0..255).collect();
    list.push(255);
    let mut skip = 0u32;
    let mut ind = 0u8;

    let part_2_lengths = {
        let mut standard_length_ext = vec![17u8, 31, 73, 47, 23];
        let mut lengths: Vec<u8> = line.into_bytes();
        lengths.append(&mut standard_length_ext);
        lengths
    };
    

    for _ in 0..64 {
        let (i, s) = run_round(&mut list, ind, skip, &part_2_lengths);
        ind = i;
        skip = s;
    }

    let dense = reduce_hash(&list);
    println!("Knot hash is {}", hexadecimal(&dense));
}

// returns (index, skip)
fn run_round(list: &mut Vec<u8>, cur_pos: u8, skip: u32, lengths: &Vec<u8>) -> (u8 , u32) {
    let mut ind = cur_pos;
    let mut skip = skip;

    for length in lengths {
        let mut left = ind;
        let mut right = (ind.wrapping_add(*length)).wrapping_sub(1);

        let mut ops_left = length / 2;
        loop {
            if ops_left == 0 {
                break;
            }

            list.swap(left as usize, right as usize);
            left = left.wrapping_add(1);
            right = right.wrapping_sub(1);
            ops_left -= 1;
        }
        
        ind = ind.wrapping_add(*length);
        ind = (((ind as u32) + skip) % 256) as u8;
        skip += 1;
    }

    (ind, skip)
}

fn reduce_hash(sparse: &Vec<u8>) -> Vec<u8> {
    assert!(sparse.len() == 256);

    let mut dense = vec![0u8; 16];
    for i in 0..16 {
        let ind = i * 16;
        dense[i] = sparse[ind+0] ^ sparse[ind+1] ^ sparse[ind+2] ^ sparse[ind+3] ^
            sparse[ind+4] ^ sparse[ind+5] ^ sparse[ind+6] ^ sparse[ind+7] ^
            sparse[ind+8] ^ sparse[ind+9] ^ sparse[ind+10] ^ sparse[ind+11] ^
            sparse[ind+12] ^ sparse[ind+13] ^ sparse[ind+14] ^ sparse[ind+15];
    }

    dense
}

fn hexadecimal(hash: &Vec<u8>) -> String {
    let mut out = String::with_capacity(hash.len()*2);

    for b in hash.iter() {
        if *b < 16 {
            out.push('0');
        }
        out.push_str(&format!("{:x}", b));
    }

    out
}

