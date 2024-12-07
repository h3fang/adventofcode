use ahash::HashSet;

fn parse(data: &str) -> Vec<(u8, u32)> {
    data.trim()
        .lines()
        .map(|line| {
            let d = line.as_bytes()[0];
            let n = line[2..].parse().unwrap();
            (d, n)
        })
        .collect()
}

fn dir(d: u8) -> (i32, i32) {
    match d {
        b'R' => (1, 0),
        b'L' => (-1, 0),
        b'U' => (0, 1),
        b'D' => (0, -1),
        _ => unreachable!(),
    }
}

fn move_tail(h: (i32, i32), mut t: (i32, i32)) -> (i32, i32) {
    let (dx, dy) = (h.0 - t.0, h.1 - t.1);
    if !(dx.abs() <= 1 && dy.abs() <= 1) {
        t.0 += dx.signum();
        t.1 += dy.signum();
    }
    t
}

fn part1(motions: &[(u8, u32)]) -> usize {
    let mut h = (0, 0);
    let mut t = (0, 0);
    let mut visited = HashSet::default();
    visited.insert(t);
    for &(d, n) in motions {
        let (dx, dy) = dir(d);
        for _ in 0..n {
            h = (h.0 + dx, h.1 + dy);
            t = move_tail(h, t);
            visited.insert(t);
        }
    }
    visited.len()
}

fn part2(motions: &[(u8, u32)]) -> usize {
    let mut knots = [(0, 0); 10];
    let mut visited = HashSet::default();
    visited.insert(knots[9]);
    for &(d, n) in motions {
        let (dx, dy) = dir(d);
        for _ in 0..n {
            knots[0] = (knots[0].0 + dx, knots[0].1 + dy);
            for i in 0..9 {
                knots[i + 1] = move_tail(knots[i], knots[i + 1]);
            }
            visited.insert(knots[9]);
        }
    }
    visited.len()
}

pub fn main() {
    let data = std::fs::read_to_string("data/2022/day9").unwrap();
    let motions = parse(&data);
    println!("part1: {}", part1(&motions));
    println!("part2: {}", part2(&motions));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        let motions = parse(data);
        assert_eq!(13, part1(&motions));
        assert_eq!(1, part2(&motions));
    }

    #[test]
    fn case2() {
        let data = "
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        let motions = parse(data);
        assert_eq!(88, part1(&motions));
        assert_eq!(36, part2(&motions));
    }
}
