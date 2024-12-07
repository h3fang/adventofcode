use ahash::AHashMap as HashMap;

type Input = (u8, u8, HashMap<[u8; 2], usize>, HashMap<[u8; 2], u8>);
fn parse(data: &str) -> Input {
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
    let first = template[0];
    let last = *template.last().unwrap();
    let mut pairs: HashMap<[u8; 2], usize> = HashMap::new();
    for w in template.windows(2) {
        *pairs.entry([w[0], w[1]]).or_default() += 1;
    }
    (first, last, pairs, rules)
}

fn polymerization(
    first: u8,
    last: u8,
    mut pairs: HashMap<[u8; 2], usize>,
    rules: &HashMap<[u8; 2], u8>,
    steps: usize,
) -> usize {
    for _ in 0..steps {
        let mut next = pairs.clone();
        for (ab, &c) in rules {
            if let Some(n) = pairs.get(ab) {
                *next.entry(*ab).or_default() -= n;
                *next.entry([ab[0], c]).or_default() += n;
                *next.entry([c, ab[1]]).or_default() += n;
            }
        }
        pairs = next;
    }
    let mut count = [0usize; 26];
    count[(first - b'A') as usize] = 1;
    count[(last - b'A') as usize] = 1;
    for (ab, n) in pairs {
        for e in ab {
            count[(e - b'A') as usize] += n;
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
    let (first, last, pairs, rules) = parse(&data);
    println!(
        "day14 part1: {}",
        polymerization(first, last, pairs.clone(), &rules, 10)
    );
    println!(
        "day14 part2: {}",
        polymerization(first, last, pairs, &rules, 10)
    );
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
        let (first, last, pairs, rules) = parse(data);
        assert_eq!(1588, polymerization(first, last, pairs.clone(), &rules, 10));
        assert_eq!(
            2188189693529,
            polymerization(first, last, pairs, &rules, 40)
        );
    }
}
