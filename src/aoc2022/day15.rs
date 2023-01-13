use std::collections::BTreeMap;

use ahash::HashSet;
use rayon::prelude::*;

fn manhattan_distance(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

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
            p.push(manhattan_distance(p[0], p[1], p[2], p[3]));
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
    let mut m = vec![];
    for s in sensors {
        let d = s[4];
        if (s[1] - d..=s[1] + d).contains(&y0) {
            let dx = d - (s[1] - y0).abs();
            let mut x1 = (s[0] - dx).max(0);
            let mut x2 = (s[0] + dx).min(max);
            let mut inserted = false;
            let mut next = Vec::with_capacity(m.len() + 1);
            for (a, b) in m {
                if b < x1 {
                    next.push((a, b));
                } else if x2 + 1 < a {
                    if !inserted {
                        next.push((x1, x2));
                        inserted = true;
                    }
                    next.push((a, b));
                } else if x2 + 1 == a {
                    x2 = b;
                    next.push((x1, x2));
                } else {
                    x1 = x1.min(a);
                    x2 = x2.max(b);
                }
            }
            if !inserted {
                next.push((x1, x2));
            }
            m = next;
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

#[allow(unused)]
fn part2_scanline(sensors: &[Vec<i32>], max: i32) -> usize {
    (0..=max)
        .into_par_iter()
        .find_map_first(|y0| find_empty_pos(sensors, max, y0).map(|x| x * 400_0000 + y0 as usize))
        .unwrap()
}

/// Let m be the search area side length, n be the number of sensors.
///
/// Previous method has time complexity O(mn^2).
///
/// The distress beacon must be next to sensor's scan area's edges.
/// We can rotate the map and check the intersections of these points
/// if any of them are not in the range of any sensor.
/// This will have time complexity O(n^2).
fn part2(sensors: &[Vec<i32>], max: i32) -> usize {
    let mut us = HashSet::default();
    let mut vs = HashSet::default();
    for s in sensors {
        let d = s[4] + 1;
        let u0 = s[0] - s[1];
        let v0 = s[0] + s[1];
        us.insert(u0 - d);
        us.insert(u0 + d);
        vs.insert(v0 - d);
        vs.insert(v0 + d);
    }

    for u in us {
        for &v in &vs {
            let x = (u + v) / 2;
            let y = (v - u) / 2;
            if x < 0 || y < 0 || x > max || y > max {
                continue;
            }
            if sensors
                .iter()
                .all(|s| manhattan_distance(x, y, s[0], s[1]) > s[4])
            {
                return x as usize * 400_0000 + y as usize;
            }
        }
    }

    unreachable!()
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
        assert_eq!(56000011, part2_scanline(&sensors, 20));
        assert_eq!(56000011, part2(&sensors, 20));
    }
}
