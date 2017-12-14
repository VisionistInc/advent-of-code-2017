use std::io::Result;
use std::cmp::max;

pub fn solve<L: Iterator<Item=Result<String>>>(input: &mut L) -> () {
    let line = input.next().unwrap().unwrap();
    let origin = HexCoord::new(0,0,0);
    let mut p = HexCoord::new(0,0,0);
    let mut max_d = 0;
    for m in line.split(',') {
        match m {
            "n" => p = p.N(),
            "ne" => p = p.NE(),
            "se" => p = p.SE(),
            "s" => p = p.S(),
            "sw" => p = p.SW(),
            "nw" => p = p.NW(),
            _ => panic!("failure captain!"),
        }
        max_d = max(max_d, origin.cube_distance(&p));
    }

    println!("Distance to reach child: {}", origin.cube_distance(&p));
    println!("Distance to reach child: {}", origin.cube_distance_max(&p));
    println!("Max distance: {}", max_d);
}

#[derive(Debug, PartialEq, Eq)]
struct HexCoord {
    x: i32,
    y: i32,
    z: i32,
}

// with thanks to https://www.redblobgames.com/grids/hexagons/
#[allow(non_snake_case)]
impl HexCoord {
    fn new(x: i32, y: i32, z: i32) -> Self {
        HexCoord {
            x: x,
            y: y,
            z: z,
        }
    }

    fn N(&self) -> Self {
        HexCoord{
            x: self.x,
            y: self.y+1,
            z: self.z-1,
        }
    }

    fn NE(&self) -> Self {
        HexCoord{
            x: self.x+1,
            y: self.y,
            z: self.z-1,
        }
    }

    fn SE(&self) -> Self {
        HexCoord{
            x: self.x+1,
            y: self.y-1,
            z: self.z,
        }
    }

    fn S(&self) -> Self {
        HexCoord {
            x: self.x,
            y: self.y-1,
            z: self.z+1,
        }
    }

    fn SW(&self) -> Self {
        HexCoord {
            x: self.x-1,
            y: self.y,
            z: self.z+1,
        }
    }

    fn NW(&self) -> Self {
        HexCoord {
            x: self.x-1,
            y: self.y+1,
            z: self.z,
        }
    }

    fn cube_distance(&self, oth: &Self) -> i32 {
        return ((self.x - oth.x).abs() + (self.y - oth.y).abs() + (self.z - oth.z).abs())/2;
    }

    fn cube_distance_max(&self, oth: &Self) -> i32 {
        let x = (self.x - oth.x).abs();
        let y = (self.y - oth.y).abs();
        let z = (self.z - oth.z).abs();
        return max(max(x, y), max(y, z));
    }
}
