
fn shell_range(s: i32) -> (i32, i32) {
    let l = 2*s + 1;
    let p = 2*(s-1) + 1;
    return (p*p+1, l * l)
}

fn main() {
    let input = 361527i32;
    for i in 2..1024i32 {
        let (min, max) = shell_range(i);
        if input >= min && input <= max {
            let l = 2 * i + 1; // size of side
            // Offset within this cycle (min-1 is max of prev)
            let seq_offset = input - (min - 1);
            let side_mod = seq_offset % (l-1);
            let mh_dist = i + (side_mod - i).abs(); // min dist + offset from axis
            println!("Found shell: {}, {}", i, mh_dist);
            break;
        }
    }

    // Good enough for @aladdy and the rest of the internet so it's good enough for me
    // In order to solve part 2 I cheated and looked up the pregenerated sequence for
    // http://oeis.org/A141481 -> http://oeis.org/A141481/b141481.txt
}