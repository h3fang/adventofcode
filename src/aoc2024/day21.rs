use std::collections::VecDeque;

use ahash::{HashMap, HashMapExt};

fn parse(input: &str) -> Vec<&[u8]> {
    input.trim().lines().map(|line| line.as_bytes()).collect()
}

const NUMERICAL: &[u8] = b"789456123 0A";
const DIRECTIONAL: &[u8] = b" ^A<v>";

fn shortest_paths(keypad: &[u8], from: u8, to: u8) -> Vec<Vec<u8>> {
    let m = keypad.len() / 3;
    let start = keypad.iter().position(|&b| b == from).unwrap() as i8;
    let end = keypad.iter().position(|&b| b == to).unwrap() as i8;

    let mut dist = vec![i8::MAX; keypad.len()];
    let mut q = VecDeque::with_capacity(keypad.len());
    q.push_back((0, start));
    dist[start as usize] = 0;
    while let Some((d, k)) = q.pop_front() {
        if keypad[k as usize] == to || d > dist[k as usize] {
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

    let mut result = Vec::with_capacity(2);
    let mut q = Vec::with_capacity(keypad.len());
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
                || i1 == m as i8
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

fn button_presses(
    keypad: &[u8],
    code: Vec<u8>,
    depth: u8,
    curr: &mut [u8],
    cache: &mut HashMap<(Vec<u8>, u8, u8), usize>,
) -> usize {
    let k = (code, depth, curr[depth as usize]);
    if let Some(&r) = cache.get(&k) {
        return r;
    }
    let mut result = 0;
    for &b in &k.0 {
        let paths = shortest_paths(keypad, curr[depth as usize], b);
        result += if depth == 0 {
            paths.iter().map(|p| p.len()).min().unwrap()
        } else {
            paths
                .into_iter()
                .map(|p| button_presses(DIRECTIONAL, p, depth - 1, curr, cache))
                .min()
                .unwrap()
        };
        curr[depth as usize] = b;
    }
    cache.insert(k, result);
    result
}

fn solve(codes: &[&[u8]], depth: u8) -> usize {
    let mut cache = HashMap::with_capacity(2048);
    codes
        .iter()
        .map(|code| {
            let len = button_presses(
                NUMERICAL,
                code.to_vec(),
                depth,
                &mut vec![b'A'; depth as usize + 1],
                &mut cache,
            );
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
    println!("part1: {}", solve(&codes, 25));
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
