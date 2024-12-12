use ahash::{HashMap, HashMapExt};

fn parse(input: &str) -> Vec<u64> {
    input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn blink(stones: &[u64], times: &[usize]) -> Vec<usize> {
    let mut m: HashMap<u64, usize> = HashMap::with_capacity(stones.len());
    for &x in stones {
        *m.entry(x).or_insert(0) += 1;
    }
    let mut blinks = 0;
    let mut result = Vec::with_capacity(times.len());
    for &t in times {
        while blinks < t {
            let mut next = HashMap::with_capacity(m.len() * 2);
            for (x, c) in m {
                if x == 0 {
                    *next.entry(1).or_default() += c;
                } else if x.ilog10() % 2 == 1 {
                    let base = 10u64.pow((x.ilog10() + 1) / 2);
                    *next.entry(x / base).or_default() += c;
                    *next.entry(x % base).or_default() += c;
                } else {
                    *next.entry(x * 2024).or_default() += c;
                }
            }
            m = next;
            blinks += 1;
        }
        result.push(m.values().sum());
    }
    result
}

pub fn main() {
    let input: String = std::fs::read_to_string("data/2024/day11").unwrap();
    let stones = parse(&input);
    let result = blink(&stones, &[25, 75]);
    println!("part1: {}", result[0]);
    println!("part2: {}", result[1]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = "125 17";
        let stones = parse(input);
        assert_eq!(vec![22, 55312], blink(&stones, &[6, 25]));
    }
}
