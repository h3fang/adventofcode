use std::collections::HashMap;

fn two_sum(target: i32, nums: &[i32]) -> Option<i32> {
    let mut numbers = HashMap::new();
    for &n in nums {
        let entry = numbers.entry(target - n).or_insert(0);
        if *entry > 0 {
            return Some((target - n) * n);
        }
        *numbers.entry(n).or_insert(0) += 1;
    }
    None
}

fn three_sum(target: i32, nums: &[i32]) -> Option<i32> {
    for &n in nums {
        if let Some(r) = two_sum(target - n, nums) {
            return Some(r * n);
        }
    }
    None
}

pub fn main() {
    let numbers: Vec<i32> = std::fs::read_to_string("data/2020/day1")
        .unwrap()
        .lines()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();

    if let Some(n) = two_sum(2020, &numbers) {
        println!("day1 part1: {}", n);
    }

    if let Some(n) = three_sum(2020, &numbers) {
        println!("day1 part2: {}", n);
    }
}
