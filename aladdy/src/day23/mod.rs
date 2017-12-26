use std::io::Result;
use std::convert::From;
use std::collections::HashMap;

pub fn solve<L: Iterator<Item=Result<String>>>(input: &mut L) -> () {
    let mut instrs = Vec::with_capacity(input.size_hint().0);
    let mut regs: HashMap<char, i64> = "abcdefgh"
        .chars().map(|c| (c, 0)).collect();
    for res in input {
        instrs.push(Code::from(&*res.unwrap()));
    }

    let mut regs2 = regs.clone();

    let mult = exec_mult(&instrs, &mut regs);
    println!("Executed mul {} times.", mult);

    regs2.insert('a', 1); // turn off debug
    let _ = exec_mult(&instrs, &mut regs2);
    println!("Register H is {}", regs2[&'h']);
}

// executes instructions counting how many times MUL is called
fn exec_mult(instrs: &Vec<Code>, regs: &mut HashMap<char, i64>) -> i64 {
    let mut ip = 0;
    let mut mult = 0;

    loop {
        match instrs[ip] {
            Code::SET(r, y) => {
                let v = get(&regs, y);
                let e = regs.entry(r).or_insert(0);
                *e = v;
            },

            Code::SUB(r, y) => {
                let v = get(&regs, y);
                let e = regs.entry(r).or_insert(0);
                *e -= v;
            },

            Code::MUL(r, y) => {
                let v = get(&regs, y);
                let e = regs.entry(r).or_insert(0);
                mult += 1;
                *e *= v;
            },

            Code::JNZ(r, y) => {
                let test = if r.is_digit(10) {
                    r.to_digit(10).unwrap() as i64
                } else {
                    regs[&r]
                };
                if test != 0 {
                    let jmp = get(regs, y);
                    if jmp == 0 {
                        panic!("jmp of zero: {:?}", instrs[ip]);
                    } else if jmp > 0 {
                        ip += jmp as usize;
                        if ip >= instrs.len() {
                            break;
                        }
                    } else {
                        let j = jmp.abs() as usize;
                        if j > ip {
                            // this jump would go outside valid space
                            break;
                        }
                        ip -= j;
                    }
                    continue; // skip ip += 1
                }
            },
        }

        ip += 1;
    }

    mult
}

fn get(regs: &HashMap<char, i64>, ri: RegORImm) -> i64 {
    match ri {
        RegORImm::Reg(r) => regs[&r],
        RegORImm::Imm(i) => i,
    }
}

#[derive(Debug)]
enum Code {
    SET(char, RegORImm),
    SUB(char, RegORImm),
    MUL(char, RegORImm),
    JNZ(char, RegORImm),
}

// Register or Immediate
#[derive(Debug, Clone, Copy)]
enum RegORImm {
    Reg(char),
    Imm(i64),
}

impl<'a> From<&'a str> for Code {
    fn from(s: &str) -> Self {
        use day23::RegORImm::{Reg, Imm};
        use day23::Code::*;
        use std::str;

        let s: Vec<u8> = s.bytes().collect();

        let code = str::from_utf8(&s[0..3]).unwrap();
        let reg = s[4];
        let y = if s.len() > 5 {
            let parsed = str::from_utf8(&s[6..s.len()]).unwrap().parse();
            if let Ok(imm) = parsed {
                Some(Imm(imm))
            } else {
                Some(Reg(char::from(s[6])))
            }
        } else {
            None
        };

        
        let x = char::from(reg);
        match code {
            "set" => SET(x, y.unwrap()),
            "sub" => SUB(x, y.unwrap()),
            "mul" => MUL(x, y.unwrap()),
            "jnz" => JNZ(x, y.unwrap()),
            _ => panic!("{} is not a valid code", code),
        }
    }
}

// initial static analysis of program
//fn part_two() -> i64 {
//    let mut a = 0;
//    let mut b = 93;
//    let mut c = 93;
//    let mut d = 0;
//    let mut e = 0;
//    let mut f = 0;
//    let mut g = 0;
//    let mut h = 0;
//
//    a = 1; // turn off debug mode
//    if a != 0 {
//        b = (b*100) + 100000;
//        c = b + 17000;
//    }
//    loop { // 1000 times
//        f = 1;
//        d = 2;
//        loop {
//            e = 2;
//            loop {
//                g = (d*e)-b;
//                if g == 0 {
//                    f = 0;
//                }
//                e += 1;
//                g = e-b;
//                if g == 0 {
//                    break;
//                }
//            }
//            d += 1;
//            g = d-b;
//            if g == 0 {
//                break;
//            }
//        }
//        if f == 0 {
//            h += 1;
//        }
//        g = b - c;
//        if g == 0 {
//            return h;
//        }
//        b += 17;
//    }
//}
