use adventofcode::days;

days!(1, 2, 3, 4, 5, 6, 7, 8, 9);

fn main() {
    if let Some(day) = std::env::args().nth(1) {
        run_day(&day);
    } else {
        run_all();
    }
}
