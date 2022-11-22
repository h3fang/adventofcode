use std::time::Instant;

use adventofcode::days;

days!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25);

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
