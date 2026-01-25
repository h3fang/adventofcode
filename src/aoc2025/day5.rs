fn parse(data: &str) -> (Vec<(usize, usize)>, Vec<usize>) {
    let (p1, p2) = data.trim().split_once("\n\n").unwrap();

    let ranges = p1
        .trim()
        .lines()
        .map(|line| {
            let (start, end) = line.split_once("-").unwrap();
            let start = start.parse().unwrap();
            let end = end.parse().unwrap();
            (start, end)
        })
        .collect();

    let ids = p2
        .trim()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    (ranges, ids)
}

fn part1(ranges: &[(usize, usize)], ids: &[usize]) -> usize {
    ids.iter()
        .filter(|&id| {
            ranges
                .iter()
                .any(|&(start, end)| (start..=end).contains(id))
        })
        .count()
}

fn part2(mut ranges: Vec<(usize, usize)>) -> usize {
    ranges.sort_unstable();
    let mut prev = ranges[0];
    let mut ans = 0;
    for r in ranges.into_iter().skip(1) {
        if r.0 > prev.1 {
            ans += prev.1 - prev.0 + 1;
            prev = r;
        } else {
            prev.1 = prev.1.max(r.1);
        }
    }
    ans + prev.1 - prev.0 + 1
}

pub fn main() {
    let data = std::fs::read_to_string("data/2025/day5").unwrap();
    let (ranges, ids) = parse(&data);
    println!("part1: {}", part1(&ranges, &ids));
    println!("part2: {}", part2(ranges));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "
3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        let (ranges, ids) = parse(data);
        assert_eq!(3, part1(&ranges, &ids));
        assert_eq!(14, part2(ranges));
    }
}
