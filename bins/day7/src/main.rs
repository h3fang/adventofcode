use anyhow::Result;
use regex::Regex;
use std::{
    collections::HashMap,
    env, fs,
    io::{self, BufRead},
};

fn split(re: &Regex, bags: &str) -> Vec<(usize, String)> {
    if bags == "no other bags" {
        return Vec::new();
    }

    bags.split(", ")
        .map(|s| {
            if let Some(cap) = re.captures(s) {
                (cap[1].parse().unwrap(), cap[2].to_string())
            } else {
                panic!("invalid format: {}", s);
            }
        })
        .collect()
}

fn contains_shiny_gold(
    bag: &str,
    map: &HashMap<String, Vec<(usize, String)>>,
    table: &mut HashMap<String, bool>,
) -> bool {
    if bag == "shiny gold" {
        return true;
    }
    if let Some(&r) = table.get(bag) {
        return r;
    }
    for (_, node) in map
        .get(bag)
        .unwrap_or_else(|| panic!("map doesn't contain: {}", bag))
    {
        let r = match table.get(node) {
            Some(&v) => v,
            None => contains_shiny_gold(node, map, table),
        };

        if r {
            table.insert(bag.to_string(), true);
            return true;
        }
    }
    table.insert(bag.to_string(), false);
    false
}

fn count_contained_bags(
    bag: &str,
    map: &HashMap<String, Vec<(usize, String)>>,
    table: &mut HashMap<String, usize>,
) -> usize {
    let n = map
        .get(bag)
        .unwrap_or_else(|| panic!("map doesn't contain: {}", bag))
        .iter()
        .map(|(n, node)| {
            n * match table.get(node) {
                Some(&v) => v,
                None => count_contained_bags(node, map, table),
            }
        })
        .sum::<usize>()
        + 1;
    table.insert(bag.to_string(), n);
    n
}

fn main() -> Result<()> {
    let mut args = env::args();
    let file_path = args
        .nth(1)
        .expect("not enough arguments, missing data file path");
    let data_file = fs::File::open(file_path)?;
    let r1 = Regex::new(r"^(\w+ \w+) bags contain (.+)\.$").unwrap();
    let r2 = Regex::new(r"(\d+) (\w+ \w+) bags?").unwrap();
    let mut map = HashMap::new();
    for line in io::BufReader::new(data_file).lines().flatten() {
        if let Some(cap) = r1.captures(&line) {
            let b1 = &cap[1];
            let b2 = split(&r2, &cap[2]);
            map.insert(b1.to_string(), b2);
        } else {
            panic!("invalid format: {}", line);
        }
    }

    // part 1
    let mut table = HashMap::new();
    let n = map
        .keys()
        .filter(|&k| k != "shiny gold" && contains_shiny_gold(k, &map, &mut table))
        .count();
    println!("n: {}", n);

    // part 2
    let mut table = HashMap::new();
    let n = count_contained_bags("shiny gold", &map, &mut table) - 1;
    println!("n part2: {}", n);
    Ok(())
}
