use std::{collections::BTreeSet, iter::FromIterator};

fn parse() -> Vec<u64> {
    include_str!("../data/day9")
        .lines()
        .map(|line| {
            line.parse::<u64>()
                .unwrap_or_else(|_| panic!("invalid line: {}", line))
        })
        .collect()
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

pub fn main() {
    let nums = parse();

    // part 1
    let target = part1(&nums, 25);
    println!("day9 part1: {}", target);

    // part 2
    println!("day9 part2: {}", part2(&nums, target));
}
