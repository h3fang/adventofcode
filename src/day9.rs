use anyhow::Result;
use std::{
    collections::BTreeSet,
    fs,
    io::{self, BufRead},
    iter::FromIterator,
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

fn part1(nums: &[u64], preamble: usize) -> u64 {
    let mut set = BTreeSet::from_iter(&nums[0..preamble]);
    for i in preamble..nums.len() {
        let mut iter = set.iter();
        let min = **iter.next().unwrap() + **iter.next().unwrap();
        let mut iter = set.iter().rev();
        let max = **iter.next().unwrap() + **iter.next().unwrap();
        if nums[i] < min || nums[i] > max {
            return nums[i];
        }
        set.remove(&nums[i - preamble]);
        set.insert(&nums[i]);
    }
    panic!("solution not found");
}

fn part2(nums: &[u64], target: u64) -> u64 {
    let n = nums.len();
    for i in 0..(n - 1) {
        let mut sum = nums[i];
        for j in (i + 1)..n {
            sum += nums[j];
            if sum == target {
                return *nums[i..=j].iter().max().unwrap() + *nums[i..=j].iter().min().unwrap();
            }
        }
    }
    panic!("solution not found");
}

pub fn main(file_path: &str) -> Result<()> {
    let nums = parse(file_path)?;

    // part 1
    let target = part1(&nums, 25);
    println!("day9 part1: {}", target);

    // part 2
    println!("day9 part2: {}", part2(&nums, target));

    Ok(())
}
