use std::collections::BTreeMap;

use ahash::HashSet;
use rayon::prelude::*;

fn parse(data: &str) -> Vec<Vec<i32>> {
    data.trim()
        .lines()
        .map(|line| {
            let mut p = line
                .split_ascii_whitespace()
                .filter(|p| p.contains('='))
                .map(|p| {
                    let p = p.trim_end_matches(|c| c == ',' || c == ':');
                    p[2..].parse().unwrap()
                })
                .collect::<Vec<i32>>();
            p.push((p[0] - p[2]).abs() + (p[1] - p[3]).abs());
            p
        })
        .collect()
}

fn part1(sensors: &[Vec<i32>], y0: i32) -> usize {
    let mut beacons = HashSet::default();
    for s in sensors {
        if s[3] == y0 {
            beacons.insert((s[2], s[3]));
        }
    }
    let mut m = BTreeMap::default();
    for s in sensors {
        let d = s[4];
        if (s[1] - d..=s[1] + d).contains(&y0) {
            let dx = d - (s[1] - y0).abs();
            let mut x1 = s[0] - dx;
            let mut x2 = s[0] + dx;
            let mut merged = vec![];
            for (&a, &b) in m.range(..=x2) {
                if b < x1 {
                    continue;
                }
                x1 = x1.min(a);
                x2 = x2.max(b);
                merged.push(a);
            }
            for x in merged {
                m.remove(&x);
            }
            if let Some(&x3) = m.get(&(x2 + 1)) {
                m.remove(&(x2 + 1));
                x2 = x3;
            }
            m.insert(x1, x2);
        }
    }
    m.iter().map(|(a, b)| b - a + 1).sum::<i32>() as usize - beacons.len()
}

fn find_empty_pos(sensors: &[Vec<i32>], max: i32, y0: i32) -> Option<usize> {
    let mut m = BTreeMap::new();
    for s in sensors {
        let d = s[4];
        if (s[1] - d..=s[1] + d).contains(&y0) {
            let dx = d - (s[1] - y0).abs();
            let mut x1 = (s[0] - dx).max(0);
            let mut x2 = (s[0] + dx).min(max);
            let mut merged = vec![];
            for (&a, &b) in m.range(..=x2) {
                if b < x1 {
                    continue;
                }
                x1 = x1.min(a);
                x2 = x2.max(b);
                merged.push(a);
            }
            for x in merged {
                m.remove(&x);
            }
            if let Some(&x3) = m.get(&(x2 + 1)) {
                m.remove(&(x2 + 1));
                x2 = x3;
            }
            m.insert(x1, x2);
        }
    }
    let mut prev = 0;
    for (a, b) in m {
        if a > prev {
            break;
        }
        prev = b + 1;
    }
    if prev <= max {
        Some(prev as usize)
    } else {
        None
    }
}

fn part2(sensors: &[Vec<i32>], max: i32) -> usize {
    (0..=max)
        .into_par_iter()
        .find_map_first(|y0| find_empty_pos(sensors, max, y0).map(|x| x * 400_0000 + y0 as usize))
        .unwrap() as usize
}

pub fn main() {
    let data = std::fs::read_to_string("data/2022/day15").unwrap();
    let sensors = parse(&data);
    println!("part1: {}", part1(&sensors, 200_0000));
    println!("part2: {}", part2(&sensors, 400_0000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";
        let sensors = parse(&data);
        assert_eq!(26, part1(&sensors, 10));
        assert_eq!(56000011, part2(&sensors, 20));
    }
}
