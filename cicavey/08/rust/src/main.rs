use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::collections::HashMap;

fn main() {
    let f = File::open("../input.txt").unwrap();
    let file = BufReader::new(&f);

    let mut regs = HashMap::<String, i64>::new();
    let mut peak: i64 = 0;

    for line in file.lines() {
        let mut tk_line: Vec<String> = line.unwrap().split_whitespace().map(|s| s.to_string()).collect();
        let cond_reg_value = *regs.entry(tk_line[4].to_string()).or_insert(0);
        let cond_value = tk_line[6].parse::<i64>().unwrap();
        let cond = match &tk_line[5] as &str {
            ">" => cond_reg_value > cond_value,
            ">=" => cond_reg_value >= cond_value,
            "<" => cond_reg_value < cond_value,
            "<=" => cond_reg_value <= cond_value,
            "==" => cond_reg_value == cond_value,
            "!=" => cond_reg_value != cond_value,
            _ => false
        };
        if !cond {
            continue
        }
        let op_value = tk_line[2].parse::<i64>().unwrap();
        if tk_line[1] == "inc" {
            *regs.entry(tk_line[0].to_string()).or_insert(0) += op_value;
        } else {
            *regs.entry(tk_line[0].to_string()).or_insert(0) -= op_value;
        }

        let lm = *regs.values().max().unwrap();
        if lm > peak {
            peak = lm;
        }
    }

    let m = regs.values().max().unwrap();
    println!("{} {}", m, peak);
}