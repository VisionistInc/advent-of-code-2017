use std::io::Result;
use std::collections::{HashMap, HashSet};

pub fn solve<L: Iterator<Item=Result<String>>>(input: &mut L) -> () {
    let mut conns = HashMap::new();
    for res in input {
        let line = res.unwrap();
        let toks: Vec<&str> = line.split_whitespace().collect();
        let pid: u32 = toks[0].parse().unwrap();
        // toks[1] == '<->'
        
        let e = conns.entry(pid).or_insert(Vec::new());
        for i in 2..toks.len() {
            let id = if i == toks.len()-1 {
                toks[i]
            } else {
                // has trailing comma
                let t = toks[i];
                &t[0..t.len()-1]
            }.parse().unwrap();
            (*e).push(id);
        }
    }

    let gr0 = find_group(0, &conns);
    println!("Group with pid 0 in it contains {} programs.", gr0.len());

    let mut unseen: HashSet<u32> = conns.keys().cloned().collect();
    let mut groups = 1;

    unseen = unseen.difference(&gr0.iter().cloned().collect()).cloned().collect();
    while unseen.len() > 0 {
        let group = find_group(*unseen.iter().next().unwrap(), &conns);
        groups += 1;
        unseen = unseen.difference(&group.iter().cloned().collect()).cloned().collect();
    }

    println!("Found {} program groups.", groups);
}

fn find_group(id: u32, conns: &HashMap<u32, Vec<u32>>) -> HashSet<u32> {
    let mut group = HashSet::new();
    let mut next = Vec::new();
    next.push(id);
     
    while next.len() > 0 {
        let id = next.pop().unwrap();
        if group.contains(&id) { continue; }
        group.insert(id);
        let connections = conns.get(&id).unwrap();
        for v in connections.iter() {
            next.push(*v);
        }
    }

    group
}
