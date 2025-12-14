use ahash::{HashSet, HashSetExt};

fn parse(data: &str) -> Vec<(&str, &str)> {
    data.trim()
        .split(',')
        .map(|range| range.split_once('-').unwrap())
        .collect()
}

fn check_invalid(a: &str, b: &str, k: usize, invalid: &mut HashSet<u64>) {
    let x: u64 = a.parse().unwrap();
    let y: u64 = b.parse().unwrap();
    let n = (a.len() + 1) / k;
    let m = 10u64.pow(n as u32);
    let left_min: u64 = a[..a.len() - n * (k - 1)].parse().unwrap_or(1).max(m / 10);
    let left_max: u64 = b[..b.len() - n * (k - 1)]
        .parse::<u64>()
        .unwrap()
        .min(m - 1);
    for left in left_min..=left_max {
        let z = left.to_string().repeat(k).parse().unwrap();
        if (x..=y).contains(&z) {
            invalid.insert(z);
        }
    }
}

fn part1(ranges: &[(&str, &str)]) -> u64 {
    let mut invalid = HashSet::with_capacity(1024);
    ranges
        .iter()
        .map(|&(a, b)| {
            invalid.clear();
            check_invalid(a, b, 2, &mut invalid);
            invalid.iter().sum::<u64>()
        })
        .sum()
}

fn part2(ranges: &[(&str, &str)]) -> u64 {
    let mut ans = 0;
    let mut invalid = HashSet::with_capacity(1024);
    for &(a, b) in ranges {
        invalid.clear();
        let n = b.len();
        for k in 2..=n {
            check_invalid(a, b, k, &mut invalid);
        }
        ans += invalid.iter().sum::<u64>();
    }
    ans
}

pub fn main() {
    let data = std::fs::read_to_string("data/2025/day2").unwrap();
    let ranges = parse(&data);
    println!("part1: {}", part1(&ranges));
    println!("part2: {}", part2(&ranges));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let ranges = parse(data);
        assert_eq!(1227775554, part1(&ranges));
        assert_eq!(4174379265, part2(&ranges));
    }
}
