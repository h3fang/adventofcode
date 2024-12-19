use ahash::{HashMap, HashMapExt, HashSet};

fn parse(input: &str) -> (Vec<&str>, Vec<&str>) {
    let (patterns, designs) = input.trim().split_once("\n\n").unwrap();
    let patterns = patterns.split(", ").collect();
    let designs = designs.lines().collect();
    (patterns, designs)
}

fn different_ways<'a>(
    design: &'a [u8],
    patterns: &HashSet<&'a [u8]>,
    cache: &mut HashMap<&'a [u8], usize>,
) -> usize {
    if design.is_empty() {
        return 1;
    }
    if let Some(&r) = cache.get(design) {
        return r;
    }
    let mut result = 0;
    for i in 1..=design.len() {
        if patterns.contains(&design[..i]) {
            result += different_ways(&design[i..], patterns, cache);
        }
    }
    cache.insert(design, result);
    result
}

fn solve(patterns: &[&str], designs: &[&str]) -> (usize, usize) {
    let patterns: HashSet<&[u8]> = patterns.iter().map(|p| p.as_bytes()).collect();
    let mut cache = HashMap::with_capacity(designs.len() * patterns.len());
    let (mut p1, mut p2) = (0, 0);
    for d in designs {
        let w = different_ways(d.as_bytes(), &patterns, &mut cache);
        if w > 0 {
            p1 += 1;
        }
        p2 += w;
    }
    (p1, p2)
}

pub fn main() {
    let input: String = std::fs::read_to_string("data/2024/day19").unwrap();
    let (patterns, designs) = parse(&input);
    let (p1, p2) = solve(&patterns, &designs);
    println!("part1: {p1}");
    println!("part2: {p2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = "
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
        let (patterns, designs) = parse(input);
        assert_eq!((6, 16), solve(&patterns, &designs));
    }
}
