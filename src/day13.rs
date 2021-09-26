use anyhow::Result;
use std::{
    fs,
    io::{self, BufRead},
};

fn parse(file_path: &str) -> Result<(usize, Vec<Option<usize>>)> {
    let data_file = fs::File::open(file_path)?;
    let mut lines = io::BufReader::new(data_file).lines().flatten();
    let timestamp = lines.next().unwrap().parse::<usize>()?;
    let buses = lines
        .next()
        .unwrap()
        .split(',')
        .map(|e| {
            if let Ok(n) = e.parse::<usize>() {
                Some(n)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    Ok((timestamp, buses))
}

fn part1(timestamp: usize, buses: &[Option<usize>]) -> usize {
    let wait_time = buses
        .iter()
        .flatten()
        .map(|&b| {
            let remaining = timestamp % b;
            if remaining > 0 {
                (b, b - remaining)
            } else {
                (b, 0)
            }
        })
        .min_by_key(|e| e.1)
        .unwrap();
    wait_time.0 * wait_time.1
}

fn part2(_timestamp: usize, buses: &[Option<usize>]) -> usize {
    let buses = buses.iter().enumerate().collect::<Vec<_>>();
    let mut sorted = buses
        .iter()
        .filter_map(|&(d, b)| b.as_ref().map(|&b| (d, b)))
        .collect::<Vec<_>>();
    sorted.sort_unstable_by_key(|(_, b)| *b);
    let (d_max, b_max) = *sorted.last().unwrap();
    let mut t = b_max - d_max;
    let mut step = b_max;
    sorted.iter().for_each(|&(d, b)| {
        while (t + d) % b != 0 {
            t += step;
        }
        step *= b;
    });
    t
}

pub fn main(file_path: &str) -> Result<()> {
    let (timestamp, buses) = parse(file_path)?;

    // part 1
    println!("day 13 part1: {}", part1(timestamp, &buses));

    // part 2
    println!("day 13 part2: {}", part2(timestamp, &buses));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let (timestamp, buses) = parse("data/day13-1").unwrap();
        assert_eq!(1068781, part2(timestamp, &buses));

        let (timestamp, buses) = parse("data/day13-2").unwrap();
        assert_eq!(1202161486, part2(timestamp, &buses));
    }
}
