use crate::day5::Intcode;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Movement {
    Right,
    Left,
    Forward,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Facing {
    North,
    South,
    East,
    West,
}

impl Facing {
    fn delta(&self) -> Position {
        match self {
            Facing::North => Position(0, -1),
            Facing::South => Position(0, 1),
            Facing::East => Position(1, 0),
            Facing::West => Position(-1, 0),
        }
    }

    fn turn(&self, m: Movement) -> Self {
        match m {
            Movement::Right => match self {
                Facing::North => Facing::East,
                Facing::South => Facing::West,
                Facing::East => Facing::South,
                Facing::West => Facing::North,
            },
            Movement::Left => match self {
                Facing::North => Facing::West,
                Facing::South => Facing::East,
                Facing::East => Facing::North,
                Facing::West => Facing::South,
            },
            Movement::Forward => *self,
        }
    }
}

impl From<u8> for Facing {
    fn from(s: u8) -> Self {
        match s {
            b'^' => Facing::North,
            b'v' => Facing::South,
            b'<' => Facing::West,
            b'>' => Facing::East,
            x => panic!("invalid facing symbol: {x}"),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
struct Position(i64, i64);

impl std::ops::Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

fn get_image(p: &mut Intcode) -> String {
    let mut img = String::new();
    loop {
        p.run();
        if p.is_halted() {
            break;
        }
        img.push(p.outputs.pop_front().unwrap() as u8 as char);
    }
    img
}

fn part1(codes: &[i64]) -> (usize, Vec<Vec<u8>>) {
    let mut p = Intcode::new(codes);
    let img = get_image(&mut p);
    // println!("{}", img);
    let img = img
        .lines()
        .filter(|row| !row.is_empty())
        .map(|row| row.as_bytes().to_vec())
        .collect::<Vec<_>>();
    let height = img.len();
    let width = img[0].len();
    let mut result = 0;
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            if img[y][x] != b'.'
                && img[y - 1][x] != b'.'
                && img[y + 1][x] != b'.'
                && img[y][x - 1] != b'.'
                && img[y][x + 1] != b'.'
            {
                result += x * y;
            }
        }
    }
    (result, img)
}

fn find_robot(img: &[Vec<u8>]) -> (Position, Facing) {
    let mut pos = Position::default();
    let mut facing = Facing::North;
    for (y, row) in img.iter().enumerate() {
        for (x, &pixel) in row.iter().enumerate() {
            if pixel != b'.' && pixel != b'#' {
                pos = Position(x as i64, y as i64);
                facing = pixel.into();
                break;
            }
        }
    }
    (pos, facing)
}

fn get_path(img: &[Vec<u8>]) -> Vec<Movement> {
    let (mut pos, mut facing) = find_robot(img);
    let mut result = vec![];
    let mut prev = pos;
    let height = img.len() as i64;
    let width = img[0].len() as i64;
    let is_valid = |p: Position, prev| {
        p != prev
            && p.0 >= 0
            && p.0 < width
            && p.1 >= 0
            && p.1 < height
            && img[p.1 as usize][p.0 as usize] == b'#'
    };
    loop {
        let forward = pos + facing.delta();
        if is_valid(forward, prev) {
            prev = pos;
            pos = forward;
            result.push(Movement::Forward);
        } else {
            let mut done = true;
            for m in [Movement::Left, Movement::Right] {
                let f = facing.turn(m);
                let p = pos + f.delta();
                if is_valid(p, prev) {
                    facing = f;
                    result.push(m);
                    done = false;
                    break;
                }
            }
            if done {
                break;
            }
        }
    }
    result
}

fn compress(seq: &[Movement]) -> String {
    let mut n = 0;
    let mut result = vec![];
    for &m in seq {
        if m == Movement::Forward {
            n += 1;
        } else {
            if n > 0 {
                result.push(n.to_string());
                n = 0;
            }
            if m == Movement::Left {
                result.push("L".to_string());
            } else if m == Movement::Right {
                result.push("R".to_string());
            }
        }
    }
    if n > 0 {
        result.push(n.to_string());
    }
    result.join(",")
}

fn find_complete_cover(path: &[Movement]) -> (String, String, String, String) {
    fn bt(
        path: &[Movement],
        fns: &mut Vec<usize>,
        patterns: &mut Vec<Vec<Movement>>,
        curr: usize,
    ) -> bool {
        if curr == path.len() {
            return fns.len() <= 10;
        }
        // try with existing patterns
        for i in 0..patterns.len() {
            if path[curr..].starts_with(&patterns[i]) {
                fns.push(i);
                if bt(path, fns, patterns, curr + patterns[i].len()) {
                    return true;
                }
                fns.pop();
            }
        }
        // try with new pattern
        if patterns.len() < 3 {
            let mut p = path[curr..].to_vec();

            while !p.is_empty() {
                while compress(&p).len() > 20 {
                    p.pop();
                }
                patterns.push(p.clone());
                fns.push(patterns.len() - 1);
                if bt(path, fns, patterns, curr + p.len()) {
                    return true;
                } else {
                    patterns.pop();
                    p.pop();
                    fns.pop();
                }
            }
        }
        false
    }
    let mut fns = vec![];
    let mut patterns = vec![];
    if !bt(path, &mut fns, &mut patterns, 0) {
        (
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
        )
    } else {
        (
            fns.iter()
                .map(|f| match f {
                    0 => "A",
                    1 => "B",
                    2 => "C",
                    _ => panic!("impossible"),
                })
                .collect::<Vec<_>>()
                .join(","),
            compress(&patterns[0]),
            compress(&patterns[1]),
            compress(&patterns[2]),
        )
    }
}

fn part2(codes: &[i64], img: Vec<Vec<u8>>) -> i64 {
    let path = get_path(&img);
    // println!("{}", compress(&path));

    let (fns, p1, p2, p3) = find_complete_cover(&path);
    // println!("{} {} {} {}", fns, p1, p2, p3);

    let mut p = Intcode::new(codes);

    for s in [fns, p1, p2, p3, "n".into()] {
        for &b in s.as_bytes() {
            p.inputs.push_back(b as i64);
        }
        p.inputs.push_back(10);
    }
    // println!("{:?}", p.inputs);
    // let mut row = String::new();
    while !p.is_halted() {
        // let c = p.output as u8 as char;
        // if c == '\n' {
        //     println!("{}", row);
        //     row.clear();
        // } else {
        //     row.push(c);
        // }
        p.run();
    }

    p.outputs.pop_back().unwrap()
}

pub fn main() {
    let mut codes = std::fs::read_to_string("data/2019/day17")
        .unwrap()
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|t| t.trim().parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let (alignment, img) = part1(&codes);
    println!("day17 part1: {alignment}");
    codes[0] = 2;
    println!("day17 part2: {}", part2(&codes, img));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn decompress(pattern: &str) -> Vec<Movement> {
        let mut result = vec![];
        for p in pattern.split(',') {
            match p {
                "L" => result.push(Movement::Left),
                "R" => result.push(Movement::Right),
                x => {
                    for _ in 0..x.parse::<usize>().unwrap() {
                        result.push(Movement::Forward);
                    }
                }
            }
        }
        result
    }

    fn complete_cover(path: &[Movement], patterns: &[Vec<Movement>]) -> String {
        fn bt(path: &[Movement], patterns: &[Vec<Movement>], result: &mut Vec<usize>) -> bool {
            if path.is_empty() {
                result.len() <= 10
            } else {
                for (j, p) in patterns.iter().enumerate() {
                    if path.starts_with(p) {
                        result.push(j);
                        if bt(&path[p.len()..], patterns, result) {
                            return true;
                        }
                        result.pop();
                    }
                }
                false
            }
        }
        let mut result = vec![];
        bt(path, patterns, &mut result);
        result
            .iter()
            .map(|f| match f {
                0 => "A",
                1 => "B",
                2 => "C",
                _ => panic!("impossible"),
            })
            .collect::<Vec<_>>()
            .join(",")
    }

    #[test]
    fn case1() {
        let img = "#######...#####
        #.....#...#...#
        #.....#...#...#
        ......#...#...#
        ......#...###.#
        ......#.....#.#
        ^########...#.#
        ......#.#...#.#
        ......#########
        ........#...#..
        ....#########..
        ....#...#......
        ....#...#......
        ....#...#......
        ....#####......"
            .lines()
            .map(|row| row.trim().as_bytes().to_vec())
            .collect::<Vec<_>>();

        let path = get_path(&img);
        let expected_path = "R,8,R,8,R,4,R,4,R,8,L,6,L,2,R,4,R,4,R,8,R,8,R,8,L,6,L,2";

        assert_eq!(expected_path, compress(&path));
        assert_eq!(path, decompress(expected_path));

        let fns = "A,B,C,B,A,C";
        let p1 = "R,8,R,8";
        let p2 = "R,4,R,4,R,8";
        let p3 = "L,6,L,2";

        assert_eq!(
            fns,
            complete_cover(&path, &[decompress(p1), decompress(p2), decompress(p3)])
        );
    }
}
