pub const DAYS: &[fn()] = &[
];

use std::time::Instant;

fn main() {
    DAYS.iter().for_each(|f| {
        let start = Instant::now();
        f();
        println!("time: {:?}\n", Instant::now() - start);
    });
}
