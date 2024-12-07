struct Map {
    width: usize,
    height: usize,
    grid: Vec<bool>,
}

impl Map {
    /// The elves will be at most one cell away from each other,
    /// so a (3 * width) by (3 * height) grid will be sufficient.
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            grid: vec![false; width * height * 9],
        }
    }

    #[inline]
    fn index(&self, i: usize, j: usize) -> usize {
        3 * self.width * (self.height + i) + j + self.width
    }

    #[inline]
    fn neighbors(&self, k: usize) -> [usize; 8] {
        let w = self.width * 3;
        [
            k - w - 1,
            k - w,
            k - w + 1,
            k - 1,
            k + 1,
            k + w - 1,
            k + w,
            k + w + 1,
        ]
    }

    #[inline]
    fn direction(&self, k: usize, d: u8) -> [usize; 3] {
        let w = self.width * 3;
        match d {
            0 => [k - w - 1, k - w, k - w + 1],
            1 => [k + w - 1, k + w, k + w + 1],
            2 => [k - w - 1, k - 1, k + w - 1],
            3 => [k - w + 1, k + 1, k + w + 1],
            _ => unreachable!(),
        }
    }

    #[inline]
    fn propose(&self, d: u8, k: usize) -> Option<usize> {
        if self.neighbors(k).into_iter().any(|p| self.grid[p]) {
            for d1 in d..d + 4 {
                let ps = self.direction(k, d1 % 4);
                if ps.into_iter().all(|p| !self.grid[p]) {
                    return Some(ps[1]);
                }
            }
        }
        None
    }

    fn spread(&mut self, d: u8) -> bool {
        // only the adjacent elve in the proposed direction can propose the same position
        let mut g = self.grid.clone();
        let mut moved = 0;
        for (k, &c) in self.grid.iter().enumerate() {
            if !c {
                continue;
            }
            if let Some(p) = self.propose(d, k) {
                if g[p] {
                    g[p] = false;
                    g[2 * p - k] = true;
                    moved -= 1;
                } else {
                    g[k] = false;
                    g[p] = true;
                    moved += 1;
                }
            }
        }
        self.grid = g;
        moved > 0
    }

    fn boundbox(&self) -> ((usize, usize), (usize, usize)) {
        let mut min = (usize::MAX, usize::MAX);
        let mut max = (usize::MIN, usize::MIN);
        let w = self.width * 3;
        for (k, &c) in self.grid.iter().enumerate() {
            if !c {
                continue;
            }
            let i = k / w;
            let j = k % w;
            min.0 = min.0.min(i);
            min.1 = min.1.min(j);
            max.0 = max.0.max(i);
            max.1 = max.1.max(j);
        }
        (min, max)
    }

    #[allow(unused)]
    fn print(&self) {
        let (min, max) = self.boundbox();
        for i in min.0..=max.0 {
            let k0 = i * 3 * self.width;
            let r = self.grid[k0 + min.1..=k0 + max.1]
                .iter()
                .map(|&c| if c { b'#' } else { b'.' })
                .collect::<Vec<_>>();
            println!("{}", std::str::from_utf8(&r).unwrap());
        }
    }
}

fn parse(data: &str) -> Vec<&[u8]> {
    data.trim().lines().map(|line| line.as_bytes()).collect()
}

fn solve(map: &[&[u8]]) -> (usize, usize) {
    let height = map.len();
    let width = map[0].len();
    let mut m = Map::new(width, height);
    let mut n_evles = 0;
    for (i, r) in map.iter().enumerate() {
        for (j, c) in r.iter().enumerate() {
            if *c == b'#' {
                let k = m.index(i, j);
                m.grid[k] = true;
                n_evles += 1;
            }
        }
    }
    let mut p1 = 0;
    let mut p2 = 0;
    for i in 0.. {
        let moved = m.spread((i % 4) as u8);
        if !moved && p2 == 0 {
            p2 = i + 1;
        }
        if i == 9 {
            let (min, max) = m.boundbox();
            let area = (max.0 - min.0 + 1) * (max.1 - min.1 + 1);
            p1 = area - n_evles;
        }
        if p1 > 0 && p2 > 0 {
            break;
        }
    }
    // m.print();
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
        let map = parse(data);
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
        let map = parse(data);
        let (p1, p2) = solve(&map);
        assert_eq!(25, p1);
        assert_eq!(4, p2);
    }
}
