fn parse(data: &str) -> Vec<&[u8]> {
    data.trim().lines().map(|line| line.as_bytes()).collect()
}

fn priority(item: u8) -> u8 {
    if item.is_ascii_lowercase() {
        (item - b'a') + 1
    } else {
        (item - b'A') + 27
    }
}

fn hash(items: &[u8]) -> u64 {
    items.iter().fold(0, |h, &item| h | (1 << priority(item)))
}

fn part1(rucksacks: &[&[u8]]) -> u32 {
    rucksacks
        .iter()
        .map(|r| {
            let n = r.len() / 2;
            let h = hash(&r[..n]) & hash(&r[n..]);
            h.trailing_zeros()
        })
        .sum()
}

fn part2(rucksacks: &[&[u8]]) -> u32 {
    rucksacks
        .chunks_exact(3)
        .map(|g| {
            let h = hash(g[0]) & hash(g[1]) & hash(g[2]);
            h.trailing_zeros()
        })
        .sum()
}

pub fn main() {
    let data = std::fs::read_to_string("data/2022/day3").unwrap();
    let rucksacks = parse(&data);
    println!("part1: {}", part1(&rucksacks));
    println!("part2: {}", part2(&rucksacks));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
            .to_string();
        let rucksacks = parse(&data);
        assert_eq!(157, part1(&rucksacks));
        assert_eq!(70, part2(&rucksacks));
    }
}
