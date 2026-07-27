use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

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
                .collect();

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

fn gaussian_elimilation(a: &mut [Vec<f64>]) -> Vec<usize> {
    const PIVOT_EPS: f64 = 1e-12;

    let (m, n) = (a.len(), a[0].len() - 1);

    let mut pivots: Vec<usize> = Vec::new();
    let mut row = 0;
    for col in 0..n {
        if row >= m {
            break;
        }
        let piv = (row..m)
            .max_by(|&r, &s| a[r][col].abs().total_cmp(&a[s][col].abs()))
            .unwrap();
        if a[piv][col].abs() < PIVOT_EPS {
            continue;
        }
        a.swap(row, piv);
        let d0 = a[row][col];
        a[row].iter_mut().for_each(|e| *e /= d0);

        let prow = a[row].clone();
        for (i, r) in a.iter_mut().enumerate() {
            if i != row && r[col].abs() > PIVOT_EPS {
                let f = r[col];
                r.iter_mut()
                    .enumerate()
                    .for_each(|(j, e)| *e -= f * prow[j]);
            }
        }
        pivots.push(col);
        row += 1;
    }

    pivots
}

struct State {
    u: Vec<i64>,
    w: Vec<f64>,
    expr: Vec<(f64, Vec<f64>)>,
    suffix_min: Vec<f64>,
    x: Vec<i64>,
    c0: f64,
    best: f64,
}

impl State {
    const EPS: f64 = 1e-9;

    fn new(buttons: &[u16], targets: &[u16]) -> Self {
        let m = targets.len();
        let n = buttons.len();

        // augmented matrix [A | b]
        let mut a = vec![vec![0.0f64; n + 1]; m];
        for (j, b) in buttons.iter().enumerate() {
            for (i, row) in a.iter_mut().enumerate() {
                if b & (1 << i) > 0 {
                    row[j] = 1.0;
                }
            }
        }
        for i in 0..m {
            a[i][n] = f64::from(targets[i]);
        }

        let pivots = gaussian_elimilation(&mut a);

        for r in &a[pivots.len()..m] {
            assert!(r[n].abs() <= Self::EPS, "no feasible solution");
        }

        // free variables in ascending upper bound order
        let mut free: Vec<(usize, i64)> = (0..n)
            .filter(|c| !pivots.contains(c))
            .map(|c| {
                let mut ub = u16::MAX;
                for (i, &t) in targets.iter().enumerate() {
                    if buttons[c] & (1 << i) > 0 {
                        ub = ub.min(t);
                    }
                }
                (c, ub as i64)
            })
            .collect();
        let d = free.len();
        free.sort_unstable_by_key(|e| e.1);
        let u: Vec<i64> = free.iter().map(|e| e.1).collect();

        // x_pivot = c - Σ coefₖ·xₖ, cost = C0 + Σ wₖ·xₖ
        let expr: Vec<(f64, Vec<f64>)> = (0..pivots.len())
            .map(|i| (a[i][n], free.iter().map(|&(f, _)| a[i][f]).collect()))
            .collect();
        let c0: f64 = expr.iter().map(|e| e.0).sum();
        let w: Vec<f64> = (0..d)
            .map(|k| 1.0 - expr.iter().map(|e| e.1[k]).sum::<f64>())
            .collect();
        let mut suffix_min = vec![0.0; d + 1];
        for i in (0..d).rev() {
            suffix_min[i] = suffix_min[i + 1] + (w[i] * u[i] as f64).min(0.0);
        }

        Self {
            u,
            w,
            expr,
            suffix_min,
            x: vec![0; d],
            c0,
            best: f64::INFINITY,
        }
    }

    fn dfs(&mut self, idx: usize, cost: f64) {
        // pruning by cost lower bound
        if self.c0 + cost + self.suffix_min[idx] >= self.best - Self::EPS {
            return;
        }

        if idx == self.x.len() {
            for (c, coefs) in &self.expr {
                let mut v = *c;
                for (k, &cf) in coefs.iter().enumerate() {
                    v -= cf * self.x[k] as f64;
                }
                if v < -Self::EPS || (v - v.round()).abs() > Self::EPS {
                    return;
                }
            }
            self.best = self.best.min(self.c0 + cost);
            return;
        }

        // tighten x[idx] upper bound
        let mut hi = self.u[idx];
        for (c, coefs) in &self.expr {
            let mut resid = *c;
            for (&c, &x) in coefs.iter().zip(&self.x).take(idx) {
                resid -= c * x as f64;
            }
            let mut others_min = 0.0;
            for (&c, &u) in coefs.iter().zip(&self.u).skip(idx + 1) {
                if c < 0.0 {
                    others_min += c * u as f64;
                }
            }
            let cf = coefs[idx];
            let self_min = if cf < 0.0 {
                cf * self.u[idx] as f64
            } else {
                0.0
            };
            if resid < others_min + self_min - Self::EPS {
                return;
            }
            if cf > Self::EPS {
                let bound = ((resid - others_min) / cf + Self::EPS).floor() as i64;
                if bound < hi {
                    hi = bound;
                }
            }
        }
        if hi < 0 {
            return;
        }
        for v in 0..=hi {
            self.x[idx] = v;
            self.dfs(idx + 1, cost + self.w[idx] * v as f64);
        }
        self.x[idx] = 0;
    }
}

fn solve_machine(buttons: &[u16], targets: &[u16]) -> i64 {
    let mut state = State::new(buttons, targets);
    state.dfs(0, 0.0);
    if state.best.is_finite() {
        state.best.round() as i64
    } else {
        eprintln!("no feasible solution for {buttons:?} {targets:?}");
        0
    }
}

fn part2(manuals: &[(Lights, Buttons, Joltages)]) -> i64 {
    manuals
        .par_iter()
        .map(|(_, buttons, joltages)| solve_machine(buttons, joltages))
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
