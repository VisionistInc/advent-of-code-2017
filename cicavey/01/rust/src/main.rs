use std::fs::File;
use std::io::prelude::*;

fn sum(numbers: &Vec<u32>, step: usize) -> u64 {
    let mut sum: u64 = 0;
    for (i, x) in numbers.iter().enumerate() {
        let next = numbers[(i+step) % numbers.len()];
        if *x == next {
            sum += *x as u64;
        }
    }
    return sum
}

fn main() {
    let mut f = File::open("../input.txt").unwrap();
    let mut input = String::new();
    f.read_to_string(&mut input);
    
    let numbers: Vec<u32> = input.trim().chars().map(|s| s.to_digit(10).unwrap()).collect();

    println!("Result 1: {}", sum(&numbers, 1));
    println!("Result 2: {}", sum(&numbers, numbers.len()/2));
}
