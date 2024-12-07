fn parse(data: &str) -> Vec<Vec<i32>> {
    data.lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|e| e.parse().unwrap())
                .collect()
        })
        .collect()
}

fn is_inc(level: &[i32]) -> bool {
    level.windows(2).all(|w| (1..=3).contains(&(w[1] - w[0])))
}

fn is_dec(level: &[i32]) -> bool {
    level.windows(2).all(|w| (1..=3).contains(&(w[0] - w[1])))
}

fn part1(levels: &[Vec<i32>]) -> usize {
    levels
        .iter()
        .filter(|level| is_dec(level) || is_inc(level))
        .count()
}

fn part2(levels: &[Vec<i32>]) -> usize {
    levels
        .iter()
        .filter(|level| {
            if is_dec(level) || is_inc(level) {
                return true;
            }
            for i in 0..level.len() {
                let filterd: Vec<_> = level
                    .iter()
                    .enumerate()
                    .filter_map(|(j, &x)| if j == i { None } else { Some(x) })
                    .collect();
                if is_dec(&filterd) || is_inc(&filterd) {
                    return true;
                }
            }
            false
        })
        .count()
}

pub fn main() {
    let data = std::fs::read_to_string("data/2024/day2").unwrap();
    let levels = parse(&data);
    println!("part1: {}", part1(&levels));
    println!("part2: {}", part2(&levels));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        let levels = parse(data);
        assert_eq!(2, part1(&levels));
        assert_eq!(4, part2(&levels));
    }
}
