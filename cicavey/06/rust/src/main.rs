use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::collections::HashSet;

fn first_max(v: &Vec<u8>) -> Option<usize> {
    let max =  v.iter().max()?;
    return v.iter().position(|x| x == max);
}

fn redistribute(v: &mut Vec<u8>) {
    let mut idx = first_max(&v).unwrap();
    let q = v[idx];
    v[idx] = 0;
    let l = v.len();
    for _ in 0..q {
        idx = (idx + 1) % l;
        v[idx] = v[idx] + 1;
    }
}

fn main() {
    let f = File::open("../input.txt").unwrap();
    let mut file = BufReader::new(&f);
    
    let mut line = String::new();
    file.read_line(&mut line);
    let mut blocks: Vec<u8> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();

    let mut state = HashSet::new();

    // Find first max
    // Redist
    let mut steps = 0;
    while state.insert(blocks.clone()) {
        redistribute(&mut blocks);
        steps = steps + 1;
    }

    // Loop until we find this state again
    let save_state = blocks.clone();
    let mut cycle_steps = 0;
    loop {
        redistribute(&mut blocks);
        cycle_steps = cycle_steps + 1;
        if blocks == save_state {
            break;
        }
    }

    println!("{} {}", steps, cycle_steps);
}
