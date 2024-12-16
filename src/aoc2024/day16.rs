use std::{cmp::Reverse, collections::BinaryHeap};

fn parse(input: &str) -> Vec<&[u8]> {
    input.trim().lines().map(|x| x.as_bytes()).collect()
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

fn solve(map: &[&[u8]]) -> (i32, i32) {
    let (m, n) = (map.len(), map[0].len());
    let (i0, j0) = find_start(map);
    let index = |i: i32, j: i32, d: i32| (i * n as i32 + j) * 4 + d;
    let mut scores = vec![i32::MAX; m * n * 4];
    let mut parent = vec![vec![]; m * n * 4];
    let mut paths = Vec::with_capacity(m * n);
    let mut q = BinaryHeap::with_capacity(m * n);
    let mut best = i32::MAX;
    let k0 = index(i0, j0, 1);
    scores[k0 as usize] = 0;
    q.push((Reverse(0), k0));
    while let Some((Reverse(s), k)) = q.pop() {
        let (i, j, d) = ((k / 4) / n as i32, (k / 4) % n as i32, k % 4);
        if map[i as usize][j as usize] == b'E' {
            match s.cmp(&best) {
                std::cmp::Ordering::Less => {
                    best = s;
                    paths.clear();
                    paths.push(index(i, j, d));
                }
                std::cmp::Ordering::Equal => {
                    paths.push(index(i, j, d));
                }
                _ => {}
            }
            continue;
        }
        for (i1, j1, d1) in [(i - 1, j, 0), (i, j - 1, 3), (i, j + 1, 1), (i + 1, j, 2)] {
            if d1 == (d + 2) % 4 || map[i1 as usize][j1 as usize] == b'#' {
                continue;
            }
            let s1 = s + if d1 == d { 1 } else { 1001 };
            let k1 = index(i1, j1, d1) as usize;
            match s1.cmp(&scores[k1]) {
                std::cmp::Ordering::Less => {
                    scores[k1] = s1;
                    parent[k1].clear();
                    parent[k1].push(k);
                    q.push((Reverse(s1), k1 as i32));
                }
                std::cmp::Ordering::Equal => {
                    parent[k1].push(k);
                }
                _ => {}
            }
        }
    }
    let mut best_tiles = vec![false; m * n];
    while let Some(k) = paths.pop() {
        best_tiles[(k / 4) as usize] = true;
        paths.extend(&parent[k as usize]);
    }
    let count = best_tiles.into_iter().filter(|&t| t).count() as i32;
    (best, count)
}

pub fn main() {
    let input: String = std::fs::read_to_string("data/2024/day16").unwrap();
    let map = parse(&input);
    let (p1, p2) = solve(&map);
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
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
        let map = parse(input);
        let (p1, p2) = solve(&map);
        assert_eq!(7036, p1);
        assert_eq!(45, p2);
    }

    #[test]
    fn case2() {
        let input = "
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";
        let map = parse(input);
        let (p1, p2) = solve(&map);
        assert_eq!(11048, p1);
        assert_eq!(64, p2);
    }
}
