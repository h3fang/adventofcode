use ahash::HashMap;
use rayon::prelude::*;

type Int = u16;

struct Blueprint {
    resources: [[Int; 3]; 4],
    max: [Int; 3],
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Factory {
    resources: [Int; 4],
    robots: [Int; 4],
}

impl Default for Factory {
    fn default() -> Self {
        Self {
            resources: Default::default(),
            robots: [1, 0, 0, 0],
        }
    }
}

impl Factory {
    /// Number of minutes we need to wait to fullfill the resource requirements.
    fn wait_for_resources(&self, requirements: [Int; 3]) -> Option<u8> {
        let mut max = 0;
        for (r, (req, robot)) in self
            .resources
            .into_iter()
            .zip(requirements.into_iter().zip(self.robots))
        {
            if r < req {
                if robot == 0 {
                    return None;
                }
                let t = (req - r).div_ceil(robot);
                max = max.max(t);
            }
        }
        Some(max as u8)
    }

    /// Do not build an intermediate robot if there are enough robot for the
    /// highest demanding recipe in the blueprint (that depends on it's output).
    fn try_build_robot(&self, t: u8, i: usize, bp: &Blueprint) -> Option<(u8, Factory)> {
        if (i == 3 || self.robots[i] < bp.max[i])
            && (0..3).all(|j| self.robots[j] > 0 || self.resources[j] >= bp.resources[i][j])
        {
            let mut f1 = *self;
            let dt = self.wait_for_resources(bp.resources[i])?;
            if dt + 1 >= t {
                return None;
            }
            f1.resources
                .iter_mut()
                .zip(
                    self.robots
                        .into_iter()
                        .zip(bp.resources[i].into_iter().chain([0])),
                )
                .for_each(|(e, (r, req))| *e = *e + r * (dt + 1) as u16 - req);
            f1.robots[i] += 1;
            Some((t - dt - 1, f1))
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

fn dfs(
    t: u8,
    f: Factory,
    bp: &Blueprint,
    best: &mut Int,
    cache: &mut HashMap<(u8, Factory), Int>,
) -> Int {
    if t <= 1 {
        let geodes = if t == 1 {
            f.resources[3] + f.robots[3]
        } else {
            f.resources[3]
        };
        *best = (*best).max(geodes);
        return geodes;
    }

    let key = (t, f);
    if let Some(&r) = cache.get(&key) {
        return r;
    }

    let mut max = f.resources[3] + f.robots[3] * t as Int;

    // maximum possible geodes we can get assuming unlimited intermediate resources
    let max_possible = max + t as Int * (t as Int - 1) / 2;
    if max_possible <= *best {
        return 0;
    }

    // The assumption "If we can build a geode robot, choose to build it rather
    // than other choices." is wrong. (There are couter examples)

    // Fast forward to the minute we can build a type of robot.
    // This cut the branches significantly!!!
    for i in 0..4 {
        if let Some((t1, f1)) = f.try_build_robot(t, i, bp) {
            max = max.max(dfs(t1, f1, bp, best, cache));
        }
    }

    cache.insert(key, max);
    max
}

fn solve(blueprints: &[Blueprint]) -> (usize, usize) {
    blueprints
        .par_iter()
        .enumerate()
        .map(|(i, bp)| {
            let mut cache = HashMap::default();
            let f = Factory::default();
            let p1 = (i + 1) * dfs(24, f, bp, &mut 0, &mut cache) as usize;
            let p2 = if i < 3 {
                dfs(32, f, bp, &mut 0, &mut cache) as usize
            } else {
                1
            };
            (p1, p2)
        })
        .reduce(|| (0, 1), |r, e| (r.0 + e.0, r.1 * e.1))
}

pub fn main() {
    let data = std::fs::read_to_string("data/2022/day19").unwrap();
    let blueprints = parse(&data);
    let (p1, p2) = solve(&blueprints);
    println!("part1: {}", p1);
    println!("part2: {}", p2);
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
        let (p1, p2) = solve(&blueprints);
        assert_eq!(33, p1);
        assert_eq!(56 * 62, p2);
    }
}
