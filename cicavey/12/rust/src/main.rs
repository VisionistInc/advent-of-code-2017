use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let f = File::open("../input.txt").unwrap();
    let file = BufReader::new(&f);

    let mut nlist = HashMap::new();

    for line in file.lines() {
        let sl: Vec<String> = line.unwrap().split(" <-> ").map(|s| s.to_string()).collect();
        let id: u32 = sl[0].parse().unwrap();
        let neighbors: Vec<u32> = sl[1].split(",").map(|s| s.trim().parse().unwrap()).collect();
        nlist.insert(id, neighbors);
    }

    let mut groups = 1;
    let mut visited: HashSet<u32> = HashSet::new();
    let mut pending: Vec<u32> = Vec::new();
    
    visited.insert(0u32);
    pending.append(&mut nlist.get(&0u32).unwrap().clone());

    while !pending.is_empty() {
        let next = pending.pop().unwrap();
        if visited.contains(&next) {
            continue;
        }
        visited.insert(next);
        pending.append(&mut nlist.get(&next).unwrap().clone());
    }
    println!("{}", visited.len());

    // Count groups by traversing from each original node
    // If the node was already visited, it's alrleady int group
    // then follow the same alg as node 0
    // NOTE: This should 100% be a function ... but Rust.
    for id in nlist.keys() {
        if visited.contains(id) {
            continue
        }

        let mut pending: Vec<u32> = Vec::new();
        visited.insert(*id);
        pending.append(&mut nlist.get(id).unwrap().clone());
        while !pending.is_empty() {
            let next = pending.pop().unwrap();
            if visited.contains(&next) {
                continue;
            }
            visited.insert(next);
            pending.append(&mut nlist.get(&next).unwrap().clone());
        }

        groups += 1;
    }

     println!("{}", groups);
}