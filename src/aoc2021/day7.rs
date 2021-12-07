fn part1(pos: &[i64]) -> i64 {
    let n = pos.len();
    let m = pos[n / 2];
    pos.iter().map(|p| (p - m).abs()).sum()
}

fn part2(pos: &[i64]) -> i64 {
    let n = pos.len();
    let sum = pos.iter().sum::<i64>();
    let avg = sum / n as i64;
    (avg - 1..=avg + 1)
        .map(|avg| {
            pos.iter()
                .map(|p| {
                    let d = (p - avg).abs();
                    d * (d + 1) / 2
                })
                .sum::<i64>()
        })
        .min()
        .unwrap()
}

pub fn main() {
    let data = std::fs::read_to_string("data/2021/day7").unwrap();
    let mut pos = data
        .trim()
        .split(',')
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    pos.sort_unstable();
    println!("day7 part1: {}", part1(&pos));
    println!("day7 part2: {}", part2(&pos));
}
