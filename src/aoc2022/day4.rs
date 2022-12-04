type Range = (u8, u8);

fn parse_range(input: &str) -> Range {
    let (left, right) = input.split_once('-').unwrap();
    let left = left.parse().unwrap();
    let right = right.parse().unwrap();
    (left, right)
}

fn parse(data: &str) -> Vec<(Range, Range)> {
    data.trim()
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(',').unwrap();
            let a = parse_range(a);
            let b = parse_range(b);
            (a, b)
        })
        .collect()
}

fn part1(assignments: &[(Range, Range)]) -> usize {
    assignments
        .iter()
        .filter(|(a, b)| (a.0 <= b.0 && a.1 >= b.1) || (b.0 <= a.0 && b.1 >= a.1))
        .count()
}

fn part2(assignments: &[(Range, Range)]) -> usize {
    assignments
        .iter()
        .filter(|(a, b)| !(a.1 < b.0 || b.1 < a.0))
        .count()
}

pub fn main() {
    let data = std::fs::read_to_string("data/2022/day4").unwrap();
    let assignments = parse(&data);
    println!("part1: {}", part1(&assignments));
    println!("part2: {}", part2(&assignments));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        let assignments = parse(&data);
        assert_eq!(2, part1(&assignments));
        assert_eq!(4, part2(&assignments));
    }
}
