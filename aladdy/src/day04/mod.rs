use std::io::Result;
use std::collections::HashMap;

pub fn solve<L: Iterator<Item=Result<String>>>(input: &mut L) -> () {
    // allocating out here is unnecessary, but saves a number of allocations
    // each map will hold onto its memory even after being drained
    // and the drains ensure that no extra data makes its way between loops
    let mut anagrams = HashMap::new();
    let mut wordcount = HashMap::new();

    let mut valid_words = 0;
    let mut valid_anagrams = 0;
    for res in input {

        let line = res.expect("failed to get line");
        let ws: Vec<String> = line
            .split(' ')
            .map(|s| String::from(s))
            .collect();

        // part 1
        for w in ws.clone() {
            let count = wordcount.entry(w).or_insert(0);
            *count += 1;
        }
        if wordcount.drain().all(|(_, v)| v == 1) {
            valid_words += 1;
        }

        // part 2
        for w in ws {
            let mut chars = w.into_bytes(); // consumes w to create a Vec<u8>
            chars.sort_unstable(); // in place sort

            let count = anagrams.entry(chars).or_insert(0);
            *count += 1;
        }
        if anagrams.drain().all(|(_, v)| v == 1) {
            valid_anagrams += 1;
        }

    }

    println!("Valid passphrases w/out matching words: {}", valid_words);
    println!("Valid passphrases w/out matching anagrams: {}", valid_anagrams);
}
