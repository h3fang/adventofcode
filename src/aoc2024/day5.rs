use ahash::{HashMap, HashMapExt, HashSet};

fn parse(input: &str) -> (HashMap<u8, HashSet<u8>>, Vec<Vec<u8>>) {
    let (rules, update) = input.trim().split_once("\n\n").unwrap();
    let mut g: HashMap<u8, HashSet<u8>> = HashMap::with_capacity(100);
    for rule in rules.lines() {
        let (a, b) = rule.split_once('|').unwrap();
        let a: u8 = a.parse().unwrap();
        let b: u8 = b.parse().unwrap();
        g.entry(b).or_default().insert(a);
    }
    let updates = update
        .lines()
        .map(|line| line.split(',').map(|e| e.parse().unwrap()).collect())
        .collect();
    (g, updates)
}

fn part1(g: &HashMap<u8, HashSet<u8>>, updates: &[Vec<u8>]) -> i32 {
    updates
        .iter()
        .filter(|u| {
            u.iter().enumerate().all(|(i, a)| {
                u.iter().skip(i + 1).all(|b| {
                    if let Some(rules) = g.get(a) {
                        !rules.contains(b)
                    } else {
                        true
                    }
                })
            })
        })
        .map(|u| u[u.len() / 2] as i32)
        .sum()
}

fn part2(g: &HashMap<u8, HashSet<u8>>, updates: &mut [Vec<u8>]) -> i32 {
    updates
        .iter_mut()
        .map(|u| {
            let n = u.len();
            let mut valid = true;
            for i in 0..n - 1 {
                for j in i + 1..n {
                    let (a, b) = (u[i], u[j]);
                    if let Some(rules) = g.get(&a)
                        && rules.contains(&b)
                    {
                        valid = false;
                        u.swap(i, j);
                    }
                }
            }

            if !valid { u[u.len() / 2] as i32 } else { 0 }
        })
        .sum()
}

pub fn main() {
    let input: String = std::fs::read_to_string("data/2024/day5").unwrap();
    let (g, mut updates) = parse(&input);
    println!("part1: {}", part1(&g, &updates));
    println!("part2: {}", part2(&g, &mut updates));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = "
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        let (g, mut updates) = parse(input);
        assert_eq!(143, part1(&g, &updates));
        assert_eq!(123, part2(&g, &mut updates));
    }

    #[test]
    fn case2() {
        let input: String = std::fs::read_to_string("data/2024/day5").unwrap();
        let (g, mut updates) = parse(&input);
        assert_eq!(5991, part1(&g, &updates));
        assert_eq!(5479, part2(&g, &mut updates));
    }
}
