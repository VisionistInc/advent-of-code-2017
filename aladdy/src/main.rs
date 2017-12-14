use std::io::stdin;
use std::io::BufRead;
use std::ops::Deref;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;

fn main() {
    let stdin = stdin();
    let handle = stdin.lock();

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("missing argument for day0 to run");
        return;
    } else if args.len() > 2 {
        println!("too many arguments");
        return;
    }

    let ref day0 = args[1];
    match day0.deref() {
        "day01" => day01::solve(&mut (handle.lines())),
        "day02" => day02::solve(&mut (handle.lines())),
        "day03" => day03::solve(&mut (handle.lines())),
        "day04" => day04::solve(&mut (handle.lines())),
        "day05" => day05::solve(&mut (handle.lines())),
        "day06" => day06::solve(&mut (handle.lines())),
        "day07" => day07::solve(&mut (handle.lines())),
        "day08" => day08::solve(&mut (handle.lines())),
        "day09" => day09::solve(&mut (handle.lines())),
        "day10" => day10::solve(&mut (handle.lines())),
        "day11" => day11::solve(&mut (handle.lines())),

        _ => println!("Unknown day0: {}", args[1]),
    }
}
