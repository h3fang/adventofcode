struct ClawMachine {
    a: [i64; 2],
    b: [i64; 2],
    prize: [i64; 2],
}

fn parse_button(s: &str) -> [i64; 2] {
    let (x, y) = s.split_once(", ").unwrap();
    let x = x.trim_start_matches("X+").parse().unwrap();
    let y = y.trim_start_matches("Y+").parse().unwrap();
    [x, y]
}

fn parse_prize(s: &str) -> [i64; 2] {
    let (x, y) = s.split_once(", ").unwrap();
    let x = x.trim_start_matches("X=").parse().unwrap();
    let y = y.trim_start_matches("Y=").parse().unwrap();
    [x, y]
}

fn parse(input: &str) -> Vec<ClawMachine> {
    input
        .trim()
        .split("\n\n")
        .map(|part| {
            let mut lines = part.lines();
            let a = parse_button(lines.next().unwrap().trim_start_matches("Button A: "));
            let b = parse_button(lines.next().unwrap().trim_start_matches("Button B: "));
            let prize = parse_prize(lines.next().unwrap().trim_start_matches("Prize: "));
            ClawMachine { a, b, prize }
        })
        .collect()
}

const OFFSET: i64 = 10000000000000;

fn solve(machines: &[ClawMachine], offset: i64) -> i64 {
    machines
        .iter()
        .map(|m| {
            let p = [m.prize[0] + offset, m.prize[1] + offset];
            let det = m.a[0] * m.b[1] - m.a[1] * m.b[0];
            if det == 0 {
                let x = if m.a[0] > 3 * m.b[0] { m.a } else { m.b };
                let (a, b) = (p[0] / x[0], p[1] / x[1]);
                if a == b && a * x[0] == p[0] && b * x[1] == p[1] {
                    return if x == m.a { 3 * a } else { a };
                } else {
                    return 0;
                }
            }
            let a = (m.b[1] * p[0] - m.b[0] * p[1]) / det;
            let b = (-m.a[1] * p[0] + m.a[0] * p[1]) / det;
            if a * m.a[0] + b * m.b[0] == p[0] && a * m.a[1] + b * m.b[1] == p[1] {
                a * 3 + b
            } else {
                0
            }
        })
        .sum()
}

pub fn main() {
    let input: String = std::fs::read_to_string("data/2024/day13").unwrap();
    let machines = parse(&input);
    println!("part1: {}", solve(&machines, 0));
    println!("part2: {}", solve(&machines, OFFSET));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = "
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        let machines = parse(input);
        assert_eq!(480, solve(&machines, 0));
        assert_eq!(875318608908, solve(&machines, OFFSET));
    }
}
