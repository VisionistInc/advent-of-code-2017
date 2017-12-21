use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::str;

fn main() {
    let f = File::open("../input.txt").unwrap();
    let file = BufReader::new(&f);
    let mut grid: Vec<Vec<u8>> = Vec::new();
    for line in file.lines() {
        grid.push(line.unwrap().into_bytes());
    }

    let mut x: isize = 0;
    let mut y: isize = 0;
    let mut vx: isize = 0;
    let mut vy: isize = 1;
    let mut turn = 0;
    let mut l : Vec<u8> = Vec::new();
    const SPACE: u8 = 32;
    const PIPE: u8 = 124;
    const DASH: u8 = 45;
    const PLUS: u8 = 43;

    // find start pos
    for i in 0..grid[0].len() {
        if grid[0][i] != SPACE {
            x = i as isize;
            break;
        }
    }

    for i in 0..500000 {
        // Move
        y += vy;
        x += vx;
        // look
        let cur = grid[y as usize][x as usize];

        if cur == SPACE {
            println!("{}", i+1);
            break;
        }

        if cur == PLUS {
            // turn by looking ahead in perpendicular directions\
            // moving horz
            if vx != 0 {
                // look up/down
                if grid[y as usize+1][x as usize] != SPACE {
                    vy = 1;
                    vx = 0;
                } else if grid[y as usize-1][x as usize] != SPACE{
                    vy = -1;
                    vx = 0;
                } else {
                    break;
                }
            } else { // moving vert
                // look left/right
                if grid[y as usize][x as usize+1] != SPACE {
                    vy = 0;
                    vx = 1;
                } else if grid[y as usize][x as usize-1] != SPACE{
                    vy = 0;
                    vx = -1;
                } else {
                    break;
                }
            }
        }

        if cur >= 65 && cur <= 65+26 {
            l.push(cur);
        }
    }

    println!("{}", str::from_utf8(&l).unwrap());
/*
    let mut r = 0;
    for row in grid {
        if r == y {
            let mut crow = row.clone();
            crow[x as usize] = 64;
            println!("{}", str::from_utf8(&crow).unwrap());
        } else {
            println!("{}", str::from_utf8(&row).unwrap());
        }
        r += 1;

        if r > y + 2 {
            break;
        }
    }

    println!("{} {} {} {}", x, y, vx, vy);
    */
}
