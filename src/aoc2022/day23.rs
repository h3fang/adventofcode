use std::collections::hash_map::Entry;

use ahash::{HashMap, HashMapExt, HashSet};

fn parse(data: &str) -> Vec<&[u8]> {
    data.trim().lines().map(|line| line.as_bytes()).collect()
}

fn boundbox(elves: &HashSet<(i16, i16)>) -> ((i16, i16), (i16, i16)) {
    let mut min = (i16::MAX, i16::MAX);
    let mut max = (i16::MIN, i16::MIN);
    for &(i, j) in elves {
        min.0 = min.0.min(i);
        min.1 = min.1.min(j);
        max.0 = max.0.max(i);
        max.1 = max.1.max(j);
    }
    (min, max)
}

fn neighbors(i: i16, j: i16) -> [(i16, i16); 8] {
    [
        (i - 1, j - 1),
        (i - 1, j),
        (i - 1, j + 1),
        (i, j - 1),
        (i, j + 1),
        (i + 1, j - 1),
        (i + 1, j),
        (i + 1, j + 1),
    ]
}

fn direction(i: i16, j: i16, d: u8) -> [(i16, i16); 3] {
    match d {
        0 => [(i - 1, j - 1), (i - 1, j), (i - 1, j + 1)],
        1 => [(i + 1, j - 1), (i + 1, j), (i + 1, j + 1)],
        2 => [(i - 1, j - 1), (i, j - 1), (i + 1, j - 1)],
        3 => [(i - 1, j + 1), (i, j + 1), (i + 1, j + 1)],
        _ => unreachable!(),
    }
}

// fn print_map(elves: &HashSet<(i16, i16)>) {
//     let (min, max) = boundbox(elves);
//     let w = max.1 - min.1 + 1;
//     let h = max.0 - min.0 + 1;
//     let mut map = vec![vec![b'.'; w as usize]; h as usize];
//     for &(i, j) in elves {
//         map[(i - min.0) as usize][(j - min.1) as usize] = b'#';
//     }
//     for r in map {
//         println!("{}", std::str::from_utf8(&r).unwrap());
//     }
// }

fn propose(elves: &HashSet<(i16, i16)>, d: u8, (i, j): (i16, i16)) -> Option<(i16, i16)> {
    if neighbors(i, j).iter().any(|p| elves.contains(p)) {
        for d1 in d..d + 4 {
            let ps = direction(i, j, d1 % 4);
            if ps.iter().all(|p| !elves.contains(p)) {
                return Some(ps[1]);
            }
        }
    }
    None
}

fn spread(elves: &mut HashSet<(i16, i16)>, d: u8) -> bool {
    // only the adjacent elve in the proposed direction can propose the same position
    let mut m: HashMap<_, Option<(i16, i16)>> = HashMap::with_capacity(elves.len());
    for &p in elves.iter() {
        if let Some(p1) = propose(elves, d, p) {
            match m.entry(p1) {
                Entry::Occupied(mut e) => {
                    if e.get().is_some() {
                        e.insert(None);
                    }
                }
                Entry::Vacant(e) => {
                    e.insert(Some(p));
                }
            };
        }
    }
    let mut moved = false;
    for (proposed, original) in m {
        if let Some(p) = original {
            elves.remove(&p);
            elves.insert(proposed);
            moved = true;
        }
    }
    moved
}

fn solve(map: &[&[u8]]) -> (usize, usize) {
    let mut elves = HashSet::default();
    for (i, r) in map.iter().enumerate() {
        for (j, c) in r.iter().enumerate() {
            if *c == b'#' {
                elves.insert((i as i16, j as i16));
            }
        }
    }
    let mut p1 = 0;
    let mut p2 = 0;
    for i in 0.. {
        let moved = spread(&mut elves, (i % 4) as u8);
        if !moved && p2 == 0 {
            p2 = i + 1;
        }
        if i == 9 {
            let (min, max) = boundbox(&elves);
            let area = (max.0 - min.0 + 1) as usize * (max.1 - min.1 + 1) as usize;
            p1 = area - elves.len();
        }
        if p1 > 0 && p2 > 0 {
            break;
        }
    }
    // print_map(&elves);
    (p1, p2)
}

pub fn main() {
    let data = std::fs::read_to_string("data/2022/day23").unwrap();
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
....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";
        let map = parse(&data);
        let (p1, p2) = solve(&map);
        assert_eq!(110, p1);
        assert_eq!(20, p2);
    }

    #[test]
    fn case2() {
        let data = "
.....
..##.
..#..
.....
..##.
.....";
        let map = parse(&data);
        let (p1, p2) = solve(&map);
        assert_eq!(25, p1);
        assert_eq!(4, p2);
    }
}
