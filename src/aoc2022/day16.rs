use std::{
    cmp::Reverse,
    collections::VecDeque,
    sync::atomic::{AtomicU16, Ordering},
};

use ahash::HashMap;
use rayon::prelude::*;

struct Valve {
    flow_rate: u16,
    connected: Vec<u8>,
}

struct Map {
    start: u8,
    valves: Vec<Valve>,
    paths: Vec<Vec<i8>>,
    m: usize,
    cache: Vec<AtomicU16>,
}

impl Map {
    fn new(start: u8, valves: Vec<Valve>) -> Self {
        let m = valves.iter().filter(|v| v.flow_rate > 0).count();
        let paths = shortest_paths(&valves);
        let cache = (0..30 * (m + 1) * (1 << m))
            .map(|_| AtomicU16::new(u16::MAX))
            .collect();
        Self {
            start,
            valves,
            paths,
            m,
            cache,
        }
    }
}

fn parse(data: &str) -> Map {
    let mut valves = data
        .trim()
        .lines()
        .map(|line| {
            let (p1, p2) = line.split_once("; ").unwrap();
            let (id, flow_rate) = p1.split_once('=').unwrap();
            let id = &id[6..8];
            let flow_rate = flow_rate.parse().unwrap();
            let connected = p2
                .trim_start_matches(|c: char| c.is_ascii_lowercase() || c.is_ascii_whitespace())
                .split(", ")
                .collect::<Vec<_>>();
            (id, flow_rate, connected)
        })
        .collect::<Vec<_>>();
    valves.sort_unstable_by_key(|v| (Reverse(v.1), v.0));

    let ids = valves
        .iter()
        .enumerate()
        .map(|(i, v)| (v.0, i as u8))
        .collect::<HashMap<_, _>>();
    let start = *ids.get("AA").unwrap();

    let valves = valves
        .into_iter()
        .map(|(_id, flow_rate, conn)| {
            let connected = conn.iter().map(|c| *ids.get(c).unwrap()).collect();
            Valve {
                flow_rate,
                connected,
            }
        })
        .collect::<Vec<_>>();
    Map::new(start, valves)
}

fn shortest_paths_from(start: u8, valves: &[Valve]) -> Vec<i8> {
    let mut result = vec![i8::MAX; valves.len()];
    let mut q = VecDeque::new();
    q.push_back((start, 0, 1u64 << start));
    while let Some((i, dist, visited)) = q.pop_front() {
        result[i as usize] = result[i as usize].min(dist);
        for &c in &valves[i as usize].connected {
            if visited & (1 << c) > 0 {
                continue;
            }
            q.push_back((c, dist + 1, visited | (1 << c)));
        }
    }
    result
}

fn shortest_paths(valves: &[Valve]) -> Vec<Vec<i8>> {
    (0..valves.len() as u8)
        .map(|i| shortest_paths_from(i, valves))
        .collect()
}

fn dfs(t: i8, curr: u8, valves_to_open: u16, map: &Map) -> u16 {
    if t <= 1 {
        return 0;
    }
    let idx = (t - 1) as usize * (map.m + 1) * (1 << map.m)
        + curr as usize * (1 << map.m)
        + valves_to_open as usize;
    let p = map.cache[idx].load(Ordering::Relaxed);
    if p < u16::MAX {
        return p;
    }
    let mut result = 0;
    for next in 0..map.m as u8 {
        if valves_to_open & (1 << next) == 0 {
            continue;
        }
        let dist = map.paths[curr as usize][next as usize];
        let t1 = t - (dist + 1);
        if t1 <= 0 {
            continue;
        }
        let p = map.valves[next as usize].flow_rate * t1 as u16;
        result = result.max(p + dfs(t1, next, valves_to_open - (1 << next), map));
    }
    map.cache[idx].store(result, Ordering::Relaxed);
    result
}

fn part1(t: i8, start: u8, valves_to_open: u16, map: &Map) -> u16 {
    let max = dfs(t, start, valves_to_open, map);
    if valves_to_open & (1 << start) > 0 {
        let p = map.valves[start as usize].flow_rate * (t - 1) as u16
            + dfs(t - 1, start, valves_to_open - (1 << start), map);
        max.max(p)
    } else {
        max
    }
}

fn part2(map: &Map) -> u16 {
    let valves_to_open = (1 << map.m) - 1;
    (1..=valves_to_open / 2)
        .into_par_iter()
        .map(|a| {
            let b = (!a) & valves_to_open;
            part1(26, map.start, a, map) + part1(26, map.start, b, map)
        })
        .max()
        .unwrap()
}

pub fn main() {
    let data = std::fs::read_to_string("data/2022/day16").unwrap();
    let map = parse(&data);
    println!("part1: {}", part1(30, map.start, (1 << map.m) - 1, &map));
    println!("part2: {}", part2(&map));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";
        let map = parse(data);
        assert_eq!(1651, part1(30, map.start, (1 << map.m) - 1, &map));
        assert_eq!(1707, part2(&map));
    }
}
