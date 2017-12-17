#![feature(slice_rotate)]

use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;
use std::str;
use std::fmt;
use std::collections::HashMap;


#[derive(Debug)]
struct Dancers {
    members: Vec<String>
}

impl fmt::Display for Dancers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.members.join(""))
    }
}

impl Dancers {

    pub fn new() -> Dancers {

        let d: Vec<String> = "abcdefghijklmnop".chars().map(|c| c.to_string()).collect();
        return Dancers { members: d};
    }

    fn perform_instruction(&mut self, ins: &str) {
         match &ins[0..1] {
            "s" => self.spin(&ins[1..]),
            "x" => self.exchange(&ins[1..]),
            "p" => self.partner(&ins[1..]),
            _ => (),
        }
    }

    fn spin(&mut self, args: &str) {
        let mut a: usize = args.parse().unwrap();
        let l = self.members.len();
        a = a % l;
        self.members.rotate(l-a);
    }

    fn exchange(&mut self, args: &str) {
        let x: Vec<usize> = args.split('/').map(|p| p.parse().unwrap()).collect();
        self.members.swap(x[0], x[1]);
    }

    fn partner(&mut self, args: &str) {
        let p: Vec<&str> = args.split('/').collect();
        let a = self.members.iter().position(|e| e == p[0]).unwrap();
        let b = self.members.iter().position(|e| e == p[1]).unwrap();
        self.members.swap(a, b);
    }

    fn jump(&mut self, args: &str) {
         let d: Vec<String> = args.chars().map(|c| c.to_string()).collect();
         self.members = d;
    }
}

fn main() {
    let file = File::open("../input.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();

    let mut opt_map: HashMap<String, String> = HashMap::new();

    let mut d = Dancers::new();

    let pre = d.to_string();
    for ins in contents.split(',') {
        d.perform_instruction(ins);
    }
    println!("{}", d);

    opt_map.insert(pre, d.to_string());

    // Every multiple of 3000 == abc....p
    // so we really only need to do 1000 iterations starting from beginning
    // 1 includes first
    // hashmap for optimization
    for i in 1..1000 {
        if opt_map.contains_key(&d.to_string()) {
            let targ = opt_map.get(&d.to_string()).unwrap();
            d.jump(targ);
            continue;
        }

        let pre = d.to_string();
        for ins in contents.split(',') {
            d.perform_instruction(ins);
        }
        opt_map.insert(pre, d.to_string());
    }

    println!("{}", d);
}