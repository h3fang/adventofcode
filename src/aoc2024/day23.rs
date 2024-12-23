use ahash::{HashMap, HashSet};

fn parse(input: &str) -> HashMap<u16, HashSet<u16>> {
    let mut g: HashMap<u16, HashSet<u16>> = HashMap::default();
    for line in input.trim().lines() {
        let (a, b) = line.split_once('-').unwrap();
        let (a, b) = (a.as_bytes(), b.as_bytes());
        let a = ((a[0] as u16) << 8) + a[1] as u16;
        let b = ((b[0] as u16) << 8) + b[1] as u16;
        g.entry(a).or_default().insert(b);
        g.entry(b).or_default().insert(a);
    }
    g
}

fn bron_kerbosch(
    g: &HashMap<u16, HashSet<u16>>,
    r: &mut HashSet<u16>,
    mut x: HashSet<u16>,
    mut p: HashSet<u16>,
    max: &mut HashSet<u16>,
) {
    if p.is_empty() && x.is_empty() {
        if max.len() < r.len() {
            *max = r.clone();
        }
        return;
    }
    while let Some(&v) = p.iter().next() {
        r.insert(v);
        let empty = HashSet::default();
        let n_v = g.get(&v).unwrap_or(&empty);
        let p1: HashSet<u16> = p.intersection(n_v).cloned().collect();
        if p1.len() + r.len() > max.len() {
            let x1 = x.intersection(n_v).cloned().collect();
            bron_kerbosch(g, r, x1, p1, max);
        }
        r.remove(&v);
        p.remove(&v);
        if p.len() + r.len() <= max.len() {
            return;
        }
        x.insert(v);
    }
}

fn part1(g: &HashMap<u16, HashSet<u16>>) -> usize {
    let mut sets = HashSet::default();
    for (&a, neighbors) in g {
        if (a / 256) != b't' as u16 {
            continue;
        }
        for (j, &b) in neighbors.iter().enumerate() {
            for &c in neighbors.iter().skip(j + 1) {
                if g.get(&b).is_some_and(|s| s.contains(&c)) {
                    let mut k = [a, b, c];
                    k.sort_unstable();
                    sets.insert(k);
                }
            }
        }
    }
    sets.len()
}

fn part2(g: &HashMap<u16, HashSet<u16>>) -> String {
    let mut max = HashSet::default();
    bron_kerbosch(
        g,
        &mut HashSet::default(),
        HashSet::default(),
        g.keys().cloned().collect(),
        &mut max,
    );
    let mut names: Vec<String> = max
        .into_iter()
        .map(|x| {
            let b = vec![(x / 256) as u8, (x % 256) as u8];
            String::from_utf8(b).unwrap()
        })
        .collect();
    names.sort_unstable();
    names.join(",")
}

pub fn main() {
    let input: String = std::fs::read_to_string("data/2024/day23").unwrap();
    let g = parse(&input);
    println!("part1: {}", part1(&g));
    println!("part2: {}", part2(&g));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = "
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";
        let g = parse(input);
        assert_eq!(7, part1(&g));
        assert_eq!("co,de,ka,ta", part2(&g));
    }
}
