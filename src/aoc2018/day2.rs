fn part1(ids: &[&str]) -> usize {
    let mut two = 0;
    let mut three = 0;
    for id in ids {
        let mut f = [0; 26];
        for b in id.as_bytes() {
            f[(b - b'a') as usize] += 1;
        }
        if f.iter().any(|e| *e == 2) {
            two += 1;
        }
        if f.iter().any(|e| *e == 3) {
            three += 1;
        }
    }
    two * three
}

fn part2(ids: &[&str]) -> String {
    for (i, a) in ids.iter().enumerate() {
        for b in ids.iter().skip(i + 1) {
            let mut differ = 0;
            let mut pos = 0;
            for (j, (&c1, &c2)) in a.as_bytes().iter().zip(b.as_bytes()).enumerate() {
                if c1 != c2 {
                    differ += 1;
                    pos = j;
                    if differ > 1 {
                        break;
                    }
                }
            }
            if differ == 1 {
                let mut s = a.to_string();
                s.remove(pos);
                return s;
            }
        }
    }
    unreachable!()
}

pub fn main() {
    let data = std::fs::read_to_string("data/2018/day2").unwrap();
    let ids: Vec<&str> = data.lines().collect();
    println!("part1: {}", part1(&ids));
    println!("part2: {}", part2(&ids));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz";
        let ids: Vec<&str> = data.lines().collect();
        assert_eq!("fgij", part2(&ids));
    }
}
