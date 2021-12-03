mod day1;
mod day2;
mod day3;

pub const DAYS: &[fn()] = &[day1::main, day2::main, day3::main];

use std::time::Instant;

fn main() {
    DAYS.iter().for_each(|f| {
        let start = Instant::now();
        f();
        println!("time: {:?}\n", Instant::now() - start);
    });
}
