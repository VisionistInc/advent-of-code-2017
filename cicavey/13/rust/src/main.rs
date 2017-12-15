use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

fn main() {
    let f = File::open("../input.txt").unwrap();
    let file = BufReader::new(&f);

    let mut fw: Vec<(u32, u32)> = Vec::new();

    for line in file.lines() {
        let sl: Vec<u32> = line.unwrap().split(": ").map(|s| s.parse().unwrap()).collect();
        fw.push((sl[0], sl[1]));
    }

    let part1: u32 = fw
        .iter()
        .filter(|&&(depth, range)| depth % ((range - 1) * 2) == 0)
        .map(|&(depth, range)| depth * range)
        .sum();

    let part2 = (0..10_000_000)
        .find(|wait| {
            fw.iter().all(|&(depth, range)| {
                (depth + wait) % ((range - 1) * 2) != 0
            })
        })
        .unwrap();

    println!("{} {}", part1, part2);
}