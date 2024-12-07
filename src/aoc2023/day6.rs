fn parse(data: &str) -> (&str, &str) {
    data.trim().split_once('\n').unwrap()
}

fn part1((time, distance): (&str, &str)) -> u64 {
    let time = time.strip_prefix("Time:").unwrap();
    let distance = distance.strip_prefix("Distance:").unwrap();
    time.split_ascii_whitespace()
        .map(|t| t.parse().unwrap())
        .zip(
            distance
                .split_ascii_whitespace()
                .map(|t| t.parse().unwrap()),
        )
        .map(|(time, distance)| (1..time).filter(|t| t * (time - t) > distance).count() as u64)
        .product()
}

fn part2((time, distance): (&str, &str)) -> u64 {
    let time = time
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();
    let distance = distance
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();
    let (t, d) = (time as f64, distance as f64);
    let det = (t * t - 4.0 * d).sqrt();
    let (a, b) = (((t - det) * 0.5) as u64, ((t + det) * 0.5) as u64);
    let a = if a * (time - a) > distance { a } else { a + 1 };
    let b = if (b + 1) * (time - b - 1) > distance {
        b + 1
    } else {
        b
    };
    b - a + 1
}

pub fn main() {
    let data = std::fs::read_to_string("data/2023/day6").unwrap();
    let input = parse(&data);
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "
Time:      7  15   30
Distance:  9  40  200";
        let input = parse(data);
        assert_eq!(288, part1(input));
        assert_eq!(71503, part2(input));
    }
}
