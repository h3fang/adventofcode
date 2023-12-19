fn parse(data: &str) -> Vec<(u8, i64, u32)> {
    data.trim()
        .lines()
        .map(|l| {
            let parts = l.split_ascii_whitespace().collect::<Vec<_>>();
            (
                parts[0].as_bytes()[0],
                parts[1].parse().unwrap(),
                u32::from_str_radix(parts[2].trim_start_matches("(#").trim_end_matches(')'), 16)
                    .unwrap(),
            )
        })
        .collect()
}

fn points(instructions: &[(u8, i64)]) -> usize {
    let (mut x, mut y) = (0, 0);
    let (mut area, mut boundry_points) = (0i64, 0);
    for &(dir, len) in instructions.iter() {
        boundry_points += len as usize;
        let (dx, dy) = [(0, 1), (1, 0), (0, -1), (-1, 0)][dir as usize];
        let (x1, y1) = (x + dx * len, y + dy * len);
        area += (x - x1) * (y + y1);
        (x, y) = (x1, y1);
    }
    let area = (area.abs() / 2) as usize;
    area + 1 + boundry_points / 2
}

fn part1(plan: &[(u8, i64, u32)]) -> usize {
    let instructions = plan
        .iter()
        .map(|&(d, len, _)| {
            let d = match d {
                b'R' => 0,
                b'D' => 1,
                b'L' => 2,
                b'U' => 3,
                _ => unreachable!(),
            };
            (d, len)
        })
        .collect::<Vec<_>>();
    points(&instructions)
}

fn part2(plan: &[(u8, i64, u32)]) -> usize {
    let instructions = plan
        .iter()
        .map(|&(_, _, c)| {
            let d = (c & 0xf) as u8;
            let len = (c >> 4) as i64;
            (d, len)
        })
        .collect::<Vec<_>>();
    points(&instructions)
}

pub fn main() {
    let data = std::fs::read_to_string("data/2023/day18").unwrap();
    let plan = parse(&data);
    println!("part1: {}", part1(&plan));
    println!("part2: {}", part2(&plan));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = r"
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
        let plan = parse(data);
        assert_eq!(62, part1(&plan));
        assert_eq!(952408144115, part2(&plan));
    }
}
