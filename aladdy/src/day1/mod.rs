use std::io::Result;

pub fn solve<L: Iterator<Item=Result<String>>>(input: &mut L) -> () {
    let line = input.next().unwrap().unwrap();
    let bytes = line.into_bytes();

    let mut sum: u64 = 0;
    if bytes.len() < 2 {
        println!("The captcha is {}", sum);
        return;
    }

    for i in 0..bytes.len() {
        if bytes[i] == bytes[(i+1)%bytes.len()] {
            sum += (bytes[i] as u64)-48;
        }
    }

    println!("The captcha is {}", sum);
}
