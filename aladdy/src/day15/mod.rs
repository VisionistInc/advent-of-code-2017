use std::io::Result;

const FACTOR_A: u64 = 16807;
const FACTOR_B: u64 = 48271;

const GEN_INIT_A: u64 = 679;
const GEN_INIT_B: u64 = 771;

const MAX: u64 = 2147483647;

pub fn solve<L: Iterator<Item=Result<String>>>(_input: &mut L) -> () {
    let mut matches = 0;
    let mut a = GEN_INIT_A;
    let mut b = GEN_INIT_B;
    for _ in 0..40_000_000 {
        a = (a * FACTOR_A) % MAX;
        b = (b * FACTOR_B) % MAX;
        if (a & 0xFFFF) ^ (b & 0xFFFF) == 0 {
            matches += 1;
        }
    }
    println!("Lower 16 bit matches: {}", matches);

    matches = 0;
    a = GEN_INIT_A;
    b = GEN_INIT_B;
    for _ in 0..5_000_000 {
        a = (a * FACTOR_A) % MAX;
        while a % 4 != 0 { a = (a * FACTOR_A) % MAX; }
        b = (b * FACTOR_B) % MAX;
        while b % 8 != 0 { b = (b * FACTOR_B) % MAX; }
        if (a & 0xFFFF) ^ (b & 0xFFFF) == 0 {
            matches += 1;
        }
    }
    println!("'Smarter' Lower 16 bit matches: {}", matches);
}
