fn parse(data: &str) -> Vec<Vec<i64>> {
    data.trim()
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect()
}

fn predict(signal: &[i64]) -> i64 {
    let mut last = vec![*signal.last().unwrap()];
    let mut diff = signal.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();
    last.push(*diff.last().unwrap());
    while !diff.iter().all(|e| *e == 0) {
        diff = diff.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();
        last.push(*diff.last().unwrap());
    }
    last.into_iter().sum()
}

fn part1(signals: &[Vec<i64>]) -> i64 {
    signals.iter().map(|s| predict(s)).sum()
}

fn extrapolate_backward(signal: &[i64]) -> i64 {
    let mut first = vec![signal[0]];
    let mut diff = signal.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();
    first.push(diff[0]);
    while !diff.iter().all(|e| *e == 0) {
        diff = diff.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();
        first.push(diff[0]);
    }
    first.into_iter().rev().fold(0, |acc, n| n - acc)
}

fn part2(signals: &[Vec<i64>]) -> i64 {
    signals.iter().map(|s| extrapolate_backward(s)).sum()
}

pub fn main() {
    let data = std::fs::read_to_string("data/2023/day9").unwrap();
    let signals = parse(&data);
    println!("part1: {}", part1(&signals));
    println!("part2: {}", part2(&signals));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let signals = parse(&data);
        assert_eq!(114, part1(&signals));
        assert_eq!(2, part2(&signals));
    }
}
