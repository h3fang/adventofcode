use std::cmp::Ordering;

use nom::{
    character::complete,
    combinator::map,
    multi::separated_list0,
    sequence::{preceded, terminated},
    IResult,
};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Item {
    Value(u8),
    List(Vec<Item>),
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Item::Value(a), Item::Value(b)) => a.partial_cmp(b),
            (Item::Value(a), Item::List(b)) => compare_list(&[Item::Value(*a)], b),
            (Item::List(a), Item::Value(b)) => compare_list(a, &[Item::Value(*b)]),
            (Item::List(a), Item::List(b)) => a.partial_cmp(b),
        }
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn compare_list(a: &[Item], b: &[Item]) -> Option<Ordering> {
    a.partial_cmp(b)
}

fn parse_item(s: &str) -> IResult<&str, Item> {
    if s.as_bytes()[0] == b'[' {
        map(parse_list, Item::List)(s)
    } else {
        map(complete::u8, Item::Value)(s)
    }
}

fn parse_list(s: &str) -> IResult<&str, Vec<Item>> {
    preceded(
        complete::char('['),
        terminated(
            separated_list0(complete::char(','), parse_item),
            complete::char(']'),
        ),
    )(s)
}

fn parse(data: &str) -> Vec<Vec<Item>> {
    data.trim()
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| parse_list(line).unwrap().1)
        .collect()
}

fn part1(signals: &[Vec<Item>]) -> usize {
    signals
        .chunks(2)
        .enumerate()
        .map(|(i, w)| if w[0] < w[1] { i + 1 } else { 0 })
        .sum()
}

fn part2(mut signals: Vec<Vec<Item>>) -> usize {
    let dividers = parse("[[2]]\n[[6]]");
    signals.extend(dividers.clone());
    signals.sort_unstable();
    dividers
        .iter()
        .map(|d| signals.binary_search(d).unwrap() + 1)
        .product()
}

pub fn main() {
    let data = std::fs::read_to_string("data/2022/day13").unwrap();
    let signals = parse(&data);
    println!("part1: {}", part1(&signals));
    println!("part2: {}", part2(signals));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";
        let signals = parse(&data);
        assert_eq!(13, part1(&signals));
        assert_eq!(140, part2(signals));
    }
}
