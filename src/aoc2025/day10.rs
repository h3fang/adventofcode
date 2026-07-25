use microlp::{ComparisonOp, OptimizationDirection, Problem};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

type Lights = u16;
type Buttons = Vec<u16>;
type Joltages = Vec<u16>;

fn parse(data: &str) -> Vec<(Lights, Buttons, Joltages)> {
    data.trim()
        .lines()
        .map(|line| {
            let (lights, rem) = line.split_once(']').unwrap();

            let lights = lights.strip_prefix("[").unwrap();
            let lights = lights
                .as_bytes()
                .iter()
                .enumerate()
                .fold(
                    0,
                    |acc, (i, &x)| {
                        if x == b'#' { acc | (1 << i) } else { acc }
                    },
                );

            let (buttons, joltages) = rem.split_once('{').unwrap();

            let buttons = buttons
                .split(' ')
                .filter(|b| !b.is_empty())
                .map(|b| {
                    let toggles = b.strip_prefix('(').unwrap().strip_suffix(')').unwrap();
                    toggles
                        .split(',')
                        .map(|t| t.parse::<u8>().unwrap())
                        .fold(0, |acc, t| acc | (1 << t))
                })
                .collect::<Vec<_>>();

            let joltages = joltages
                .strip_suffix('}')
                .unwrap()
                .split(',')
                .map(|j| j.parse().unwrap())
                .collect();

            (lights, buttons, joltages)
        })
        .collect()
}

fn part1(manuals: &[(Lights, Buttons, Joltages)]) -> u32 {
    let mut ans = 0;
    for (lights, buttons, _) in manuals {
        let n = buttons.len();
        let mut min = n as u32;
        for mut mask in 0u16..(1 << n) {
            let toggles = mask.count_ones();
            if toggles >= min {
                continue;
            }

            let mut current = 0;
            let mut i = 0;
            while mask > 0 {
                if mask & 1 == 1 {
                    current ^= buttons[i];
                }
                mask >>= 1;
                i += 1;
            }

            if current == *lights {
                min = toggles;
            }
        }
        ans += min;
    }
    ans
}

fn solve_ilp(buttons: &[u16], joltages: &[u16]) -> u32 {
    let mut problem = Problem::new(OptimizationDirection::Minimize);
    let num_buttons = buttons.len();
    let mut vars = Vec::with_capacity(num_buttons);
    for _ in 0..num_buttons {
        let v = problem.add_integer_var(1.0, (0, i32::from(u16::MAX)));
        vars.push(v);
    }

    for (i, &joltage) in joltages.iter().enumerate() {
        let mut coef = Vec::with_capacity(num_buttons);

        for (j, &button) in buttons.iter().enumerate() {
            if (button >> i) & 1 == 1 {
                coef.push((vars[j], 1.0));
            }
        }

        problem.add_constraint(&coef, ComparisonOp::Eq, joltage as f64);
    }

    problem
        .solve()
        .ok()
        .map(|s| s.objective().round() as u32)
        .unwrap_or(0)
}

fn part2(manuals: &[(Lights, Buttons, Joltages)]) -> u32 {
    manuals
        .into_par_iter()
        .map(|(_, buttons, joltages)| solve_ilp(buttons, joltages))
        .sum()
}

pub fn main() {
    let data = std::fs::read_to_string("data/2025/day10").unwrap();
    let manuals = parse(&data);
    println!("part1: {}", part1(&manuals));
    println!("part2: {}", part2(&manuals));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        let manuals = parse(data);
        assert_eq!(7, part1(&manuals));
        assert_eq!(33, part2(&manuals));
    }
}
