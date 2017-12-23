use std::io::Result;
use std::convert::From;
use std::collections::HashMap;
use std::collections::vec_deque::VecDeque;

pub fn solve<L: Iterator<Item=Result<String>>>(input: &mut L) -> () {
    let mut instrs = Vec::with_capacity(input.size_hint().0);
    let mut regs: HashMap<char, i64> = "abcdefghijklmnopqrstuvwxyz"
        .chars().map(|c| (c, 0)).collect();
    for res in input {
        instrs.push(Code::from(&*res.unwrap()));
    }

    let recovered = exec_sound(&instrs, &mut regs);
    println!("First sound recovered: {}", recovered);
    let sent = exec_duet(&instrs);
    println!("Program one sent {} values", sent);
}

// executes instructions for sound player, returning the first recovered sound
fn exec_sound(instrs: &Vec<Code>, regs: &mut HashMap<char, i64>) -> i64 {
    let mut ip = 0;
    let mut sound = 0;

    loop {
        match instrs[ip] {
            Code::SND(r) => sound = regs[&r],

            Code::SET(r, y) => {
                let v = get(&regs, y);
                let e = regs.entry(r).or_insert(0);
                *e = v;
            },

            Code::ADD(r, y) => {
                let v = get(&regs, y);
                let e = regs.entry(r).or_insert(0);
                *e += v;
            },

            Code::MUL(r, y) => {
                let v = get(&regs, y);
                let e = regs.entry(r).or_insert(0);
                *e *= v;
            },

            Code::MOD(r, y) => {
                let v = get(&regs, y);
                let e = regs.entry(r).or_insert(0);
                *e %= v;
            },

            Code::RCV(_) => if sound != 0 {
                return sound;
            },

            Code::JGZ(r, y) => {
                if regs[&r] > 0 {
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

    unreachable!();
}

fn exec_duet(instrs: &Vec<Code>) -> i64 {
    let mut regs0: HashMap<char, i64> = "abcdefghijklmnopqrstuvwxyz"
        .chars().map(|c| (c, 0)).collect();
    let mut regs1 = regs0.clone();
    regs0.insert('p', 0);
    regs1.insert('p', 1);
    let mut queue0 = VecDeque::new();
    let mut queue1 = VecDeque::new();
    let mut ip0 = 0;
    let mut ip1 = 0;

    let mut sent = 0;
    let mut cur_prog = 0;

    let mut stat0 = Status::Init;
    let mut stat1 = Status::Init;

    loop {
        if cur_prog == 0 {
            let (stop, _)= exec_until_blocked(&instrs, ip0, &mut regs0, &mut queue0, &mut queue1);
            stat0 = stop;

            if let Status::Blocked(sp) = stop {
                ip0 = sp;
            }
            cur_prog = 1;
        } else {
            let (stop, n) = exec_until_blocked(&instrs, ip1, &mut regs1, &mut queue1, &mut queue0);
            stat1 = stop;
            sent += n;

            if let Status::Blocked(sp) = stop {
                ip1 = sp;
            }
            cur_prog = 0;
        }

        if queue0.len() == 0 && queue1.len() == 0 {
            // deadlocked
            break;
        }
        if (stat0.ended() && stat1.ended()) ||
           (stat0.ended() && stat1.blocked()) ||
           (stat0.blocked() && stat1.ended()) {
            // blocked program can never be satisfied
            break;
        }
    }

    sent
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Status {
    Init,
    Blocked(usize),
    Ended,
}

impl Status {
    fn blocked(&self) -> bool {
        match *self {
            Status::Init | Status::Ended => false,
            Status::Blocked(_) => true,
        }
    }

    fn ended(&self) -> bool {
        *self == Status::Ended
    }
}

// run core until blocked or ended,
// returns stop reason and number items sent
#[allow(non_snake_case)]
fn exec_until_blocked(instrs: &Vec<Code>, ip: usize, regs: &mut HashMap<char, i64>, 
                      IN: &mut VecDeque<i64>, OUT: &mut VecDeque<i64>) -> (Status, i64) {
    let mut ip = ip;
    let mut sent = 0;
    loop {
        match instrs[ip] {
            Code::SND(r) => {
                OUT.push_back(regs[&r]);
                sent += 1;
            },

            Code::SET(r, y) => {
                let v = get(&regs, y);
                let e = regs.entry(r).or_insert(0);
                *e = v;
            },

            Code::ADD(r, y) => {
                let v = get(&regs, y);
                let e = regs.entry(r).or_insert(0);
                *e += v;
            },

            Code::MUL(r, y) => {
                let v = get(&regs, y);
                let e = regs.entry(r).or_insert(0);
                *e *= v;
            },

            Code::MOD(r, y) => {
                let v = get(&regs, y);
                let e = regs.entry(r).or_insert(0);
                *e %= v;
            },

            Code::RCV(r) => {
                if IN.len() == 0 {
                    return (Status::Blocked(ip), sent);
                }
                regs.insert(r, IN.pop_front().unwrap());
            },

            Code::JGZ(r, y) => {
                let test = if r.is_digit(10) {
                    r.to_digit(10).unwrap() as i64
                } else {
                    regs[&r]
                };
                if test > 0 {
                    let jmp = get(regs, y);
                    if jmp == 0 {
                        panic!("jmp of zero: {:?}", instrs[ip]);
                    } else if jmp > 0 {
                        ip += jmp as usize;
                        if ip >= instrs.len() {
                            return (Status::Ended, sent);
                        }
                    } else {
                        let j = jmp.abs() as usize;
                        if j > ip {
                            return (Status::Ended, sent);
                        }
                        ip -= j;
                    }
                    continue; // skip ip += 1
                }
            },
        }

        ip += 1;
    }
}

fn get(regs: &HashMap<char, i64>, ri: RegORImm) -> i64 {
    match ri {
        RegORImm::Reg(r) => regs[&r],
        RegORImm::Imm(i) => i,
    }
}

#[derive(Debug)]
enum Code {
    SND(char),
    SET(char, RegORImm),
    ADD(char, RegORImm),
    MUL(char, RegORImm),
    MOD(char, RegORImm),
    RCV(char),
    JGZ(char, RegORImm),
}

// Register or Immediate
#[derive(Debug, Clone, Copy)]
enum RegORImm {
    Reg(char),
    Imm(i64),
}

impl<'a> From<&'a str> for Code {
    fn from(s: &str) -> Self {
        use day18::RegORImm::{Reg, Imm};
        use day18::Code::*;
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
            "snd" => SND(x),
            "set" => SET(x, y.unwrap()),
            "add" => ADD(x, y.unwrap()),
            "mul" => MUL(x, y.unwrap()),
            "mod" => MOD(x, y.unwrap()),
            "rcv" => RCV(x),
            "jgz" => JGZ(x, y.unwrap()),
            _ => panic!("{} is not a valid code", code),
        }
    }
}

