mod day1;
mod day2;
mod day3;

pub const DAYS: &[fn()] = &[day1::main, day2::main, day3::main];

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
