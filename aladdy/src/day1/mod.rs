use std::io::Result;

pub fn solve<L: Iterator<Item=Result<String>>>(input: &mut L) -> () {
    let line = input.next().unwrap().unwrap();
    let bytes = line.into_bytes();

    if bytes.len() < 2 {
        println!("The input is too short.");
        return;
    }

    let mut sum1: u64 = 0;
    let mut sum2: u64 = 0;

    let len = bytes.len();
    for i in 0..bytes.len() {
        if bytes[i] == bytes[(i+1)%len] {
            sum1 += (bytes[i] as u64)-48; // 48 == '0'
        }
        if bytes[i] == bytes[(i+len/2)%len] {
            sum2 += (bytes[i] as u64)-48; // 48 == '0'
        }
    }

    println!("The captcha is {}", sum1);
    println!("The secondary captcha is {}", sum2);
}
