use anyhow::Result;
use std::{
    collections::HashSet,
    fs,
    io::{self, BufRead},
};

fn count(lines: &[String]) -> usize {
    let chars = lines
        .iter()
        .flat_map(|line| line.chars())
        .collect::<HashSet<_>>();
    chars.len()
}

fn count_part2(lines: &[String]) -> usize {
    let shortest = lines.iter().map(|line| line.len()).min().unwrap();
    let shortest = lines.iter().find(|line| line.len() == shortest).unwrap();
    let chars = lines
        .iter()
        .map(|s| s.chars().collect::<HashSet<_>>())
        .collect::<Vec<_>>();
    shortest
        .chars()
        .filter(|c| chars.iter().all(|ch| ch.contains(c)))
        .count()
}

pub fn main(file_path: &str) -> Result<()> {
    let data_file = fs::File::open(file_path)?;
    let mut lines = Vec::new();
    let mut n = 0;
    let mut n_part2 = 0;
    for line in io::BufReader::new(data_file).lines().flatten() {
        if line.is_empty() {
            n += count(&lines);
            n_part2 += count_part2(&lines);
            lines.clear();
        } else {
            lines.push(line);
        }
    }
    if !lines.is_empty() {
        n += count(&lines);
        n_part2 += count_part2(&lines);
    }
    println!("day6 part1: {}\nday6 part2: {}", n, n_part2);
    Ok(())
}
