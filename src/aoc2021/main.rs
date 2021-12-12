mod day1;
mod day10;
mod day11;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

pub const DAYS: &[fn()] = &[
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
];

use std::time::Instant;

fn main() {
    if let Some(day) = std::env::args().nth(1) {
        let day = day.parse::<usize>().expect("invalid day");
        if day >= 1 && day <= DAYS.len() {
            let start = Instant::now();
            DAYS[day - 1]();
            println!("time: {:?}\n", Instant::now() - start);
        } else {
            panic!("invalid day");
        }
    } else {
        DAYS.iter().for_each(|f| {
            let start = Instant::now();
            f();
            println!("time: {:?}\n", Instant::now() - start);
        });
    }
}
