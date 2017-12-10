use std::io::Result;
use std::collections::HashMap;

pub fn solve<L: Iterator<Item=Result<String>>>(input: &mut L) -> () {
    let mut registers: HashMap<String, i32> = HashMap::new();

    let mut max = i32::min_value();
    for res in input {
        let line = res.unwrap();
        let toks: Vec<&str> = line.split_whitespace().collect();
        let target = toks[0];
        let op = toks[1];
        let op_amt: i32 = toks[2].parse().unwrap();
        // tok[3] == if
        let chk_reg = toks[4];
        let cmp = toks[5];
        let cmp_amt: i32 = toks[6].parse().unwrap();

        let pass = {
            let v = registers.entry(String::from(chk_reg)).or_insert(0);
            match cmp {
                ">" => *v > cmp_amt,
                ">=" => *v >= cmp_amt,
                "<" => *v < cmp_amt,
                "<=" => *v <= cmp_amt,
                "==" => *v == cmp_amt,
                "!=" => *v != cmp_amt,
                _ => panic!("uknown operator"),
            }
        };

        if pass {
            let v = registers.entry(String::from(target)).or_insert(0);
            match op {
                "inc" => *v += op_amt,
                "dec" => *v -= op_amt,
                _ => panic!("unknown operator"),
            }
            if *v > max {
                max = *v;
            }
        }
    }

    println!("Max register value is: {}", registers.values().max().unwrap());
    println!("All time max value is: {}", max);
}
