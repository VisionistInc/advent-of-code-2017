use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

fn main() {
    let f = File::open("../input.txt").unwrap();
    let file = BufReader::new(&f);

    let mut sum1: u64 = 0;
    let mut sum2: u64 = 0;
    for line in file.lines() {

        let numbers: Vec<u32> = line.unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();

        let mut min: u32 = numbers[0];
        let mut max: u32 = min;

        // There must be some fancy library method for this. Or at least a vector stream function.
        for x in numbers.iter().skip(1) {
            max = std::cmp::max(max, *x);
            min = std::cmp::min(min, *x);
        }

        sum1 += (max - min) as u64;

        for i in 0..numbers.len() {
            for j in i+1..numbers.len() {
                let ni = numbers[i];
                let nj = numbers[j];
                let dividend = std::cmp::max(ni, nj);
                let divider = std::cmp::min(ni, nj);
                if  dividend % divider == 0 {
                    sum2 += (dividend / divider) as u64
                }
            }
        }
    }
    println!("part 1: {}", sum1);
    println!("part 2: {}", sum2);
}