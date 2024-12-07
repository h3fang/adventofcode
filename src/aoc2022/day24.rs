use std::{cmp::Reverse, collections::BinaryHeap};

use ahash::HashMap;

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

struct Map {
    width: i16,
    height: i16,
    entry: i16,
    exit: i16,
    blizzards: Vec<Vec<bool>>,
}

fn parse(data: &str) -> Map {
    let g = data
        .trim()
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();
    let height = g.len();
    let width = g[0].len();
    let entry = g[0].iter().position(|c| *c == b'.').unwrap() as i16;
    let exit = g[height - 1].iter().position(|c| *c == b'.').unwrap() as i16;
    let (h, w) = (height - 2, width - 2);
    let period = lcm(h, w);
    let blizzards = (0..period)
        .map(|t| {
            let mut b = vec![false; height * width];
            for (i, row) in g.iter().enumerate().skip(1).take(h) {
                for (j, &c) in row.iter().enumerate().skip(1).take(w) {
                    if c == b'.' {
                        continue;
                    }
                    let k = match c {
                        b'>' => i * width + 1 + ((j - 1 + t) % w),
                        b'<' => i * width + 1 + ((j - 1 + w - t % w) % w),
                        b'v' => (1 + ((i - 1 + t) % h)) * width + j,
                        b'^' => (1 + ((i - 1 + h - t % h) % h)) * width + j,
                        _ => unreachable!(),
                    };
                    b[k] = true;
                }
            }
            b
        })
        .collect();
    Map {
        width: width as i16,
        height: height as i16,
        entry,
        exit,
        blizzards,
    }
}

fn manhattan((i1, j1): (i16, i16), (i2, j2): (i16, i16)) -> i16 {
    (i1 - i2).abs() + (j1 - j2).abs()
}

fn shortest_path(map: &Map, (i0, j0): (i16, i16), end: (i16, i16), t0: usize) -> usize {
    let mut q = BinaryHeap::new();
    q.push((Reverse(0), (i0, j0, t0)));
    let mut dist = HashMap::default();
    dist.insert((i0, j0, t0), 0);
    while let Some((Reverse(_), (i, j, t))) = q.pop() {
        if (i, j) == end {
            return t;
        }
        let k = (t + 1) % map.blizzards.len();
        let blizz = &map.blizzards[k];
        for (i1, j1) in [(i, j), (i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)] {
            if i1 < 0
                || j1 <= 0
                || i1 >= map.height
                || j1 >= map.width - 1
                || (i1 == 0 && j1 != map.entry)
                || (i1 == map.height - 1 && j1 != map.exit)
                || blizz[(map.width as usize) * i1 as usize + j1 as usize]
            {
                continue;
            }
            let d = dist.entry((i1, j1, t + 1)).or_insert(usize::MAX);
            if t + 1 < *d {
                *d = t + 1;
                let h = manhattan((i1, j1), end) as usize;
                q.push((Reverse(t + 1 + h), (i1, j1, t + 1)));
            }
        }
    }
    unreachable!()
}

fn solve(map: &Map) -> (usize, usize) {
    let (start, end) = ((0, map.entry), (map.height - 1, map.exit));
    let t1 = shortest_path(map, start, end, 0);
    let t2 = shortest_path(map, end, start, t1);
    let t3 = shortest_path(map, start, end, t2);
    (t1, t3)
}

pub fn main() {
    let data = std::fs::read_to_string("data/2022/day24").unwrap();
    let map = parse(&data);
    let (p1, p2) = solve(&map);
    println!("part1: {}", p1);
    println!("part2: {}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "
#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";
        let map = parse(data);
        let (p1, p2) = solve(&map);
        assert_eq!(18, p1);
        assert_eq!(54, p2);
    }
}
