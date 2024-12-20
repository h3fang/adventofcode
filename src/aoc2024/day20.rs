use std::{cmp::Reverse, collections::BinaryHeap};

fn parse(input: &str) -> Vec<&[u8]> {
    input.trim().lines().map(|line| line.as_bytes()).collect()
}

fn find_start_end(map: &[&[u8]]) -> ((i32, i32), (i32, i32)) {
    let mut start = (-1, -1);
    let mut end = (-1, -1);
    for (i, r) in map.iter().enumerate() {
        for (j, &c) in r.iter().enumerate() {
            if c == b'S' {
                start = (i as i32, j as i32);
            } else if c == b'E' {
                end = (i as i32, j as i32);
            }
            if start != (-1, -1) && end != (-1, -1) {
                return (start, end);
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

fn solve(map: &[&[u8]], save: i32) -> (usize, usize) {
    let (m, n) = (map.len(), map[0].len());

    let (start, end) = find_start_end(map);
    let from_start = dijkstra(map, start);
    let from_end = dijkstra(map, end);
    let fastest = from_start[end.0 as usize * n + end.1 as usize];
    let (mut p1, mut p2) = (0, 0);

    let is_valid = |i: i32, j: i32| i >= 0 && j >= 0 && i < m as i32 && j < n as i32;

    for (i, r) in map.iter().enumerate() {
        for (j, &c) in r.iter().enumerate() {
            if c == b'#' {
                continue;
            }
            let d1 = from_start[i * n + j];
            let (i, j) = (i as i32, j as i32);

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
                if is_valid(i1, j1) && map[i1 as usize][j1 as usize] != b'#' {
                    let d2 = from_end[i1 as usize * n + j1 as usize];
                    if d1 + d2 + 2 + save <= fastest {
                        p1 += 1;
                    }
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
                    let d2 = from_end[i1 as usize * n + j1 as usize];
                    if d1 + d2 + d + save <= fastest {
                        p2 += 1;
                    }
                }
            }
        }
    }

    (p1, p2)
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
