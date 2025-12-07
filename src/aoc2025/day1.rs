fn parse(data: &str) -> Vec<i32> {
    data.lines()
        .map(|line| {
            let dir = match line.as_bytes()[0] {
                b'L' => -1,
                b'R' => 1,
                _ => unreachable!(),
            };
            let dist = line[1..].parse::<i32>().unwrap();
            dir * dist
        })
        .collect()
}

fn part1(rotations: &[i32]) -> i32 {
    let (mut curr, mut zeros) = (50, 0);
    for r in rotations {
        curr = (curr + r + 100) % 100;
        if curr == 0 {
            zeros += 1;
        }
    }
    zeros
}

fn part2(rotations: &[i32]) -> i32 {
    let (mut curr, mut zeros) = (50, 0);
    for r in rotations {
        zeros += r.abs() / 100;
        let next = curr + r % 100;
        if (curr > 0 && next <= 0) || next >= 100 {
            zeros += 1;
        }
        curr = (next + 100) % 100;
    }
    zeros
}

pub fn main() {
    let data = std::fs::read_to_string("data/2025/day1").unwrap();
    let rotations = parse(&data);
    println!("part1: {}", part1(&rotations));
    println!("part2: {}", part2(&rotations));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        let rotations = parse(data.trim());
        assert_eq!(3, part1(&rotations));
        assert_eq!(6, part2(&rotations));
    }
}
