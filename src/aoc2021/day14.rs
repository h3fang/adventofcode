use std::collections::HashMap;

fn parse(data: &str) -> (HashMap<[u8; 2], usize>, HashMap<[u8; 2], u8>) {
    let mut lines = data.lines();
    let template = lines.next().unwrap().trim().as_bytes();
    lines.next();
    let mut rules = HashMap::new();
    for line in lines {
        let mut parts = line.trim().split(" -> ");
        let ab = parts.next().unwrap().as_bytes();
        let c = parts.next().unwrap().as_bytes()[0];
        rules.insert([ab[0], ab[1]], c);
    }
    let mut pairs: HashMap<[u8; 2], usize> = HashMap::new();
    for w in template.windows(2) {
        *pairs.entry([w[0], w[1]]).or_default() += 1;
    }
    pairs.insert([b'$', template[0]], 1);
    pairs.insert([template[template.len() - 1], b'$'], 1);
    (pairs, rules)
}

fn polymerization(
    (mut pairs, rules): (HashMap<[u8; 2], usize>, HashMap<[u8; 2], u8>),
    steps: usize,
) -> usize {
    for _ in 0..steps {
        let mut next = pairs.clone();
        for (ab, &c) in &rules {
            if let Some(n) = pairs.get(ab) {
                *next.entry(*ab).or_default() -= n;
                *next.entry([ab[0], c]).or_default() += n;
                *next.entry([c, ab[1]]).or_default() += n;
            }
        }
        pairs = next;
    }
    let mut count = [0usize; 26];
    for (ab, n) in pairs {
        for e in ab {
            if e.is_ascii_uppercase() {
                count[(e - b'A') as usize] += n;
            }
        }
    }
    let mut max = 0;
    let mut min = usize::MAX;
    for c in count {
        if c > 0 {
            max = max.max(c);
            min = min.min(c);
        }
    }

    (max - min) / 2
}

pub fn main() {
    let data = std::fs::read_to_string("data/2021/day14").unwrap();
    let input = parse(&data);
    println!("day14 part1: {}", polymerization(input.clone(), 10));
    println!("day14 part2: {}", polymerization(input, 40));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "NNCB

        CH -> B
        HH -> N
        CB -> H
        NH -> C
        HB -> C
        HC -> B
        HN -> C
        NN -> C
        BH -> H
        NC -> B
        NB -> B
        BN -> B
        BB -> N
        BC -> B
        CC -> N
        CN -> C";
        assert_eq!(1588, polymerization(parse(data), 10));
        assert_eq!(2188189693529, polymerization(parse(data), 40));
    }
}
