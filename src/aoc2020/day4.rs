use ahash::AHashMap as HashMap;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, hex_digit1},
    combinator::{eof, map_res, recognize},
    IResult, Parser,
};

const KEYS: &[&str] = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

fn parse_u32(input: &str) -> IResult<&str, u32> {
    map_res(recognize(digit1), str::parse).parse(input)
}

fn parse_hcl(input: &str) -> IResult<&str, (&str, &str, &str)> {
    (tag("#"), hex_digit1, eof).parse(input)
}

fn parse_ecl(input: &str) -> IResult<&str, (&str, &str)> {
    let colors = alt((
        tag("amb"),
        tag("blu"),
        tag("brn"),
        tag("gry"),
        tag("grn"),
        tag("hzl"),
        tag("oth"),
    ));
    (colors, eof).parse(input)
}

fn parse_pid(input: &str) -> IResult<&str, (&str, &str)> {
    (recognize(digit1), eof).parse(input)
}

struct Passport {
    fields: HashMap<String, String>,
}

impl Passport {
    fn from_lines(lines: &[&str]) -> Self {
        let mut fields = HashMap::new();
        for line in lines {
            line.split(' ').for_each(|pattern| {
                let mut tokens = pattern.split(':');
                if let Some(key) = tokens.next() {
                    if let Some(value) = tokens.next() {
                        fields.insert(key.to_string(), value.to_string());
                    }
                }
            });
        }
        Self { fields }
    }

    fn is_valid(&self) -> bool {
        KEYS.iter().all(|&key| self.fields.contains_key(key))
    }

    fn is_valid_part2(&self) -> bool {
        KEYS.iter().all(|&key| {
            if let Some(value) = self.fields.get(key) {
                match key {
                    "byr" => {
                        if let Ok(yr) = value.parse::<u32>() {
                            (1920..=2002).contains(&yr)
                        } else {
                            false
                        }
                    }
                    "iyr" => {
                        if let Ok(yr) = value.parse::<u32>() {
                            (2010..=2020).contains(&yr)
                        } else {
                            false
                        }
                    }
                    "eyr" => {
                        if let Ok(yr) = value.parse::<u32>() {
                            (2020..=2030).contains(&yr)
                        } else {
                            false
                        }
                    }
                    "hgt" => {
                        if let Ok((_, (v, u, _))) =
                            (parse_u32, alt((tag("in"), tag("cm"))), eof).parse(value)
                        {
                            (u == "cm" && (150..=193).contains(&v))
                                || (u == "in" && (59..=76).contains(&v))
                        } else {
                            false
                        }
                    }
                    "hcl" => {
                        if let Ok((_, (_, hex, _))) = parse_hcl(value) {
                            hex.len() == 6
                                && hex
                                    .chars()
                                    .all(|c| c.is_ascii_digit() || c.is_ascii_lowercase())
                        } else {
                            false
                        }
                    }
                    "ecl" => parse_ecl(value).is_ok(),
                    "pid" => {
                        if let Ok((_, (pid, _))) = parse_pid(value) {
                            pid.len() == 9
                        } else {
                            false
                        }
                    }
                    _ => true,
                }
            } else {
                false
            }
        })
    }
}

pub fn main() {
    let mut lines = Vec::new();
    let mut valid = 0;
    let mut valid_part2 = 0;
    for line in std::fs::read_to_string("data/2020/day4").unwrap().lines() {
        if line.is_empty() {
            let p = Passport::from_lines(&lines);
            if p.is_valid() {
                valid += 1;
            }
            if p.is_valid_part2() {
                valid_part2 += 1;
            }
            lines.clear();
        } else {
            lines.push(line);
        }
    }
    println!("day4 part1: {valid}\nday4 part2: {valid_part2}");
}
