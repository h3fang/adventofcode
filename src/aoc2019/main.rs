mod day1;
pub const DAYS: &[fn()] = &[
    day1::main,
];

use std::time::Instant;

fn main() {
    DAYS.iter().for_each(|f| {
        let start = Instant::now();
        f();
        println!("time: {:?}\n", Instant::now() - start);
    });
}
