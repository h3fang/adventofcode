use anyhow::Result;
use regex::Regex;
use std::io::{self, BufRead};

fn parse(s: String) -> Result<Option<(usize, usize, char, String)>> {
    let re = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$")?;
    if let Some(cap) = re.captures_iter(&s).next() {
        return Ok(Some((
            cap[1].parse()?,
            cap[2].parse()?,
            cap[3].chars().next().expect("should not happen"),
            cap[4].to_string(),
        )));
    }
    Ok(None)
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

fn main() -> Result<()> {
    let stdin = io::stdin();
    let passwords: Vec<_> = stdin
        .lock()
        .lines()
        .flatten()
        .filter_map(|s| parse(s).expect("invalid line"))
        .collect();
    let n_valid = passwords
        .iter()
        .filter(|(min, max, c, pwd)| is_valid(min, max, c, pwd))
        .count();
    println!("valid: {}", n_valid);

    let n_valid_part2 = passwords
        .iter()
        .filter(|(min, max, c, pwd)| is_valid_part2(min, max, c, pwd))
        .count();
    println!("valid part2: {}", n_valid_part2);
    Ok(())
}
