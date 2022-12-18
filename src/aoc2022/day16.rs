use std::collections::VecDeque;

use ahash::HashMap;
use rayon::prelude::*;

struct Valve {
    flow_rate: u64,
    connected: Vec<u8>,
}

fn parse(data: &str) -> (u8, Vec<Valve>, u64) {
    let mut ids = HashMap::default();
    data.trim().lines().for_each(|line| {
        let id = [line.as_bytes()[6], line.as_bytes()[7]];
        let i = ids.len() as u8;
        ids.insert(id, i);
    });
    let start = *ids.get(&[b'A', b'A']).unwrap();
    let mut valves = Vec::with_capacity(ids.len());
    data.trim().lines().for_each(|line| {
        let (p1, p2) = line.split_once("; ").unwrap();
        let (_id, flow_rate) = p1.split_once('=').unwrap();
        let flow_rate = flow_rate.parse().unwrap();
        let connected = p2
            .trim_start_matches(|c: char| c.is_ascii_lowercase() || c.is_ascii_whitespace())
            .split(", ")
            .map(|v| *ids.get(&[v.as_bytes()[0], v.as_bytes()[1]]).unwrap())
            .collect();
        valves.push(Valve {
            flow_rate,
            connected,
        });
    });
    let valves_to_open =
        valves.iter().enumerate().fold(
            0u64,
            |acc, (i, v)| {
                if v.flow_rate > 0 {
                    acc | (1 << i)
                } else {
                    acc
                }
            },
        );
    (start, valves, valves_to_open)
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

fn dfs(
    t: i8,
    curr: u8,
    opened: u64,
    valves: &[Valve],
    paths: &[Vec<i8>],
    valves_to_open: u64,
    cache: &mut HashMap<(i8, u8, u64), u64>,
) -> u64 {
    if opened == valves_to_open || t <= 1 {
        return 0;
    }
    if let Some(&p) = cache.get(&(t, curr, opened)) {
        return p;
    }
    let mut result = 0;
    for next in 0..valves.len() as u8 {
        if valves_to_open & (1 << next) == 0 || opened & (1 << next) > 0 {
            continue;
        }
        let dist = paths[curr as usize][next as usize];
        let t1 = t - (dist + 1);
        if t1 <= 0 {
            continue;
        }
        let p = valves[next as usize].flow_rate * t1 as u64;
        result = result.max(
            p + dfs(
                t1,
                next,
                opened | (1 << next),
                valves,
                paths,
                valves_to_open,
                cache,
            ),
        );
    }
    cache.insert((t, curr, opened), result);
    result
}

fn part1(start: u8, valves: &[Valve], paths: &[Vec<i8>], valves_to_open: u64, t_max: i8) -> u64 {
    let mut cache = HashMap::default();
    let max = dfs(t_max, start, 0, valves, paths, valves_to_open, &mut cache);
    if valves_to_open & (1 << start) > 0 {
        let p = valves[start as usize].flow_rate * (t_max - 1) as u64
            + dfs(
                t_max - 1,
                start,
                1 << start,
                valves,
                paths,
                valves_to_open,
                &mut cache,
            );
        max.max(p)
    } else {
        max
    }
}

fn combinations(valves_to_open: u64, k: u32, result: &mut Vec<u64>) {
    fn dfs(valves_to_open: u64, k: u32, i: u32, curr: u64, result: &mut Vec<u64>) {
        if curr.count_ones() == k {
            result.push(curr);
        } else if i >= u64::BITS - valves_to_open.leading_zeros() {
            return;
        } else if valves_to_open & (1 << i) == 0 {
            dfs(valves_to_open, k, i + 1, curr, result);
        } else {
            dfs(valves_to_open, k, i + 1, curr, result);
            dfs(valves_to_open, k, i + 1, curr | (1 << i), result);
        }
    }
    dfs(valves_to_open, k, 0, 0, result);
}

fn part2(start: u8, valves: &[Valve], paths: &[Vec<i8>], valves_to_open: u64) -> u64 {
    let mut tasks = Vec::with_capacity(1 << valves_to_open.count_ones());
    for k in 1..=valves_to_open.count_ones() / 2 {
        combinations(valves_to_open, k, &mut tasks);
    }
    tasks
        .into_par_iter()
        .map(|a| {
            let b = (!a) & valves_to_open;
            let mut cache = HashMap::default();
            let a = dfs(26, start, 0, valves, paths, a, &mut cache);
            cache.clear();
            a + dfs(26, start, 0, valves, paths, b, &mut cache)
        })
        .max()
        .unwrap()
}

pub fn main() {
    let data = std::fs::read_to_string("data/2022/day16").unwrap();
    let (start, valves, valves_to_open) = parse(&data);
    let paths = shortest_paths(&valves);
    println!(
        "part1: {}",
        part1(start, &valves, &paths, valves_to_open, 30)
    );
    println!("part2: {}", part2(start, &valves, &paths, valves_to_open));
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
        let (start, valves, valves_to_open) = parse(&data);
        let paths = shortest_paths(&valves);
        assert_eq!(1651, part1(start, &valves, &paths, valves_to_open, 30));
        assert_eq!(1707, part2(start, &valves, &paths, valves_to_open));
    }
}
