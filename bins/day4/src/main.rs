use anyhow::Result;
use regex::Regex;
use std::{
    collections::HashMap,
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
                        let re = Regex::new(r"^(\d+)(in|cm)$").unwrap();
                        let mut caps = re.captures_iter(value);
                        if let Some(cap) = caps.next() {
                            let n = cap[1].parse::<u32>().unwrap();
                            (&cap[2] == "cm" && n >= 150 && n <= 193)
                                || (&cap[2] == "in" && n >= 59 && n <= 76)
                        } else {
                            false
                        }
                    }
                    "hcl" => {
                        let re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
                        re.is_match(value)
                    }
                    "ecl" => {
                        let re = Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap();
                        re.is_match(value)
                    }
                    "pid" => {
                        let re = Regex::new(r"^\d{9}$").unwrap();
                        re.is_match(value)
                    }
                    _ => true,
                }
            } else {
                false
            }
        })
    }
}

fn main() -> Result<()> {
    let stdin = io::stdin();
    let mut lines = Vec::new();
    let mut valid = 0;
    let mut valid_part2 = 0;
    for line in stdin.lock().lines().flatten() {
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
    println!("valid: {}, valid_part2: {}", valid, valid_part2);
    Ok(())
}
