use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::collections::HashMap;

fn main() {
    let f = File::open("../input.txt").unwrap();
    let file = BufReader::new(&f);

    let mut total = 0;
    let mut valid = 0;

    for line in file.lines() {
        let why = line.unwrap();
        let iter = why.split_whitespace();
        let mut counts = HashMap::new();

        let mut valid_flag = true;

        for word in iter {
            *(counts.entry(word).or_insert(0)) += 1;
            if *counts.get(word).unwrap() > 1 {
                valid_flag = false;
                break;
            }
        }

        let mut anamap = HashMap::new();
        for key in counts.keys() {
            // Covert each key into a char sorted version
            let mut chars: Vec<char> = key.chars().collect();
            chars.sort();
            let s: String = chars.into_iter().collect();

            *(anamap.entry(s).or_insert(0)) += 1;
            // if *anamap.get(s).unwrap() > 1 {
            //     valid_flag = false;
            //     break;
            // }
        }
        
        for (k,v) in anamap {
            if v > 1 {
                valid_flag = false;
                break;
            }
        }

        total += 1;
        if valid_flag {
            valid += 1;
        }
    }

    println!("Valid: {}/{}", valid, total);
}

// fn word_fingerprint(word: String) -> String {
//     let mut chars: Vec<char> = word.chars().collect();
//     return String::from("ASS")
// }