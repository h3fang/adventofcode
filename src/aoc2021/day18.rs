use std::{fmt::Display, ops::Add, str::FromStr};

#[derive(Debug, Clone, PartialEq)]
enum Number {
    Value(u8),
    Nested(Box<Pair>),
}

impl Number {
    fn magnitude(&self) -> i64 {
        match self {
            Number::Value(v) => *v as i64,
            Number::Nested(p) => 3 * p.left.magnitude() + 2 * p.right.magnitude(),
        }
    }

    fn reduce(&mut self) {
        loop {
            if self.explode(0).is_some() || self.split() {
                continue;
            } else {
                break;
            }
        }
    }

    fn explode(&mut self, level: usize) -> Option<(Option<u8>, Option<u8>)> {
        match self {
            Number::Value(_) => None,
            Number::Nested(p) => p.explode(level),
        }
    }

    fn split(&mut self) -> bool {
        match self {
            Number::Value(_) => false,
            Number::Nested(p) => p.split(),
        }
    }
}

impl Add for Number {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut n = Number::Nested(Box::new(Pair {
            left: self,
            right: rhs,
        }));
        n.reduce();
        n
    }
}

#[derive(Debug)]
struct InvalidSnailfishNumber;

impl FromStr for Number {
    type Err = InvalidSnailfishNumber;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        fn recursive(line: &str) -> (Number, usize) {
            let s = line.as_bytes();
            let (left, size_l) = if s[1] == b'[' {
                recursive(&line[1..])
            } else {
                (Number::Value(s[1] - b'0'), 1)
            };
            if s[size_l + 1] == b']' {
                (left, size_l)
            } else {
                let (right, size_r) = recursive(&line[size_l + 1..]);
                (
                    Number::Nested(Box::new(Pair { left, right })),
                    size_l + size_r + 3,
                )
            }
        }
        Ok(recursive(line).0)
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::Value(n) => write!(f, "{}", n),
            Number::Nested(p) => write!(f, "[{},{}]", p.left, p.right),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Pair {
    left: Number,
    right: Number,
}

impl Pair {
    fn distribute_left(&mut self, v: u8) {
        match &mut self.left {
            Number::Value(left) => *left += v,
            Number::Nested(p) => p.distribute_left(v),
        }
    }

    fn distribute_right(&mut self, v: u8) {
        match &mut self.right {
            Number::Value(right) => *right += v,
            Number::Nested(p) => p.distribute_right(v),
        }
    }

    fn explode(&mut self, level: usize) -> Option<(Option<u8>, Option<u8>)> {
        match (&mut self.left, &mut self.right) {
            (Number::Value(a), Number::Value(b)) => {
                if level >= 4 {
                    Some((Some(*a), Some(*b)))
                } else {
                    None
                }
            }
            (Number::Value(v), Number::Nested(right)) => match right.explode(level + 1) {
                Some((Some(frag), b)) => {
                    *v += frag;
                    if b.is_some() {
                        self.right = Number::Value(0);
                    }
                    Some((None, b))
                }
                x => x,
            },
            (Number::Nested(left), Number::Value(v)) => match left.explode(level + 1) {
                Some((b, Some(frag))) => {
                    *v += frag;
                    if b.is_some() {
                        self.left = Number::Value(0);
                    }
                    Some((b, None))
                }
                x => x,
            },
            (Number::Nested(left), Number::Nested(right)) => {
                if let Some((a, b)) = left.explode(level + 1) {
                    if let Some(b) = b {
                        right.distribute_left(b);
                        if a.is_some() {
                            self.left = Number::Value(0);
                        }
                    }
                    return Some((a, None));
                }
                if let Some((a, b)) = right.explode(level + 1) {
                    if let Some(a) = a {
                        left.distribute_right(a);
                        if b.is_some() {
                            self.right = Number::Value(0);
                        }
                    }
                    Some((None, b))
                } else {
                    None
                }
            }
        }
    }

    fn split(&mut self) -> bool {
        match &mut self.left {
            Number::Value(v) => {
                if *v >= 10 {
                    self.left = Number::Nested(Box::new(Pair {
                        left: Number::Value(*v / 2),
                        right: Number::Value(*v - *v / 2),
                    }));
                    return true;
                }
            }
            Number::Nested(p) => {
                if p.split() {
                    return true;
                }
            }
        }

        match &mut self.right {
            Number::Value(v) => {
                if *v >= 10 {
                    self.right = Number::Nested(Box::new(Pair {
                        left: Number::Value(*v / 2),
                        right: Number::Value(*v - *v / 2),
                    }));
                    true
                } else {
                    false
                }
            }
            Number::Nested(p) => p.split(),
        }
    }
}

fn parse(data: &str) -> Vec<Number> {
    data.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn main() {
    let data = std::fs::read_to_string("data/2021/day18").unwrap();
    let numbers = parse(&data);
    let mut n = numbers[0].clone();
    for rhs in &numbers[1..] {
        n = n + rhs.clone();
    }
    println!("day18 part1: {}", n.magnitude());

    let p2 = numbers
        .iter()
        .map(|a| numbers.iter().map(|b| (a.clone() + b.clone()).magnitude()))
        .flatten()
        .max()
        .unwrap();
    println!("day18 part2: {}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "[[[[4,3],4],4],[7,[[8,4],9]]]\n[1,1]";
        let mut numbers = parse(&data);
        let mut n = numbers.remove(0);
        for rhs in numbers {
            n = n + rhs;
        }
        let expected = Number::from_str("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").unwrap();
        assert_eq!(expected, n);
    }
}
