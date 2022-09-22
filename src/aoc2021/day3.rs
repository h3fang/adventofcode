fn part1(nums: &[&[u8]]) -> i64 {
    let n = nums.len();
    let mut gamma = 0;
    for i in 0..nums[0].len() {
        let ones = nums.iter().filter(|n| n[i] == b'1').count();
        gamma <<= 1;
        if ones > n - ones {
            gamma += 1;
        }
    }
    let epsilon = (!gamma) & ((1 << nums[0].len()) - 1);
    gamma * epsilon
}

fn part2(nums: &[&[u8]]) -> i64 {
    fn filter(nums: &[&[u8]], a: u8, b: u8) -> i64 {
        let mut candidates = nums.to_vec();
        let mut i = 0;
        while candidates.len() > 1 {
            let n = candidates.len();
            let ones = candidates.iter().filter(|c| c[i] == b'1').count();
            let common_bit = if ones >= n - ones { a } else { b };
            candidates.retain(|c| c[i] == common_bit);
            i += 1;
        }
        let mut result = 0;
        for b in candidates[0] {
            result <<= 1;
            if *b == b'1' {
                result += 1;
            }
        }
        result
    }
    let oxygen = filter(nums, b'1', b'0');
    let co2 = filter(nums, b'0', b'1');
    oxygen * co2
}

pub fn main() {
    let data = std::fs::read_to_string("data/2021/day3").unwrap();
    let nums = data
        .lines()
        .map(|s| s.trim().as_bytes())
        .collect::<Vec<_>>();

    println!("day3 part1: {}", part1(&nums));
    println!("day3 part2: {}", part2(&nums));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "00100
        11110
        10110
        10111
        10101
        01111
        00111
        11100
        10000
        11001
        00010
        01010";
        let nums = data
            .lines()
            .map(|s| s.trim().as_bytes())
            .collect::<Vec<_>>();

        assert_eq!(198, part1(&nums));
        assert_eq!(230, part2(&nums));
    }
}
