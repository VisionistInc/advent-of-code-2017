use std::io::Result;

pub fn solve<L: Iterator<Item=Result<String>>>(_input: &mut L) -> () {
    //let key = "hfdlxzhv";
    let key = "flqrgnkx";
    let mut disk: Vec<Vec<u8>> = Vec::with_capacity(128);

    for i in 0..128 {
        let hash = knot_hash(&format!("{}-{}", key, i));
        disk.push(hash);
    }

    let mut used = 0;
    for row in disk.iter() {
        for n in row.iter() {
            used += n.count_ones();
        }
    }
    println!("Used squares {}", used);
    println!("Total groups {}", groups(&disk));
}

fn groups(disk: &Vec<Vec<u8>>) -> usize {
    assert_eq!(disk.len(), 128);
    assert_eq!(disk[0].len(), 16);
    let mut scratch = disk.clone();

    let mut count = 0;
    loop {
        let mut stack = Vec::new();

        // find group
        'outer: for x in 0..128 {
            for y in 0..128 {
                if ind(&scratch, x, y) {
                    stack.push((x,y));
                    break 'outer;
                }
            }
        }
        if stack.len() == 0 {
            // no more groups
            break;
        }
        count += 1;

        // remove group
        loop {
            let (x,y) = match stack.pop() {
                Some((x,y)) => (x,y),
                None => break,
            };
            // NOTE we could end up searching the same spots multiple times,
            // but it shouldn't cause any problems
            set(&mut scratch, x, y, false);
            // up
            if x > 0 && ind(&scratch, x-1, y) {
                stack.push((x-1,y));
            }
            // left
            if y > 0 && ind(&scratch, x, y-1) {
                stack.push((x,y-1));
            }
            // down
            if x < 127 && ind(&scratch, x+1, y) {
                stack.push((x+1,y));
            }
            // right
            if y < 127 && ind(&scratch, x, y+1) {
                stack.push((x,y+1));
            }
        }
    }
    assert_ne!(&scratch, disk);
    println!("{:?}", scratch);

    count
}

fn ind(d: &Vec<Vec<u8>>, x: usize, y: usize) -> bool {
    d[x][y/8]&(1<<(y%8)) == 1
}

fn set(d: &mut Vec<Vec<u8>>, x: usize, y: usize, v: bool) {
    let n = d[x][y/8];
    d[x][y/8] = if v {
        n | (1 << (y%8))
    } else {
        n & !(1 << (y%8))
    };
}

#[cfg(test)]
mod test {
    #[test]
    fn test_set() {
        let a = 0xF0;
        assert_eq!(0xF2, a | (1 << 1));
        assert_eq!(0xD0, a & !(1 << 5));
    }
}

////////////////////////
//// Knot hash code ////
////////////////////////

fn knot_hash(input: &str) -> Vec<u8> {
    let length_ext = vec![17u8, 31, 73, 47, 23];
    let lengths = {
        let mut lengths = Vec::new();
        lengths.extend_from_slice(&input.as_bytes());
        lengths.append(&mut length_ext.clone());
        lengths
    };
    let mut list: Vec<u8> = (0..255).collect();
    list.push(255); // 0..256 overflows the u8 back to 0, resulting in an empty list

    let mut skip = 0u32;
    let mut ind = 0u8;
    for _ in 0..64 {
        let (i, s) = run_round(&mut list, ind, skip, &lengths);
        ind = i;
        skip = s;
    }

    reduce_hash(&list)
}

// returns (index, skip)
fn run_round(list: &mut Vec<u8>, cur_pos: u8, skip: u32, lengths: &Vec<u8>) -> (u8, u32) {
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

