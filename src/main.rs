use adventofcode::*;
use anyhow::Result;
use std::time::Instant;

fn main() -> Result<()> {
    let fns: Vec<(fn(&str) -> Result<()>, &str)> = vec![
        (day1::main, "data/day1"),
        (day2::main, "data/day2"),
        (day3::main, "data/day3"),
        (day4::main, "data/day4"),
        (day5::main, "data/day5"),
        (day6::main, "data/day6"),
        (day7::main, "data/day7"),
        (day8::main, "data/day8"),
        (day9::main, "data/day9"),
        (day10::main, "data/day10"),
        (day11::main, "data/day11"),
    ];

    fns.iter().for_each(|(f, path)| {
        let start = Instant::now();
        f(path).unwrap();
        println!("time: {:?}\n", Instant::now() - start);
    });

    Ok(())
}
