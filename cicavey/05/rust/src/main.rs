use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

fn main() {
    let f = File::open("../input.txt").unwrap();
    let file = BufReader::new(&f);

    let mut numbers: Vec<i32> = file.lines().map(|line| line.unwrap()).map(|s| s.parse().unwrap()).collect();

    let mut i: i32 = 0;
    let mut steps = 0;

    while i >=0 && i < numbers.len() as i32 {
        //println!("{}", numbers[i as usize]);
        // Lookup value
        let target = numbers[i as usize];
        // Increment previous
        //numbers[i as usize] += 1
        numbers[i as usize] += if target >= 3 { -1 } else {1};
        // Jump
        i += target;
        steps += 1;
    }

    println!("{}", steps)
}
