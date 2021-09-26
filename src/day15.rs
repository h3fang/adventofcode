use anyhow::Result;
use std::{
    fs,
    io::{self, BufRead},
};

fn parse(file_path: &str) -> Result<Vec<usize>> {
    let data_file = fs::File::open(file_path)?;
    let line = io::BufReader::new(data_file)
        .lines()
        .flatten()
        .next()
        .unwrap();
    Ok(line
        .split(',')
        .map(|token| token.parse().unwrap())
        .collect::<Vec<_>>())
}

fn part(starting_nums: &[usize], target_pos: usize) -> usize {
    let mut map = vec![usize::MAX; target_pos];
    let mut last = *starting_nums.first().unwrap();
    for (i, n) in starting_nums.iter().skip(1).enumerate() {
        map[last] = i;
        last = *n;
    }

    for i in starting_nums.len()..target_pos {
        let turn = map[last];
        let next = if turn == usize::MAX { 0 } else { i - 1 - turn };
        map[last] = i - 1;

        last = next;
    }
    last
}

pub fn main(file_path: &str) -> Result<()> {
    let starting_nums = parse(file_path)?;

    // part 1
    println!("day 15 part1: {}", part(&starting_nums, 2020));

    // part 2
    println!("day 15 part2: {}", part(&starting_nums, 30000000));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let starting_nums = parse("data/day15-1").unwrap();
        assert_eq!(436, part(&starting_nums, 2020));

        let starting_nums = parse("data/day15-2").unwrap();
        assert_eq!(1836, part(&starting_nums, 2020));
    }
}
