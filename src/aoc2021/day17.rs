fn parse(data: &str) -> Vec<i64> {
    let (_, coords) = data.trim().split_once(": ").unwrap();
    let (x, y) = coords.split_once(", ").unwrap();
    [x, y]
        .iter()
        .flat_map(|s| {
            let (_, range) = s.split_once('=').unwrap();
            let (low, high) = range.split_once("..").unwrap();
            [low.parse().unwrap(), high.parse().unwrap()]
        })
        .collect()
}

fn shoot(mut vx: i64, mut vy: i64, limits: &[i64]) -> (bool, i64) {
    let mut x = 0i64;
    let mut y = 0;

    let is_within =
        |x: i64, y: i64| x >= limits[0] && x <= limits[1] && y >= limits[2] && y <= limits[3];

    let mut highest = y;
    while y >= limits[2] {
        x += vx;
        y += vy;
        vx -= vx.signum();
        vy -= 1;
        highest = highest.max(y);
        if is_within(x, y) {
            return (true, highest);
        }
    }
    (false, 0)
}

fn solve(limits: &[i64]) -> (i64, usize) {
    let mut p1 = 0;
    let mut p2 = 0;
    let y_max = limits[2].abs().max(limits[3].abs());
    for vx in 1..=limits[1] {
        for vy in -y_max..=y_max {
            let r = shoot(vx, vy, limits);
            if r.0 {
                p1 = p1.max(r.1);
                p2 += 1;
            }
        }
    }
    (p1, p2)
}

pub fn main() {
    let data = std::fs::read_to_string("data/2021/day17").unwrap();
    let limits = parse(&data);
    let (p1, p2) = solve(&limits);
    println!("day17 part1: {}", p1);
    println!("day17 part2: {}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "target area: x=20..30, y=-10..-5";
        let limits = parse(&data);
        let (p1, p2) = solve(&limits);
        assert_eq!(45, p1);
        assert_eq!(112, p2);
    }
}
