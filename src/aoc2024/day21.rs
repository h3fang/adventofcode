use std::collections::VecDeque;

fn parse(input: &str) -> Vec<&[u8]> {
    input.trim().lines().map(|line| line.as_bytes()).collect()
}

const NUMERICAL: &[u8] = b"789456123 0A";
const DIRECTIONAL: &[u8] = b" ^A<v>";

fn get_path(rows: i8, dist: &[i8], start: i8, end: i8) -> Vec<Vec<u8>> {
    let mut result = Vec::with_capacity(9);
    let mut q = Vec::with_capacity(6);
    q.push((end, vec![b'A']));
    while let Some((k, path)) = q.pop() {
        if k == start {
            result.push(path);
            continue;
        }
        let (i, j) = (k / 3, k % 3);
        for (i1, j1, dir) in [
            (i - 1, j, b'v'),
            (i, j - 1, b'>'),
            (i, j + 1, b'<'),
            (i + 1, j, b'^'),
        ] {
            let k1 = i1 * 3 + j1;
            if i1 < 0
                || j1 < 0
                || i1 == rows
                || j1 == 3
                || dist[k1 as usize] != dist[k as usize] - 1
            {
                continue;
            }
            let mut path1 = Vec::with_capacity(path.len() + 1);
            path1.push(dir);
            path1.extend(&path);
            q.push((k1, path1));
        }
    }
    result
}

struct NumericalKeypad {
    paths: Vec<Vec<Vec<u8>>>,
}

impl NumericalKeypad {
    fn new() -> Self {
        let mut paths = Vec::with_capacity(11 * 11);
        for i in 0..11 {
            let key = Self::key_code(i);
            let start = NUMERICAL.iter().position(|&b| b == key).unwrap() as i8;
            let dist = shortest_paths(NUMERICAL, start);
            for j in 0..11 {
                let key = Self::key_code(j);
                let end = NUMERICAL.iter().position(|&b| b == key).unwrap() as i8;
                paths.push(get_path(4, &dist, start, end));
            }
        }
        Self { paths }
    }

    fn key_code(index: usize) -> u8 {
        match index {
            0..=9 => b'0' + index as u8,
            10 => b'A',
            _ => unreachable!(),
        }
    }

    fn key_index(code: u8) -> usize {
        match code {
            b'0'..=b'9' => (code - b'0') as usize,
            b'A' => 10,
            _ => unreachable!(),
        }
    }
}

struct DirectionalKeypad {
    paths: Vec<Vec<Vec<u8>>>,
}

impl DirectionalKeypad {
    fn new() -> Self {
        let mut paths = Vec::with_capacity(5 * 5);
        for i in 0..5 {
            let key = Self::key_code(i);
            let start = DIRECTIONAL.iter().position(|&b| b == key).unwrap() as i8;
            let dist = shortest_paths(DIRECTIONAL, start);
            for j in 0..5 {
                let key = Self::key_code(j);
                let end = DIRECTIONAL.iter().position(|&b| b == key).unwrap() as i8;
                paths.push(get_path(2, &dist, start, end));
            }
        }
        Self { paths }
    }

    fn key_code(index: usize) -> u8 {
        match index {
            0 => b'^',
            1 => b'<',
            2 => b'v',
            3 => b'>',
            4 => b'A',
            _ => unreachable!(),
        }
    }

    fn key_index(code: u8) -> usize {
        match code {
            b'^' => 0,
            b'<' => 1,
            b'v' => 2,
            b'>' => 3,
            b'A' => 4,
            _ => unreachable!(),
        }
    }
}

fn shortest_paths(keypad: &[u8], start: i8) -> Vec<i8> {
    let m = keypad.len() / 3;
    let mut dist = vec![i8::MAX; keypad.len()];
    let mut q = VecDeque::with_capacity(keypad.len());
    q.push_back((0, start));
    dist[start as usize] = 0;
    while let Some((d, k)) = q.pop_front() {
        if d > dist[k as usize] {
            continue;
        }
        let (i, j) = (k / 3, k % 3);
        for (i1, j1) in [(i - 1, j), (i, j - 1), (i, j + 1), (i + 1, j)] {
            let k1 = i1 * 3 + j1;
            if i1 < 0 || j1 < 0 || i1 == m as i8 || j1 == 3 || keypad[k1 as usize] == b' ' {
                continue;
            }
            if d + 1 < dist[k1 as usize] {
                dist[k1 as usize] = d + 1;
                q.push_back((d + 1, k1));
            }
        }
    }
    dist
}

fn solve(codes: &[&[u8]], directional_robots: u8) -> usize {
    let dir_keypad = DirectionalKeypad::new();
    let mut f_d = [[0; 5]; 5];
    for d in 0..directional_robots {
        let mut next = [[0; 5]; 5];
        for (i, r) in next.iter_mut().enumerate() {
            for (j, f) in r.iter_mut().enumerate() {
                let paths = &dir_keypad.paths[i * 5 + j];
                if d == 0 {
                    *f = paths.iter().map(|p| p.len()).min().unwrap();
                } else {
                    let mut min = usize::MAX;
                    for p in paths {
                        let len = p
                            .iter()
                            .fold((0, DirectionalKeypad::key_index(b'A')), |(len, i), &b| {
                                let j = DirectionalKeypad::key_index(b);
                                (len + f_d[i][j], j)
                            })
                            .0;
                        min = min.min(len);
                    }
                    *f = min;
                }
            }
        }
        f_d = next;
    }

    let num_keypad = NumericalKeypad::new();
    let mut f_n = [[0; 11]; 11];
    for (i, r) in f_n.iter_mut().enumerate() {
        for (j, f) in r.iter_mut().enumerate() {
            let paths = &num_keypad.paths[i * 11 + j];
            let mut min = usize::MAX;
            for p in paths {
                let len = p
                    .iter()
                    .fold((0, DirectionalKeypad::key_index(b'A')), |(len, i), &b| {
                        let j = DirectionalKeypad::key_index(b);
                        (len + f_d[i][j], j)
                    })
                    .0;
                min = min.min(len);
            }
            *f = min;
        }
    }

    codes
        .iter()
        .map(|code| {
            let len = code
                .iter()
                .fold((0, NumericalKeypad::key_index(b'A')), |(len, i), &b| {
                    let j = NumericalKeypad::key_index(b);
                    (len + f_n[i][j], j)
                })
                .0;
            let num: usize = std::str::from_utf8(&code[..code.len() - 1])
                .unwrap()
                .parse()
                .unwrap();
            num * len
        })
        .sum()
}

pub fn main() {
    let input: String = std::fs::read_to_string("data/2024/day21").unwrap();
    let codes = parse(&input);
    println!("part1: {}", solve(&codes, 2));
    println!("part2: {}", solve(&codes, 25));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = "
029A
980A
179A
456A
379A";
        let codes = parse(input);
        assert_eq!(126384, solve(&codes, 2));
    }
}
