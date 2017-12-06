use std::io::Result;

pub fn solve<L: Iterator<Item=Result<String>>>(input: &mut L) -> () {
    let mut maze: Vec<isize> = input.map(|line| line.unwrap().parse().unwrap()).collect();
    let mut maize = maze.clone();

    let mut i = 0isize;
    let mut steps = 0;
    loop {
        steps += 1;

        let jmp = maze[i as usize];
        maze[i as usize] += 1;

        i += jmp;
        if i >= maze.len() as isize || i < 0 {
            break;
        }
    }
    println!("Total steps to escape: {}", steps);

    i = 0;
    steps = 0;
    loop {
        steps += 1;

        let jmp = maize[i as usize];
        if jmp >= 3 {
            maize[i as usize] -= 1;
        } else {
            maize[i as usize] += 1;
        }

        i += jmp;
        if i >= maize.len() as isize || i < 0 {
            break;
        }
    }
    println!("Total corn to escape maize: {}", steps);
}
