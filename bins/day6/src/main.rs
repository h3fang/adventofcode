use anyhow::Result;
use std::{
    collections::HashSet,
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

fn main() -> Result<()> {
    let stdin = io::stdin();
    let mut lines = Vec::new();
    let mut n = 0;
    let mut n_part2 = 0;
    for line in stdin.lock().lines().flatten() {
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
    println!("n: {}, n_part2: {}", n, n_part2);
    Ok(())
}
