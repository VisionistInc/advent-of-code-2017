use std::io::Result;
use std::collections::HashMap;

pub fn solve<L: Iterator<Item=Result<String>>>(input: &mut L) -> () {
    let mut mem: Vec<u8> = input.next().unwrap().unwrap()
        .split(|c| c == '\t' || c == ' ')
        .map(|s| s.parse().unwrap())
        .collect();
    let mut states = HashMap::new();

    let mut step = 0;
    states.insert(mem.clone(), 0);
    loop {
        step += 1;
        balance(&mut mem);

        if states.contains_key(&mem) {
            break;
        }
        states.insert(mem.clone(), step);
    }

    println!("Found loop on step {}", step);
    println!("Loop is {} steps long", step - states.get(&mem).unwrap());
}

fn balance(mem: &mut Vec<u8>) {
    let mut max_i = 0;
    let mut max = 0;
    for i in 0..mem.len() {
        if mem[i] > max {
            max = mem[i];
            max_i = i;
        }
    }

    let mut i = max_i;
    mem[max_i] = 0;

    for _ in 0..max {
        i = (i + 1) % mem.len();
        mem[i] += 1;
    }
}
