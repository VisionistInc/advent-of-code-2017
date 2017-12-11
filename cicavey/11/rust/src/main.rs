use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;

// extern crate modulo;
// use modulo::Mod;

// redblobgames to the rescue!

fn main() {
    let file = File::open("../input.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();

    let path: Vec<&str> = contents.split(",").collect();
    //let path: Vec<&str> = "se,sw,se,sw,sw".split(",").collect();

    // use cube representation of hex grid
    let mut pos: (i32,i32,i32) = (0,0,0);
    let mut max_dist = 0;
    for m in path {
        let op = match m {
            "n"  => ( 0, 1,-1),
            "ne" => ( 1, 0,-1),
            "nw" => (-1, 1, 0),
            "s"  => ( 0,-1, 1),
            "se" => ( 1,-1, 0),
            "sw" => (-1, 0, 1),
            _    => ( 0, 0, 0),
        };

        // There must be a cleaner way?
        pos.0 += op.0;
        pos.1 += op.1;
        pos.2 += op.2;

        let dist = (pos.0.abs() + pos.1.abs() + pos.2.abs())/2;
        if dist > max_dist {
            max_dist = dist;
        }
    }

    // manhattan distance in cubic coords, (abs(dx) + abs(dy) + abs(dz)) / 2 ... we're related to zero...
    println!("{:?} {} {}", pos, (pos.0.abs() + pos.1.abs() + pos.2.abs())/2, max_dist);

    
}