use std::collections::{HashMap, HashSet};

fn is_lowercase(s: &str) -> bool {
    s.as_bytes()[0].is_ascii_lowercase()
}

fn dfs<'a>(
    g: &HashMap<&'a str, Vec<&'a str>>,
    visited: &mut HashSet<&'a str>,
    can_revisit: bool,
    curr: &'a str,
    result: &mut usize,
) {
    if curr == "end" {
        *result += 1;
        return;
    }

    let mut inserted = false;
    if is_lowercase(curr) {
        inserted = visited.insert(curr);
    }

    if let Some(children) = g.get(&curr) {
        for &child in children {
            if child == "start" {
                continue;
            }
            if !visited.contains(child) {
                dfs(g, visited, can_revisit, child, result);
            } else if can_revisit {
                dfs(g, visited, false, child, result);
            }
        }
    }

    if inserted {
        visited.remove(curr);
    }
}

fn part1(g: &HashMap<&str, Vec<&str>>) -> usize {
    let mut visited = HashSet::new();
    let mut result = 0;
    dfs(g, &mut visited, false, "start", &mut result);
    result
}

fn part2(g: &HashMap<&str, Vec<&str>>) -> usize {
    let mut visited = HashSet::new();
    let mut result = 0;
    dfs(g, &mut visited, true, "start", &mut result);
    result
}

pub fn main() {
    let data = std::fs::read_to_string("data/2021/day12").unwrap();

    let edges = data
        .lines()
        .map(|s| s.trim().split('-').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut g: HashMap<&str, Vec<&str>> = HashMap::new();
    for e in edges {
        g.entry(e[0]).or_default().push(e[1]);
        g.entry(e[1]).or_default().push(e[0]);
    }

    println!("day12 part1: {}", part1(&g));
    println!("day12 part2: {}", part2(&g));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "start-A
        start-b
        A-c
        A-b
        b-d
        A-end
        b-end";
        let edges = data
            .lines()
            .map(|s| s.trim().split('-').collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut g: HashMap<&str, Vec<&str>> = HashMap::new();
        for e in edges {
            g.entry(e[0]).or_default().push(e[1]);
            g.entry(e[1]).or_default().push(e[0]);
        }

        assert_eq!(10, part1(&g));
        assert_eq!(36, part2(&g));
    }
}
