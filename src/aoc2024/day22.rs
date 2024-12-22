use std::collections::hash_map::Entry;

use ahash::{HashMap, HashMapExt, HashSet};
use arrayvec::ArrayVec;
use rayon::prelude::*;

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

fn sequence(s: &mut i64) -> HashMap<ArrayVec<i8, 4>, i8> {
    let mut prices = Vec::with_capacity(2000);
    let mut changes = Vec::with_capacity(1999);
    prices.push((*s % 10) as i8);
    for _ in 0..2000 {
        let s1 = pseudorandom(*s);
        prices.push((s1 % 10) as i8);
        changes.push((s1 % 10 - *s % 10) as i8);
        *s = s1;
    }
    let mut m = HashMap::with_capacity(1997);
    for (i, w) in changes.windows(4).enumerate() {
        let w = ArrayVec::try_from(w).unwrap();
        match m.entry(w) {
            Entry::Occupied(_e) => {}
            Entry::Vacant(e) => {
                e.insert(prices[i + 4]);
            }
        }
    }
    m
}

fn solve(mut secrets: Vec<i64>) -> (i64, i64) {
    let maps = secrets.par_iter_mut().map(sequence).collect::<Vec<_>>();
    let p1 = secrets.into_iter().sum();
    let keys: HashSet<_> = maps.iter().flat_map(|m| m.keys()).collect();
    let p2 = keys
        .par_iter()
        .map(|&k| {
            maps.iter()
                .map(|m| m.get(k).cloned().unwrap_or(0) as i64)
                .sum()
        })
        .max()
        .unwrap();
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
