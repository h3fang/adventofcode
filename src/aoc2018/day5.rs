fn react(mut p: Vec<u8>) -> String {
    const DIFF: i16 = (b'a' as i16 - b'A' as i16).abs();
    loop {
        let mut next = Vec::with_capacity(p.len());
        let mut i = 0;
        while i < p.len() {
            if i + 1 < p.len() {
                if (p[i] as i16 - p[i + 1] as i16).abs() == DIFF {
                    i += 2;
                } else {
                    next.push(p[i]);
                    i += 1;
                }
            } else {
                next.push(p[i]);
                i += 1;
            }
        }
        if next.len() == p.len() {
            return unsafe { String::from_utf8_unchecked(p) };
        }
        p = next;
    }
}

fn part1(polymer: &str) -> usize {
    polymer.len()
}

fn part2(polymer: &str) -> usize {
    (b'a'..=b'z')
        .map(|c| {
            let p = polymer
                .as_bytes()
                .iter()
                .filter(|&&b| b.to_ascii_lowercase() != c)
                .cloned()
                .collect::<Vec<_>>();
            react(p).len()
        })
        .min()
        .unwrap()
}

pub fn main() {
    let polymer = std::fs::read_to_string("data/2018/day5").unwrap();
    let reducted = react(polymer.trim().as_bytes().to_vec());
    println!("part1: {}", part1(&reducted));
    println!("part2: {}", part2(&reducted));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let polymer = "dabAcCaCBAcCcaDA".to_string();
        assert_eq!("dabCBAcaDA", react(polymer.as_bytes().to_vec()));
        assert_eq!(4, part2(&polymer));
    }
}
