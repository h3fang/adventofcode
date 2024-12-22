use ahash::{HashMap, HashMapExt, HashSet, HashSetExt};

fn parse(input: &str) -> Vec<i64> {
    input
        .trim()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

fn pseudorandom(mut x: i64) -> i64 {
    x = ((x * 64) ^ x) % 16777216;
    x = ((x / 32) ^ x) % 16777216;
    ((x * 2048) ^ x) % 16777216
}

fn sequence(s: &mut i64, profits: &mut HashMap<u32, i32>) {
    let (mut p1, mut w) = ((*s % 10) as i8, 0);
    for _ in 0..4 {
        *s = pseudorandom(*s);
        let p2 = (*s % 10) as i8;
        w = w * 19 + (p2 - p1 + 9) as u32;
        p1 = p2;
    }
    *profits.entry(w).or_default() += p1 as i32;
    let mut seen = HashSet::with_capacity(1997);
    for _ in 0..2000 - 4 {
        *s = pseudorandom(*s);
        let p2 = (*s % 10) as i8;
        w %= 19 * 19 * 19;
        w = w * 19 + (p2 - p1 + 9) as u32;
        p1 = p2;
        if seen.insert(w) {
            *profits.entry(w).or_default() += p1 as i32;
        }
    }
}

fn solve(mut secrets: Vec<i64>) -> (i64, i32) {
    let mut maps = HashMap::with_capacity(19 * 19 * 19 * 19);
    secrets.iter_mut().for_each(|x| sequence(x, &mut maps));
    println!("{}", maps.len());
    let p1 = secrets.into_iter().sum();
    let p2 = *maps.values().max().unwrap();
    (p1, p2)
}

pub fn main() {
    let input: String = std::fs::read_to_string("data/2024/day22").unwrap();
    let secrets = parse(&input);
    let (p1, p2) = solve(secrets);
    println!("part1: {p1}");
    println!("part2: {p2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random() {
        let nums = [
            123, 15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484,
            7753432, 5908254,
        ];
        for w in nums.windows(2) {
            assert_eq!(w[1], pseudorandom(w[0]));
        }
    }

    #[test]
    fn case1() {
        let input = "
1
10
100
2024";
        let secrets = parse(input);
        assert_eq!(37327623, solve(secrets).0);
    }

    #[test]
    fn case2() {
        let input = "
1
2
3
2024";
        let secrets = parse(input);
        assert_eq!(23, solve(secrets).1);
    }
}
