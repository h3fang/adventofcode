mod day1;
mod day2;
pub const DAYS: &[fn()] = &[
    day1::main,
    day2::main,
];

use std::time::Instant;

fn main() {
    DAYS.iter().for_each(|f| {
        let start = Instant::now();
        f();
        println!("time: {:?}\n", Instant::now() - start);
    });
}
