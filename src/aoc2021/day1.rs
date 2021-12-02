fn part1(measurements: &[i64]) -> usize {
    measurements.windows(2).filter(|w| w[1] > w[0]).count()
}

fn part2(measurements: &[i64]) -> usize {
    measurements.windows(4).filter(|w| w[3] > w[0]).count()
}

pub fn main() {
    let measurements: Vec<i64> = std::fs::read_to_string("data/2021/day1")
        .unwrap()
        .lines()
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    println!("day1 part1: {}", part1(&measurements));
    println!("day1 part2: {}", part2(&measurements));
}
