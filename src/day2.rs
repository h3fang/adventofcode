use lazy_static::lazy_static;
use regex::Regex;

fn parse(s: &str) -> Option<(usize, usize, char, String)> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").unwrap();
    };
    if let Some(cap) = RE.captures_iter(s).next() {
        return Some((
            cap[1].parse().unwrap(),
            cap[2].parse().unwrap(),
            cap[3].chars().next().expect("should not happen"),
            cap[4].to_string(),
        ));
    }
    None
}

fn is_valid(min: &usize, max: &usize, c: &char, pwd: &str) -> bool {
    let n = pwd.chars().filter(|e| e == c).count() as usize;
    n >= *min && n <= *max
}

fn is_valid_part2(i1: &usize, i2: &usize, c: &char, pwd: &str) -> bool {
    let first = pwd.chars().nth(i1 - 1).expect("password too short") == *c;
    let second = pwd.chars().nth(i2 - 1).expect("password too short") == *c;
    (first || second) && !(first && second)
}

pub fn main() {
    let passwords: Vec<_> = include_str!("../data/day2")
        .lines()
        .filter_map(|s| parse(s))
        .collect();
    let n_valid = passwords
        .iter()
        .filter(|(min, max, c, pwd)| is_valid(min, max, c, pwd))
        .count();
    println!("day2 part1: {}", n_valid);

    let n_valid_part2 = passwords
        .iter()
        .filter(|(min, max, c, pwd)| is_valid_part2(min, max, c, pwd))
        .count();
    println!("day2 part2: {}", n_valid_part2);
}
