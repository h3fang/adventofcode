use ahash::AHashSet as HashSet;

fn part1(nums: &[i64]) -> i64 {
    nums.iter().sum()
}

fn part2(nums: &[i64]) -> i64 {
    let mut s = HashSet::new();
    let mut curr = 0;
    s.insert(curr);
    let mut i = 0;
    loop {
        if i == nums.len() {
            i = 0;
        }
        curr += nums[i];
        if s.contains(&curr) {
            return curr;
        }
        s.insert(curr);
        i += 1;
    }
}

pub fn main() {
    let nums: Vec<i64> = std::fs::read_to_string("data/2018/day1")
        .unwrap()
        .lines()
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    println!("part1: {}", part1(&nums));
    println!("part2: {}", part2(&nums));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let nums = vec![-1, 1];
        assert_eq!(0, part2(&nums));
    }

    #[test]
    fn case2() {
        let nums = vec![3, 3, 4, -2, -4];
        assert_eq!(10, part2(&nums));
    }

    #[test]
    fn case3() {
        let nums = vec![-6, 3, 8, 5, -6];
        assert_eq!(5, part2(&nums));
    }

    #[test]
    fn case4() {
        let nums = vec![7, 7, -2, -7, -4];
        assert_eq!(14, part2(&nums));
    }
}
