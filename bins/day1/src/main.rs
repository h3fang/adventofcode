use anyhow::Result;
use std::collections::HashMap;
use std::io::{self, BufRead};

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

fn main() -> Result<()> {
    let stdin = io::stdin();
    let numbers: Vec<i32> = stdin
        .lock()
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    if let Some(n) = two_sum(2020, &numbers)? {
        println!("two sum: {}", n);
    }
    if let Some(n) = three_sum(2020, &numbers)? {
        println!("three sum: {}", n);
    }
    Ok(())
}
