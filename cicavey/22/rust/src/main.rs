use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::collections::HashMap;

extern crate modulo;
use modulo::Mod;

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
enum NodeState {
    Clean,
    Weakened,
    Infected,
    Flagged,
    Unknown
}

fn turn_right(dir: usize) -> usize {
    (dir as isize - 1).modulo(4 as isize) as usize
}

fn turn_left(dir: usize) -> usize {
    (dir as isize + 1).modulo(4 as isize) as usize
}

fn reverse(dir: usize) -> usize {
    (dir as isize + 2).modulo(4 as isize) as usize
}

fn mutate_part1(ns: NodeState, dir: usize) -> (NodeState, usize, u64) {
    match ns {
        NodeState::Infected => (NodeState::Clean,    turn_right(dir), 0),
        NodeState::Clean    => (NodeState::Infected, turn_left(dir),  1),
        _                   => (NodeState::Unknown,  0,               0) 
    }
}

fn mutate_part2(ns: NodeState, dir: usize) -> (NodeState, usize, u64) {
    match ns {
        NodeState::Clean    => (NodeState::Weakened, turn_left(dir),  0),
        NodeState::Weakened => (NodeState::Infected, dir,             1),
        NodeState::Infected => (NodeState::Flagged,  turn_right(dir), 0),
        NodeState::Flagged  => (NodeState::Clean,    reverse(dir),    0),
        _                   => (NodeState::Unknown,  0,               0) 
    }
}

fn main() {
    let f = File::open("../input.txt").unwrap();
    let file = BufReader::new(&f);

    //              U       L       D      R
    let dirs = vec![(0,-1), (-1,0), (0,1), (1,0)];

    let mut grid: HashMap<(i64, i64), NodeState> = HashMap::new();

    let mut y = 0;
    for line in file.lines() {
        let mut x = 0;
        for c in line.unwrap().chars() {
            if c == '#' {
                grid.insert((x, y), NodeState::Infected);
            } else {
                grid.insert((x, y), NodeState::Clean);
            }
            
            x += 1;
        }
        y += 1;
    }

    let mut pos: (i64, i64) = (y>>1, y>>1);
    let mut dir: usize = 0; // index into DIRS
    let mut infection_counter: u64 = 0;

    let mutate = mutate_part2;

    for _t in 0..10000000 {
        let ns = grid.entry(pos).or_insert(NodeState::Clean);
        let m = mutate(*ns, dir);
        *ns = m.0;
        dir = m.1;
        infection_counter += m.2;

        // move in direction
        pos.0 += dirs[dir].0;
        pos.1 += dirs[dir].1;
    }

    println!("{}", infection_counter);
}