use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
                
fn main() {   
    let f = File::open("input").unwrap();
    let file = BufReader::new(&f);
    let mut sol1 = 0;
    let mut sol2 = 0;
    for line in file.lines() {
        let l = line.unwrap();
        if l.len() == 0 {
        	continue;
        }
        let numbers: Vec<i32> = l.split_whitespace()
        			             .map(|s| s.parse().unwrap())
        			             .collect();
        let length = numbers.len();
        for x in 0..length {
        	for y in (x+1)..length {
        		if numbers[x] % numbers[y] == 0 {
        			sol2 = sol2 + (numbers[x]/numbers[y]);
        			break;
        		}
        		if numbers[y] % numbers[x] == 0 {
        			sol2 = sol2 + (numbers[y]/numbers[x]);
        			break;
        		}
        	}
        }
        let mut low = numbers[0];
        let mut hi = low;
        for num in numbers {
        	if num < low {
        		low = num;
        	}
        	if num > hi {
        		hi = num;
        	}
        }
        sol1 = sol1 + hi - low;
    }
    println!("{}", sol1);
    println!("{}", sol2);
}