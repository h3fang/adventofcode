fn parse(input: &str) -> (Vec<&str>, Vec<&str>) {
    let (patterns, designs) = input.trim().split_once("\n\n").unwrap();
    let patterns = patterns.split(", ").collect();
    let designs = designs.lines().collect();
    (patterns, designs)
}

fn color_index(b: u8) -> usize {
    match b {
        b'w' => 0,
        b'u' => 1,
        b'b' => 2,
        b'r' => 3,
        b'g' => 4,
        _ => unreachable!(),
    }
}

#[derive(Default)]
struct Trie {
    next: [Option<Box<Trie>>; 5],
    is_end: bool,
}

impl Trie {
    fn insert(&mut self, w: &[u8]) {
        let mut t = self;
        for &b in w {
            let i = color_index(b);
            t = t.next[i].get_or_insert_default();
        }
        t.is_end = true;
    }
}

fn different_ways(design: &[u8], trie: &Trie) -> usize {
    let n = design.len();
    let mut f = vec![0; n + 1];
    f[0] = 1;
    for i in 0..n {
        let mut t = trie;
        for (j, &b) in design.iter().enumerate().skip(i) {
            if let Some(next) = &t.next[color_index(b)] {
                if next.is_end {
                    f[j + 1] += f[i];
                }
                t = next;
            } else {
                break;
            }
        }
    }
    f[n]
}

fn solve(patterns: &[&str], designs: &[&str]) -> (usize, usize) {
    let mut trie = Trie::default();
    for w in patterns {
        trie.insert(w.as_bytes());
    }
    let (mut p1, mut p2) = (0, 0);
    for d in designs {
        let w = different_ways(d.as_bytes(), &trie);
        p1 += usize::from(w > 0);
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
