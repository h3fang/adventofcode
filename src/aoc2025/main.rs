use aoc_proc::aoc_event;

aoc_event!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);

fn main() {
    if let Some(day) = std::env::args().nth(1) {
        run_day(&day);
    } else {
        run_all();
    }
}
