use adventofcode::*;
use std::time::Instant;

fn main() {
    // to make cargo clippy happy
    type Days = Vec<fn() -> ()>;

    let fns: Days = vec![
        day1::main,
        day2::main,
        day3::main,
        day4::main,
        day5::main,
        day6::main,
        day7::main,
        day8::main,
        day9::main,
        day10::main,
        day11::main,
        day12::main,
        day13::main,
        day14::main,
        day15::main,
        day16::main,
        day17::main,
    ];

    fns.iter().for_each(|f| {
        let start = Instant::now();
        f();
        println!("time: {:?}\n", Instant::now() - start);
    });
}
