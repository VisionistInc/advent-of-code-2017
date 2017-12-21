use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Pattern {
    data: Vec<bool>,
    size: usize,
}

impl Pattern {
    fn base() -> Pattern {
        Pattern { data: vec![false, true, false, false, false, true, true, true, true], size: 3}
    }
    fn new(input: & String) -> Pattern {
        let src: Vec<Vec<u8>> = input.split("/").map(|s| s.bytes().collect()).collect();
        let size = src.len();
        let mut data: Vec<bool> = Vec::with_capacity(size * size);
        // packed row major
        for y in 0..size {
            for x in 0..size {
                data.push(src[y][x] == 35);
            }
        }
        Pattern { data: data, size: size}
    }
    fn newsize(size: usize) -> Pattern {
        let mut data: Vec<bool> = Vec::with_capacity(size * size);
        for _i in 0..size*size {
            data.push(false);
        }
        Pattern { data: data, size: size}
    }
    // flip around y axis
    fn flip(&self) -> Pattern {
        let mut data: Vec<bool> = Vec::with_capacity(self.data.len());
        for y in 0..self.size {
            for x in 0..self.size {
                data.push(self.data[y*self.size + (self.size - 1 - x)]);
            }
        }
        Pattern { data: data, size: self.size}
    }

    // row becomes col, sq mat ftw
    fn transpose(&self) -> Pattern {
        let mut data: Vec<bool> = self.data.clone();
        for y in 0..self.size {
            for x in 0..self.size {
                let dx = y;
                let dy = x;
                data[dy*self.size + dx] = self.data[y*self.size + x];
            }
        }
        Pattern { data: data, size: self.size}
    }
    fn subpattern(&self, sx: usize, sy: usize, s: usize) -> Pattern {
        let mut data: Vec<bool> = Vec::with_capacity(s*s);
        let offset = sy * self.size + sx;
        for y in 0..s {
            for x in 0..s {
                data.push(self.data[offset + y * self.size + x]);
            }
        }
        Pattern{ data: data, size: s}
    }

    fn write_subpattern(&mut self, sx: usize, sy: usize, p: &Pattern) {
        let offset = sy * self.size + sx;
        for y in 0..p.size {
            for x in 0..p.size {
                self.data[offset + y * self.size + x] = p.data[y * p.size + x];
            }
        }
    }

    fn on(&self) -> usize {
        self.data.iter().filter(|&x| *x).count()
    }
}

impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for y in 0..self.size {
            for x in 0..self.size {
                if self.data[y*self.size + x] {
                    s.push_str("#");
                } else {
                    s.push_str(".");
                }
            }
            if y != self.size-1 {
                s.push_str("\n");
            }
        }
        write!(f, "{}", s)
    }
}

fn main() {
    let f = File::open("../input.txt").unwrap();
    let file = BufReader::new(&f);

    let mut pmap = HashMap::new();

    for line in file.lines() {
        let p: Vec<String> = line.unwrap().split(" => ").map(|s| s.to_string()).collect();
        // Calculate all possible permuations of input pattern and dedup with hashset
        let mut pat = Pattern::new(&p[0]);
        let mut hs = HashSet::new();
        hs.insert(pat.clone());
        for _i in 0..4 {
            let pt = pat.transpose();
            hs.insert(pt.clone());
            pat = pt.flip();
            hs.insert(pat.clone());
        }

        // Map all possible permuations to target pattern
        let mut dst_pat = Pattern::new(&p[1]);
        for pat_perm in &hs {
            pmap.insert(pat_perm.clone(), dst_pat.clone());
        }
    }

    let mut grid = Pattern::base();

    for i in 0..18 {
        if grid.size % 2 == 0 {
            let mut newgrid = Pattern::newsize(grid.size / 2 * 3);
            // Break into 2x2 chunks
            for x in 0..grid.size/2 {
                for y in 0..grid.size/2 {
                    let src = grid.subpattern(x*2, y*2, 2);
                    let dst = pmap.get(&src).unwrap();
                    //println!("{}\n=>\n{}\n", src, dst);
                    newgrid.write_subpattern(x*3, y*3, dst);
                }
            }
            // Map to new patterns
            // Reassemble
            grid = newgrid;
        } else if grid.size % 3 == 0 {
            let mut newgrid = Pattern::newsize(grid.size / 3 * 4);
            // Break into 3x3 chunks
            for x in 0..grid.size/3 {
                for y in 0..grid.size/3 {
                    let src = grid.subpattern(x*3, y*3, 3);
                    let dst = pmap.get(&src).unwrap();
                    newgrid.write_subpattern(x*4, y*4, dst);
                }
            }
            // Map to new patterns
            // Reassemble
            grid = newgrid;
        } else {
            // uh oh
        }

        if i == 4 {
            println!("{}", grid.on());        
        }
    }
    println!("{}", grid.on());
}