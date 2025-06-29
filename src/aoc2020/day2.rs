use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, anychar, char, digit1},
    combinator::{eof, map_res, recognize},
    IResult, Parser,
};

fn parse_nom(s: &str) -> IResult<&str, (usize, usize, char, String)> {
    fn parse_usize(input: &str) -> IResult<&str, usize> {
        map_res(recognize(digit1), str::parse).parse(input)
    }
    let (_, (min, _, max, _, c, _, pwd, _)) = (
        parse_usize,
        char('-'),
        parse_usize,
        char(' '),
        anychar,
        tag(": "),
        alpha1,
        eof,
    )
        .parse(s)?;
    Ok(("", (min, max, c, pwd.to_string())))
}

fn is_valid(min: &usize, max: &usize, c: &char, pwd: &str) -> bool {
    let n = pwd.chars().filter(|e| e == c).count();
    n >= *min && n <= *max
}

fn is_valid_part2(i1: &usize, i2: &usize, c: &char, pwd: &str) -> bool {
    let first = pwd.chars().nth(i1 - 1).expect("password too short") == *c;
    let second = pwd.chars().nth(i2 - 1).expect("password too short") == *c;
    (first || second) && !(first && second)
}

pub fn main() {
    let passwords: Vec<_> = std::fs::read_to_string("data/2020/day2")
        .unwrap()
        .lines()
        .map(|line| parse_nom(line).map(|r| r.1).unwrap())
        .collect();
    let n_valid = passwords
        .iter()
        .filter(|(min, max, c, pwd)| is_valid(min, max, c, pwd))
        .count();
    println!("day2 part1: {n_valid}");

    let n_valid_part2 = passwords
        .iter()
        .filter(|(min, max, c, pwd)| is_valid_part2(min, max, c, pwd))
        .count();
    println!("day2 part2: {n_valid_part2}");
}
