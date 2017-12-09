use std::io::stdin;
use std::io::BufRead;
use std::ops::Deref;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

fn main() {
    let stdin = stdin();
    let handle = stdin.lock();

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("missing argument for day to run");
        return;
    } else if args.len() > 2 {
        println!("too many arguments");
        return;
    }

    let ref day = args[1];
    match day.deref() {
        "day1" => day1::solve(&mut (handle.lines())),
        "day2" => day2::solve(&mut (handle.lines())),
        "day3" => day3::solve(&mut (handle.lines())),
        "day4" => day4::solve(&mut (handle.lines())),
        "day5" => day5::solve(&mut (handle.lines())),
        "day6" => day6::solve(&mut (handle.lines())),
        "day7" => day7::solve(&mut (handle.lines())),
        "day8" => day8::solve(&mut (handle.lines())),

        _ => println!("Unknown day: {}", args[1]),
    }
}
