mod aoc2020;

use std::time::Instant;

fn main() {
    aoc2020::DAYS.iter().for_each(|f| {
        let start = Instant::now();
        f();
        println!("time: {:?}\n", Instant::now() - start);
    });
}
