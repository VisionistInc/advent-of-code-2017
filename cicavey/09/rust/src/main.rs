use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let file = File::open("../input.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    
    let mut iter = contents.chars();
    let mut group_level = 0;
    let mut group_count = 0;
    let mut score = 0;
    let mut gc = 0;
    while let Some(c) = iter.next() {
        if c == '{' {
            // group start
            group_level += 1;
            continue;
        }
        if c == '<' {
            // consume garabge!
            while let Some(g) = iter.next() {
                if g == '!' {
                    iter.next();
                    continue;
                }
                if g == '>' {
                    break;
                }
                gc += 1; // count actual garbage
            }
        }
        if c == '}' {
            score += group_level;
            group_count += 1;
            group_level -= 1;
        }
    }
    println!("{} {} {}", group_count, score, gc);
    
}