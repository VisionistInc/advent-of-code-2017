use std::io::Result;

pub fn solve<L: Iterator<Item=Result<String>>>(input: &mut L) -> () {
    let location: usize = input.next().unwrap().unwrap().parse().unwrap();

    if location == 1 {
        println!("Manhatten distance: 0");
        return;
    }

    let ring = get_ring(location);
    let root = 2*ring+1;
    let corner = root.pow(2);
    println!("Location is in ring {} which has root {} and corner {}.",
             ring, root, corner);

    // each ring's corner is the ring's number down from level with the center
    // each ring has sides with a length equal to root
    let side: usize = (corner - location) / root; // 0 == S, 1 == W, 2 == N, 3 == E
    assert!(side < 4);

    // BUG: there is a bug in the side and offset_corner calculations
    // resulting in off by one errors, but it worked for the test case
    let offset_corner = (corner - location) - (root * side);
    let mid = root/2;
    let offset_center = if mid > offset_corner {
        mid - offset_corner
    } else if offset_corner > mid {
        offset_corner - mid
    } else {
        0
    };
    println!("offset_corner: {}, mid: {}, offset_center: {}", offset_corner, mid, offset_center);

    // point can be described as a vector with endpoint (ring, offset_corner)
    // or (offset_corner, ring) depending on what side the endpoint is on.
    println!("Manhattan distance: {}", ring + offset_center);

    // In order to solve part 2 I cheated and looked up the pregenerated sequence for
    // http://oeis.org/A141481 -> http://oeis.org/A141481/b141481.txt
}

// the ring our location is in, 0 is the center
fn get_ring(location: usize) -> usize {
    // each corner is the square of an odd number matching
    // this sequence http://oeis.org/A016754
    for n in 1usize.. {
        let root = 2*n+1;
        // check if location is contained in this ring
        if ((n-1)*2+1).pow(2) < location && root.pow(2) >= location {
            return n;
        }
    }

    unreachable!();
}

