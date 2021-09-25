use anyhow::Result;
use std::collections::HashMap;
use std::{
    fs,
    io::{self, BufRead},
};

fn two_sum(target: i32, nums: &[i32]) -> Result<Option<i32>> {
    let mut numbers = HashMap::new();
    for &n in nums {
        let entry = numbers.entry(target - n).or_insert(0);
        if *entry > 0 {
            return Ok(Some((target - n) * n));
        }
        *numbers.entry(n).or_insert(0) += 1;
    }
    Ok(None)
}

fn three_sum(target: i32, nums: &[i32]) -> Result<Option<i32>> {
    for &n in nums {
        if let Some(r) = two_sum(target - n, nums)? {
            return Ok(Some(r * n));
        }
    }
    Ok(None)
}

pub fn main(file_path: &str) -> Result<()> {
    let data_file = fs::File::open(file_path)?;
    let numbers: Vec<i32> = io::BufReader::new(data_file)
        .lines()
        .flatten()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();

    if let Some(n) = two_sum(2020, &numbers)? {
        println!("day1 part1: {}", n);
    }

    if let Some(n) = three_sum(2020, &numbers)? {
        println!("day1 part2: {}", n);
    }

    Ok(())
}
