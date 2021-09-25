use anyhow::Result;
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::HashMap,
    fs,
    io::{self, BufRead},
};

const KEYS: &[&str] = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

struct Passport {
    fields: HashMap<String, String>,
}

impl Passport {
    fn from_lines(lines: &[String]) -> Self {
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
                        lazy_static! {
                            static ref RE: Regex = Regex::new(r"^(\d+)(in|cm)$").unwrap();
                        }
                        let mut caps = RE.captures_iter(value);
                        if let Some(cap) = caps.next() {
                            let n = cap[1].parse::<u32>().unwrap();
                            (&cap[2] == "cm" && n >= 150 && n <= 193)
                                || (&cap[2] == "in" && n >= 59 && n <= 76)
                        } else {
                            false
                        }
                    }
                    "hcl" => {
                        lazy_static! {
                            static ref RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
                        }
                        RE.is_match(value)
                    }
                    "ecl" => {
                        lazy_static! {
                            static ref RE: Regex =
                                Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap();
                        }
                        RE.is_match(value)
                    }
                    "pid" => {
                        lazy_static! {
                            static ref RE: Regex = Regex::new(r"^\d{9}$").unwrap();
                        }
                        RE.is_match(value)
                    }
                    _ => true,
                }
            } else {
                false
            }
        })
    }
}

pub fn main(file_path: &str) -> Result<()> {
    let data_file = fs::File::open(file_path)?;
    let mut lines = Vec::new();
    let mut valid = 0;
    let mut valid_part2 = 0;
    for line in io::BufReader::new(data_file).lines().flatten() {
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
    println!("day4 part1: {}\nday4 part2: {}", valid, valid_part2);
    Ok(())
}
