pub fn main() {
    let numbers: Vec<i64> = std::fs::read_to_string("data/2019/day1")
        .unwrap()
        .lines()
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    let total: i64 = numbers.iter().map(|n| n / 3 - 2).sum();
    println!("day1 part1: {}", total);

    fn fuel(f: i64) -> i64 {
        let f = f / 3 - 2;
        if f <= 0 {
            0
        } else {
            f + fuel(f)
        }
    }

    let total: i64 = numbers.iter().map(|&n| fuel(n)).sum();
    println!("day1 part2: {}", total);
}
