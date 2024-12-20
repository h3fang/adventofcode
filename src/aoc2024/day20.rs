use std::{cmp::Reverse, collections::BinaryHeap};

use rayon::prelude::*;

fn parse(input: &str) -> Vec<&[u8]> {
    input.trim().lines().map(|line| line.as_bytes()).collect()
}

fn find_start(map: &[&[u8]]) -> (i32, i32) {
    for (i, r) in map.iter().enumerate() {
        for (j, &c) in r.iter().enumerate() {
            if c == b'S' {
                return (i as i32, j as i32);
            }
        }
    }
    unreachable!()
}

fn dijkstra(map: &[&[u8]], (i0, j0): (i32, i32)) -> Vec<i32> {
    let (m, n) = (map.len(), map[0].len());
    let mut dist = vec![i32::MAX; m * n];
    let mut q = BinaryHeap::new();
    let k0 = i0 * n as i32 + j0;
    dist[k0 as usize] = 0;
    q.push((Reverse(0), k0));
    while let Some((Reverse(d), k)) = q.pop() {
        if d > dist[k as usize] {
            continue;
        }
        let (i, j) = (k / n as i32, k % n as i32);
        for (i1, j1) in [(i - 1, j), (i, j - 1), (i, j + 1), (i + 1, j)] {
            if i1 < 0
                || j1 < 0
                || i1 == m as i32
                || j1 == n as i32
                || map[i1 as usize][j1 as usize] == b'#'
            {
                continue;
            }
            let k1 = i1 * n as i32 + j1;
            if d + 1 < dist[k1 as usize] {
                dist[k1 as usize] = d + 1;
                q.push((Reverse(d + 1), k1));
            }
        }
    }
    dist
}

fn cheat(map: &[&[u8]], dist: &[i32], i: i32, j: i32, save: i32) -> (usize, usize) {
    let (m, n) = (map.len(), map[0].len());
    let (mut p1, mut p2) = (0, 0);
    let d1 = dist[i as usize * n + j as usize];

    for (i1, j1) in [
        (i - 2, j),
        (i - 1, j - 1),
        (i - 1, j + 1),
        (i, j - 2),
        (i, j + 2),
        (i + 1, j - 1),
        (i + 1, j + 1),
        (i + 2, j),
    ] {
        if i1 < 0
            || j1 < 0
            || i1 >= m as i32
            || j1 >= n as i32
            || map[i1 as usize][j1 as usize] == b'#'
        {
            continue;
        }
        let d2 = dist[i1 as usize * n + j1 as usize];
        if d2 - d1 - 2 >= save {
            p1 += 1;
        }
    }

    for i1 in i - 20..=i + 20 {
        let di = (i1 - i).abs();
        for j1 in j - (20 - di)..=j + 20 - di {
            let d = di + (j1 - j).abs();
            if i1 < 0
                || j1 < 0
                || i1 >= m as i32
                || j1 >= n as i32
                || map[i1 as usize][j1 as usize] == b'#'
            {
                continue;
            }
            let d2 = dist[i1 as usize * n + j1 as usize];
            if d2 - d1 - d >= save {
                p2 += 1;
            }
        }
    }

    (p1, p2)
}

fn solve(map: &[&[u8]], save: i32) -> (usize, usize) {
    let start = find_start(map);
    let dist = dijkstra(map, start);
    let (m, n) = (map.len(), map[0].len());

    (0..m * n)
        .into_par_iter()
        .map(|k| {
            let (i, j) = (k / n, k % n);
            if map[i][j] == b'#' {
                (0, 0)
            } else {
                cheat(map, &dist, i as i32, j as i32, save)
            }
        })
        .reduce(|| (0, 0), |(p1, p2), (a, b)| (p1 + a, p2 + b))
}

pub fn main() {
    let input: String = std::fs::read_to_string("data/2024/day20").unwrap();
    let map = parse(&input);
    let (p1, p2) = solve(&map, 100);
    println!("part1: {p1}");
    println!("part2: {p2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = "
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
        let map = parse(input);
        assert_eq!((44, 3081), solve(&map, 2));
        assert_eq!((2, 593), solve(&map, 40));
        assert_eq!((1, 86), solve(&map, 64));
        assert_eq!((0, 29), solve(&map, 72));
        assert_eq!((0, 7), solve(&map, 74));
        assert_eq!((0, 3), solve(&map, 76));
    }
}
