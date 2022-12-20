use ahash::HashMap;
use rayon::prelude::*;

type Int = u8;

#[derive(Debug)]
struct Blueprint {
    resources: [[Int; 3]; 4],
    max: [Int; 3],
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Intermediate {
    resources: [Int; 3],
    robots: [Int; 3],
}

impl Default for Intermediate {
    fn default() -> Self {
        Self {
            resources: Default::default(),
            robots: [1, 0, 0],
        }
    }
}

#[derive(Default, Clone)]
struct Factory {
    im: Intermediate,
    geode_robot: Int,
    geode: Int,
}

impl Factory {
    fn collect_resources(&mut self) {
        self.im
            .resources
            .iter_mut()
            .zip(self.im.robots)
            .for_each(|(e, n)| *e += n);
        self.geode += self.geode_robot;
    }

    fn try_build_robot(&self, i: usize, bp: &Blueprint) -> Option<Factory> {
        // do not build intermediate robot more than the maximum required by the blueprint
        if (i == 3 || (i < 3 && self.im.robots[i] < bp.max[i]))
            && self
                .im
                .resources
                .into_iter()
                .zip(bp.resources[i])
                .all(|(a, b)| a >= b)
        {
            let mut f1 = self.clone();
            f1.im
                .resources
                .iter_mut()
                .zip(bp.resources[i])
                .for_each(|(r, n)| *r -= n);
            f1.collect_resources();
            if i == 3 {
                f1.geode_robot += 1;
            } else {
                f1.im.robots[i] += 1;
            }
            Some(f1)
        } else {
            None
        }
    }
}

fn parse(data: &str) -> Vec<Blueprint> {
    data.trim()
        .lines()
        .map(|line| {
            let mut p = line
                .split_ascii_whitespace()
                .filter_map(|p| p.parse::<Int>().ok());
            let resources = [
                [p.next().unwrap(), 0, 0],
                [p.next().unwrap(), 0, 0],
                [p.next().unwrap(), p.next().unwrap(), 0],
                [p.next().unwrap(), 0, p.next().unwrap()],
            ];
            let mut max = [0; 3];
            max.iter_mut().enumerate().for_each(|(i, e)| {
                *e = resources.iter().map(|r| r[i]).max().unwrap();
            });
            Blueprint { resources, max }
        })
        .collect()
}

fn bt(
    t: u8,
    mut f: Factory,
    bp: &Blueprint,
    best: &mut u16,
    cache: &mut HashMap<(u8, Intermediate), Int>,
) -> Int {
    if t == 0 {
        *best = (*best).max(f.geode as u16);
        return f.geode;
    }
    let key = (t, f.im);
    let geodes = f.geode_robot * t as Int + f.geode;
    if let Some(&r) = cache.get(&key) {
        return r + geodes;
    }

    // maximum possible geodes we can get assume we build a geode robot every day
    let max_possible = geodes as u16 + t as u16 * (t as u16 - 1) / 2;
    if max_possible <= *best {
        return 0;
    }

    // if we can build a geode robot, choose to build it
    let max = if let Some(f1) = f.try_build_robot(3, bp) {
        bt(t - 1, f1, bp, best, cache)
    } else {
        let mut max = 0;
        for i in 0..3 {
            if let Some(f1) = f.try_build_robot(i, bp) {
                max = max.max(bt(t - 1, f1, bp, best, cache));
            }
        }
        f.collect_resources();
        max.max(bt(t - 1, f, bp, best, cache))
    };
    cache.insert(key, max.saturating_sub(geodes));
    max
}

fn part1(blueprints: &[Blueprint]) -> usize {
    blueprints
        .par_iter()
        .enumerate()
        .map(|(i, b)| {
            let mut cache = HashMap::default();
            let geodes = bt(24, Factory::default(), b, &mut 0, &mut cache) as usize;
            (i + 1) * geodes
        })
        .sum()
}

fn part2(blueprints: &[Blueprint]) -> usize {
    blueprints
        .par_iter()
        .take(3)
        .map(|b| {
            let mut cache = HashMap::default();
            bt(32, Factory::default(), b, &mut 0, &mut cache) as usize
        })
        .product()
}

pub fn main() {
    let data = std::fs::read_to_string("data/2022/day19").unwrap();
    let blueprints = parse(&data);
    println!("part1: {}", part1(&blueprints));
    println!("part2: {}", part2(&blueprints));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "
Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";
        let blueprints = parse(&data);
        assert_eq!(33, part1(&blueprints));
        assert_eq!(56 * 62, part2(&blueprints));
    }
}
