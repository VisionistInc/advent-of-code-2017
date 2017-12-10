use std::io::Result;

pub fn solve<L: Iterator<Item=Result<String>>>(input: &mut L) -> () {
    let mut checksum = 0;
    let mut evensum = 0;

    for line in input {
        let numbers: Vec<usize> = line
            .expect("Failed to get line.")
            .split(|c| c == ' ' || c == '\t')
            .map(|s| s.parse().expect("not a number?"))
            .collect();
        let max = numbers.iter().max().unwrap();
        let min = numbers.iter().min().unwrap();
        checksum += max - min;

        println!("Line diff: {}, rolling checksum: {}", max-min, checksum);

        let len = numbers.len();
        let mut diff = 0;

        'outer: for i in 0..len {
            for k in i..len {
                let (a, b) = (numbers[i], numbers[k]);
                if a > b && a % b == 0 {
                    diff = a/b;
                    break 'outer;
                } else if b > a && b % a == 0 {
                    diff = b/a;
                    break 'outer;
                }
            }
        }
        evensum += diff;

        println!("Line even diff: {}, rolling evensum: {}", diff, evensum);
    }
}
