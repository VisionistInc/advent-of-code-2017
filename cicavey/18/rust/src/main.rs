use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;
use std::str;
use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug)]
struct Program {
    regs: HashMap<String, i64>,
    pc: usize,
    ic: u64,
    send_buffer: VecDeque<i64>,
    send_count: u64,
    blocked: bool,
}

impl Program {
    pub fn new() -> Self {
        Program {
            regs: HashMap::with_capacity(27),
            pc: 0,
            ic: 0,
            send_buffer: VecDeque::new(),
            send_count: 0,
            blocked: false
        }
    }
    pub fn new_id(id: i64) -> Self {
        let mut p = Self::new();
        p.regs.insert("p".to_string(), id);
        return p;
    }

    pub fn lookup(&mut self, reg_or_value: &str) -> i64 {
        match reg_or_value.parse() {
            Ok(n) => n,
            Err(_) => { self.regs.entry(reg_or_value.to_string()).or_insert(0).clone() },
        }
    }
}

fn main() {
    let file = File::open("../input.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    
    let code: Vec<String> = contents.lines().map(|s| s.to_string()).collect();

    /*
    let mut p = Program::new();
    let mut first_rcv = true;
    while p.pc < code.len() && p.ic < 10000 {
        //println!("IC: {:09} PC: {:04} {}", p.ic, p.pc, code[p.pc]);
        let next_instruction = &code[p.pc];
        let sins: Vec<String> = next_instruction.split_whitespace().map(|s| s.to_string()).collect();
        let mut step_pc = true;
        match sins[0].as_str() {
            "snd" => {
                let v = p.lookup(&sins[1]);
                p.regs.insert("snd".to_string(), v);
            },
            "set" => {
                let v = p.lookup(&sins[2]);
                p.regs.insert(sins[1].to_string(), v);
            },
            "add" => {
                let v = p.lookup(&sins[2]);
                let t = p.lookup(&sins[1]);
                p.regs.insert(sins[1].to_string(), v+t);
            },
            "mul" => {
                let v = p.lookup(&sins[2]);
                let t = p.lookup(&sins[1]);
                p.regs.insert(sins[1].to_string(), v*t);
            },
            "mod" => {
                let v = p.lookup(&sins[2]);
                let t = p.lookup(&sins[1]);
                p.regs.insert(sins[1].to_string(), t % v);
            },
            "rcv" => {
                let v = p.lookup(&sins[1]);
                if v != 0 {
                    let s = p.lookup("snd");
                    p.regs.insert("rcv".to_string(), s);
                    if first_rcv {
                        println!("FIRST_RCV: {}", s);
                        first_rcv = false;
                        p.pc = code.len();
                    }
                }
            },
            "jgz" => {
                let offset = p.lookup(&sins[2]);
                let test = p.lookup(&sins[1]);
                if test > 0 {
                    step_pc = false;
                    p.pc = (p.pc as i64 + offset) as usize;
                }
            },
            _ => (),
        };

        if step_pc {
            p.pc += 1;
        }

        p.ic += 1;

        //println!("\tPC:{} {:?}", p.pc, p.regs);
    }
    */

    let mut limit = 0;
    let mut programs = [Program::new_id(0), Program::new_id(1)];
    loop {
        // alternate between the two programs
        for pidx in 0..2 {
            let next_instruction = &code[programs[pidx].pc];
            let sins: Vec<String> = next_instruction.split_whitespace().map(|s| s.to_string()).collect();
            let mut step_pc = true;
            match sins[0].as_str() {
                "snd" => {
                    let v = programs[pidx].lookup(&sins[1]);
                    //programs[pidx].regs.insert("snd".to_string(), v);
                    programs[(pidx + 1) % 2].send_buffer.push_back(v);
                    //println!("send: {} +1 {}", pidx, programs[pidx].send_count);
                    programs[pidx].send_count += 1;
                },
                "set" => {
                    let v = programs[pidx].lookup(&sins[2]);
                    programs[pidx].regs.insert(sins[1].to_string(), v);
                },
                "add" => {
                    let v = programs[pidx].lookup(&sins[2]);
                    let t = programs[pidx].lookup(&sins[1]);
                    programs[pidx].regs.insert(sins[1].to_string(), v+t);
                },
                "mul" => {
                    let v = programs[pidx].lookup(&sins[2]);
                    let t = programs[pidx].lookup(&sins[1]);
                    programs[pidx].regs.insert(sins[1].to_string(), v*t);
                },
                "mod" => {
                    let v = programs[pidx].lookup(&sins[2]);
                    let t = programs[pidx].lookup(&sins[1]);
                    programs[pidx].regs.insert(sins[1].to_string(), t % v);
                },
                "rcv" => {
                    step_pc = false;
                    if let Some(v) = programs[pidx].send_buffer.pop_front() {
                        step_pc = true;
                        programs[pidx].regs.insert(sins[1].to_string(), v);
                        programs[pidx].blocked = false;
                    } else {
                        programs[pidx].blocked = true;
                    }
                },
                "jgz" => {
                    let offset = programs[pidx].lookup(&sins[2]);
                    let test = programs[pidx].lookup(&sins[1]);
                    if test > 0 {
                        step_pc = false;
                        programs[pidx].pc = (programs[pidx].pc as i64 + offset) as usize;
                    }
                },
                _ => (),
            };

            if step_pc {
                programs[pidx].pc += 1;
            }

            programs[pidx].ic += 1;
        }

        if programs[0].blocked && programs[1].blocked {
            println!("{} deadlock: {} {}", limit, programs[0].send_count, programs[1].send_count);
            println!("0> IC:{:09} PC:{:04} R:{:?} SB:{:?}", programs[0].ic, programs[0].pc, programs[0].regs, programs[0].send_buffer);
            println!("1> IC:{:09} PC:{:04} R:{:?} SB:{:?}", programs[1].ic, programs[1].pc, programs[1].regs, programs[1].send_buffer);
            break;
        }

        limit += 1;
        if limit > 1000000 {
            println!("limit reached");
            break;
        }
    }
}
