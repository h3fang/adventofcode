use hashbrown::{HashMap, HashSet};

fn part1(g: &HashMap<&str, HashSet<&str>>) -> usize {
    fn dfs(g: &HashMap<&str, HashSet<&str>>, s: &str, result: &mut usize) -> usize {
        let mut r = 0;
        if let Some(children) = g.get(&s) {
            for &child in children {
                r += dfs(g, child, result) + 1;
            }
        }
        *result += r;
        r
    }
    let mut result = 0;
    dfs(g, "COM", &mut result);
    result
}

fn part2(g: &HashMap<&str, HashSet<&str>>) -> usize {
    fn dfs(g: &HashMap<&str, HashSet<&str>>, c: &str, result: &mut usize) -> (usize, usize) {
        let mut r = (usize::MAX, usize::MAX);
        if let Some(children) = g.get(&c) {
            for &child in children {
                if child == "YOU" {
                    r.0 = 0;
                } else if child == "SAN" {
                    r.1 = 0;
                } else {
                    let dist = dfs(g, child, result);
                    if r.0 == usize::MAX && dist.0 < usize::MAX {
                        r.0 = dist.0 + 1;
                    }
                    if r.1 == usize::MAX && dist.1 < usize::MAX {
                        r.1 = dist.1 + 1;
                    }
                }
            }
        }

        if *result == usize::MAX && r.0 < usize::MAX && r.1 < usize::MAX {
            *result = r.0 + r.1;
        }
        r
    }
    let mut result = usize::MAX;
    dfs(g, "COM", &mut result);
    result
}

pub fn main() {
    let data = std::fs::read_to_string("data/2019/day6").unwrap();
    let mut g: HashMap<&str, HashSet<&str>> = HashMap::new();
    data.lines().for_each(|s| {
        let parts = s.split(')').collect::<Vec<_>>();
        g.entry(parts[0]).or_default().insert(parts[1]);
    });

    println!("day6 part1: {}", part1(&g));

    println!("day6 part2: {}", part2(&g));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "COM)B
        B)C
        C)D
        D)E
        E)F
        B)G
        G)H
        D)I
        E)J
        J)K
        K)L
        K)YOU
        I)SAN";

        let mut g: HashMap<&str, HashSet<&str>> = HashMap::new();
        data.lines().for_each(|s| {
            let parts = s.trim().split(')').collect::<Vec<_>>();
            g.entry(parts[0]).or_default().insert(parts[1]);
        });

        assert_eq!(4, part2(&g));
    }
}
