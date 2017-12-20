use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::collections::HashMap;

extern crate regex;
use regex::Regex;

extern crate rayon;
use rayon::prelude::*;

#[derive(Debug)]
struct Particle {
    p: (i64, i64, i64),
    v: (i64, i64, i64),
    a: (i64, i64, i64),
}
/*
impl fmt::Display for Dancers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.members.join(""))
    }
}
*/
impl Particle {
    fn new(d: &Vec<i64>) -> Particle {
        Particle{p: (d[0],d[1],d[2]), v: (d[3],d[4],d[5]), a: (d[6],d[7],d[8])}
    }

    fn mdist(&self) -> u64 {
        (self.p.0.abs() as u64) + (self.p.1.abs() as u64) + (self.p.2.abs() as u64)
    }

    fn tick(&mut self) {
        self.v.0 += self.a.0;
        self.v.1 += self.a.1;
        self.v.2 += self.a.2;
        self.p.0 += self.v.0;
        self.p.1 += self.v.1;
        self.p.2 += self.v.2;
    }
}

fn main() {
    let f = File::open("../input.txt").unwrap();
    let file = BufReader::new(&f);
    let r = Regex::new(r"-?\d+").unwrap();

    let mut particles: Vec<Particle> = Vec::new();

    for line in file.lines() {
        let args: Vec<i64> = r.find_iter(&line.unwrap()).map(|m| m.as_str().parse().unwrap()).collect();
        particles.push(Particle::new(&args));
    }
    /*
    let mut min_i = 0;
    for _t in 0..100000 {
        particles.par_iter_mut().for_each(|p| p.tick());
        let mut min: u64 = particles[0].mdist();
        min_i = 0;
        for i in 1..particles.len() {
            let temp = particles[i].mdist();
            if temp < min {
                min = temp;
                min_i = i;
            }
        }
    }
    println!("MinIndex: {}", min_i);
    */
    let mut cmap: HashMap<(i64,i64,i64), u64> = HashMap::with_capacity(particles.len());
    for _t in 0..1000 {
        particles.par_iter_mut().for_each(|p| p.tick());

        // Calculate collisons
        cmap.clear();
        for p in &particles {
            *(cmap.entry(p.p).or_insert(0)) += 1;
        }
        for (key, val) in cmap.iter() {
            if *val == 1 as u64 {
                continue;
            }
            // purge any particles at this position
            particles.retain(|x| x.p != *key);
        }
    }
    println!("PL: {}", particles.len());
}