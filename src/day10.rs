use anyhow::Result;
use std::{
    fs,
    io::{self, BufRead},
};

fn parse(file_path: &str) -> Result<Vec<u64>> {
    let data_file = fs::File::open(file_path)?;
    Ok(io::BufReader::new(data_file)
        .lines()
        .flatten()
        .map(|line| {
            line.parse::<u64>()
                .unwrap_or_else(|_| panic!("invalid line: {}", line))
        })
        .collect())
}

fn part1(adapters: &[u64]) -> u64 {
    let mut current = 0;
    let mut dj = [0, 0, 0, 1];
    for &j in adapters {
        dj[(j - current) as usize] += 1;
        current = j;
    }
    dj[1] * dj[3]
}

fn part2(jolts: &[u64]) -> u64 {
    let jolts = [&[0], jolts].concat();
    let n = jolts.len();
    let mut dp = vec![0; n];
    dp[n - 1] = 1;
    for i in (0..=n - 2).rev() {
        let upper = jolts[i] + 3;
        for j in 1..=3 {
            if i + j < n && upper >= jolts[i + j] {
                dp[i] += dp[i + j];
            } else {
                break;
            }
        }
    }
    dp[0]
}

pub fn main(file_path: &str) -> Result<()> {
    let mut adapters = parse(file_path)?;
    adapters.sort_unstable();

    // part 1
    println!("day 10 part1: {}", part1(&adapters));

    // part 2
    println!("day 10 part2: {}", part2(&adapters));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day10_1() {
        let mut adapters = parse("data/day10-1").unwrap();
        adapters.sort_unstable();

        assert_eq!(35, part1(&adapters));
        assert_eq!(8, part2(&adapters));
    }

    #[test]
    fn test_day10_2() {
        let mut adapters = parse("data/day10-2").unwrap();
        adapters.sort_unstable();

        assert_eq!(220, part1(&adapters));
        assert_eq!(19208, part2(&adapters));
    }
}
