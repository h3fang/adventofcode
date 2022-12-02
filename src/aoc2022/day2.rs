fn parse(data: &str) -> Vec<(u8, u8)> {
    data.trim()
        .lines()
        .map(|e| {
            let e = e.as_bytes();
            (e[0] - b'A', e[2] - b'X')
        })
        .collect()
}

fn part1(strategy: &[(u8, u8)]) -> u32 {
    strategy
        .iter()
        .map(|&(o, m)| {
            let r = m
                + 1
                + if o == (m + 1) % 3 {
                    0
                } else if o == m {
                    3
                } else {
                    6
                };
            r as u32
        })
        .sum()
}

fn part2(strategy: &[(u8, u8)]) -> u32 {
    strategy
        .iter()
        .map(|&(opponent, outcome)| {
            let myself = if outcome == 0 {
                (opponent + 3 - 1) % 3
            } else if outcome == 1 {
                opponent
            } else {
                (opponent + 1) % 3
            };
            (myself + 1 + outcome * 3) as u32
        })
        .sum()
}

pub fn main() {
    let data = std::fs::read_to_string("data/2022/day2").unwrap();
    let strategy = parse(&data);
    println!("part1: {}", part1(&strategy));
    println!("part2: {}", part2(&strategy));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "
A Y
B X
C Z";
        let strategy = parse(data);
        assert_eq!(15, part1(&strategy));
        assert_eq!(12, part2(&strategy));
    }
}
