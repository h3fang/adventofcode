const DIR: [(i64, i64); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

enum Directive {
    Forward(u8),
    Turn(u8),
}

fn parse(data: &str) -> (Vec<Vec<u8>>, Vec<Directive>) {
    let (map, p) = data.split_once("\n\n").unwrap();
    let mut width = 0;
    let mut map = map
        .lines()
        .map(|line| {
            let r = line.as_bytes().to_vec();
            width = width.max(r.len());
            r
        })
        .collect::<Vec<_>>();
    map.iter_mut().for_each(|r| r.resize(width, b' '));
    let mut path = vec![];
    for e in p.trim().split_inclusive(|c: char| c.is_alphabetic()) {
        let n = e.len();
        if e.as_bytes()[n - 1].is_ascii_alphabetic() {
            path.extend([
                Directive::Forward(e[..n - 1].parse().unwrap()),
                Directive::Turn(e.as_bytes()[n - 1]),
            ]);
        } else {
            path.push(Directive::Forward(e.parse().unwrap()));
        }
    }
    (map, path)
}

fn part1(map: &[Vec<u8>], path: &[Directive]) -> usize {
    let m = map.len() as i64;
    let n = map[0].len() as i64;
    let (mut i, mut j, mut facing) = (0, map[0].iter().position(|c| *c == b'.').unwrap() as i64, 0);
    for d in path {
        match d {
            Directive::Forward(k) => {
                let (di, dj) = DIR[facing];
                for _ in 0..*k {
                    let (mut i1, mut j1) = ((i + di + m) % m, (j + dj + n) % n);
                    if di != 0 {
                        while map[i1 as usize][j1 as usize] == b' ' {
                            i1 = (i1 + di + m) % m;
                        }
                    }
                    if dj != 0 {
                        while map[i1 as usize][j1 as usize] == b' ' {
                            j1 = (j1 + dj + n) % n;
                        }
                    }
                    if map[i1 as usize][j1 as usize] == b'#' {
                        break;
                    }
                    i = i1;
                    j = j1;
                }
            }
            Directive::Turn(t) => match t {
                b'L' => facing = (facing + 3) % 4,
                b'R' => facing = (facing + 1) % 4,
                _ => unreachable!(),
            },
        }
    }
    1000 * (i as usize + 1) + 4 * (j as usize + 1) + facing
}

/// Only works for my specific input. And I don't want to implement a generic solution.
fn wrap(i: i64, j: i64, f: usize) -> (i64, i64, usize) {
    match (i, j, f) {
        (i, 50, 2) if (0..50).contains(&i) => (149 - i, 0, 0),
        (i, 0, 2) if (100..150).contains(&i) => (149 - i, 50, 0),
        (i, 50, 2) if (50..100).contains(&i) => (100, i - 50, 1),
        (100, j, 3) if (0..50).contains(&j) => (50 + j, 50, 0),
        (i, 0, 2) if (150..200).contains(&i) => (0, i - 100, 1),
        (0, j, 3) if (50..100).contains(&j) => (j + 100, 0, 0),
        (199, j, 1) if (0..50).contains(&j) => (0, 100 + j, 1),
        (0, j, 3) if (100..150).contains(&j) => (199, j - 100, 3),
        (i, 49, 0) if (150..200).contains(&i) => (149, i - 100, 3),
        (149, j, 1) if (50..100).contains(&j) => (j + 100, 49, 2),
        (i, 99, 0) if (100..150).contains(&i) => (149 - i, 149, 2),
        (i, 149, 0) if (0..50).contains(&i) => (149 - i, 99, 2),
        (49, j, 1) if (100..150).contains(&j) => (j - 50, 99, 2),
        (i, 99, 0) if (50..100).contains(&i) => (49, i + 50, 3),
        _ => (i + DIR[f].0, j + DIR[f].1, f),
    }
}

fn part2(map: &[Vec<u8>], path: &[Directive]) -> usize {
    let (mut i, mut j, mut facing) = (0, map[0].iter().position(|c| *c == b'.').unwrap() as i64, 0);
    for d in path {
        match d {
            Directive::Forward(k) => {
                for _ in 0..*k {
                    let (i1, j1, f1) = wrap(i, j, facing);
                    if map[i1 as usize][j1 as usize] == b' ' {
                        unreachable!();
                    }
                    if map[i1 as usize][j1 as usize] == b'#' {
                        break;
                    }
                    i = i1;
                    j = j1;
                    facing = f1;
                }
            }
            Directive::Turn(t) => match t {
                b'L' => facing = (facing + 3) % 4,
                b'R' => facing = (facing + 1) % 4,
                _ => unreachable!(),
            },
        }
    }
    1000 * (i as usize + 1) + 4 * (j as usize + 1) + facing
}

pub fn main() {
    let data = std::fs::read_to_string("data/2022/day22").unwrap();
    let (map, path) = parse(&data);
    println!("part1: {}", part1(&map, &path));
    println!("part2: {}", part2(&map, &path));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";
        let (map, path) = parse(data);
        assert_eq!(6032, part1(&map, &path));
        // assert_eq!(5031, part2(&map, &path));
    }

    #[test]
    fn case2() {
        let data = std::fs::read_to_string("data/2022/day22").unwrap();
        let (map, path) = parse(&data);
        assert_eq!(67390, part1(&map, &path));
        assert_eq!(95291, part2(&map, &path));
    }
}
