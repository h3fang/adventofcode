use ahash::HashSet;

/// reverse engineered solution, dependent on input
fn solve() -> (u64, u64) {
    let mut p1 = 0;
    let mut p2 = 0;
    let mut seen = HashSet::default();
    let mut r = [0u64; 6];
    loop {
        r[2] = r[3] | 65536;
        r[3] = 14070682;
        while r[2] > 0 {
            r[3] = (((r[3] + (r[2] & 0xff)) & 0xffffff) * 65899) & 0xffffff;
            r[2] /= 256;
        }
        if seen.is_empty() {
            p1 = r[3];
        }
        if !seen.insert(r[3]) {
            return (p1, p2);
        }
        p2 = r[3];
    }
}

pub fn main() {
    let (p1, p2) = solve();
    println!("part1: {p1}");
    println!("part2: {p2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let (p1, p2) = solve();
        assert_eq!(6132825, p1);
        assert_eq!(8307757, p2);
    }
}
