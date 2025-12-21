fn parse(data: &str) -> Vec<&str> {
    data.trim().lines().collect()
}

fn part1(banks: &[&str]) -> u64 {
    let mut ans = 0;
    for bank in banks {
        let mut max = 0;
        let mut max_d = bank.as_bytes().last().unwrap() - b'0';
        for b in bank.as_bytes().iter().rev().skip(1) {
            let d = b - b'0';
            let joltage = d * 10 + max_d;
            max = max.max(joltage);
            max_d = max_d.max(d);
        }
        ans += max as u64;
    }
    ans
}

fn part2(banks: &[&str]) -> u64 {
    let mut ans = 0;
    for bank in banks {
        let mut f = [0; 13];
        for (i, b) in bank.as_bytes().iter().rev().enumerate() {
            let mut d = (b - b'0') as u64;
            let digits = 11.min(i);
            d *= 10u64.pow(digits as u32);
            for i in (0..=digits).rev() {
                let joltage = d + f[i];
                f[i + 1] = f[i + 1].max(joltage);
                d /= 10;
            }
        }
        ans += f[12];
    }
    ans
}

pub fn main() {
    let data = std::fs::read_to_string("data/2025/day3").unwrap();
    let ranges = parse(&data);
    println!("part1: {}", part1(&ranges));
    println!("part2: {}", part2(&ranges));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "
987654321111111
811111111111119
234234234234278
818181911112111";
        let banks = parse(data);
        assert_eq!(357, part1(&banks));
        assert_eq!(3121910778619, part2(&banks));
    }
}
