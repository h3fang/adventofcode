use ahash::AHashSet as HashSet;

const WIDTH: usize = 5;

#[derive(Clone, Copy)]
struct Grid(u64);

impl Grid {
    const fn mask(x: usize, y: usize) -> u64 {
        1 << ((y * 8) + x)
    }

    const fn get_bit(&self, mask: u64) -> bool {
        self.0 & mask > 0
    }

    fn set_bit(&mut self, mask: u64, set: bool) {
        if set {
            self.0 |= mask;
        } else {
            self.0 &= !mask;
        }
    }

    fn count_bugs(pos: &[(&Grid, &[(usize, usize)])]) -> usize {
        let mut c = 0;
        for &(g, ps) in pos {
            for &(x, y) in ps {
                if g.get_bit(Self::mask(x, y)) {
                    c += 1;
                }
                if c > 3 {
                    return c;
                }
            }
        }
        c
    }

    fn count_adjacent_bugs(&self, prev: &Grid, next: &Grid, x: usize, y: usize) -> usize {
        match (x, y) {
            (1, 1) => Self::count_bugs(&[(self, &[(1, 2), (2, 1)]), (prev, &[(2, 3), (3, 2)])]),
            (5, 1) => Self::count_bugs(&[(self, &[(5, 2), (4, 1)]), (prev, &[(4, 3), (3, 2)])]),
            (1, 5) => Self::count_bugs(&[(self, &[(1, 4), (2, 5)]), (prev, &[(2, 3), (3, 4)])]),
            (5, 5) => Self::count_bugs(&[(self, &[(4, 5), (5, 4)]), (prev, &[(4, 3), (3, 4)])]),
            (1, y) => Self::count_bugs(&[
                (self, &[(1, y - 1), (1, y + 1), (1 + 1, y)]),
                (prev, &[(2, 3)]),
            ]),
            (x, 1) => Self::count_bugs(&[
                (self, &[(x - 1, 1), (x, 1 + 1), (x + 1, 1)]),
                (prev, &[(3, 2)]),
            ]),
            (5, y) => Self::count_bugs(&[
                (self, &[(5, y - 1), (5, y + 1), (5 - 1, y)]),
                (prev, &[(4, 3)]),
            ]),
            (x, 5) => Self::count_bugs(&[
                (self, &[(x - 1, 5), (x, 5 - 1), (x + 1, 5)]),
                (prev, &[(3, 4)]),
            ]),
            (3, 2) => Self::count_bugs(&[
                (self, &[(x - 1, y), (x, y - 1), (x + 1, y)]),
                (next, &[(1, 1), (2, 1), (3, 1), (4, 1), (5, 1)]),
            ]),
            (2, 3) => Self::count_bugs(&[
                (self, &[(x, y - 1), (x, y + 1), (x - 1, y)]),
                (next, &[(1, 1), (1, 2), (1, 3), (1, 4), (1, 5)]),
            ]),
            (4, 3) => Self::count_bugs(&[
                (self, &[(x, y + 1), (x, y - 1), (x + 1, y)]),
                (next, &[(5, 1), (5, 2), (5, 3), (5, 4), (5, 5)]),
            ]),
            (3, 4) => Self::count_bugs(&[
                (self, &[(x - 1, y), (x, y + 1), (x + 1, y)]),
                (next, &[(1, 5), (2, 5), (3, 5), (4, 5), (5, 5)]),
            ]),
            (x, y) => {
                Self::count_bugs(&[(self, &[(x - 1, y), (x, y - 1), (x, y + 1), (x + 1, y)])])
            }
        }
    }

    fn step(&mut self) {
        let g = Grid(self.0);
        for y in 1..=WIDTH {
            for x in 1..=WIDTH {
                let m = Self::mask(x, y);
                let b = g.get_bit(m);
                let adj = [(x - 1, y), (x, y - 1), (x, y + 1), (x + 1, y)]
                    .into_iter()
                    .filter(|&(x, y)| g.get_bit(Self::mask(x, y)))
                    .count();
                if b && adj != 1 {
                    self.set_bit(m, false);
                } else if !b && (1..=2).contains(&adj) {
                    self.set_bit(m, true);
                }
            }
        }
    }

    fn step_recursive(&mut self, prev: &Grid, next: &Grid) {
        let g = Grid(self.0);
        for y in 1..=WIDTH {
            for x in 1..=WIDTH {
                if x == 3 && y == 3 {
                    continue;
                }
                let m = Self::mask(x, y);
                let b = g.get_bit(m);
                let adj = g.count_adjacent_bugs(prev, next, x, y);
                if b && adj != 1 {
                    self.set_bit(m, false);
                } else if !b && (1..=2).contains(&adj) {
                    self.set_bit(m, true);
                }
            }
        }
    }

    fn biodiversity(&self) -> usize {
        let mut result = 0;
        let mut pow = 1;
        for y in 1..=WIDTH {
            for x in 1..=WIDTH {
                if self.get_bit(Self::mask(x, y)) {
                    result += pow;
                }
                pow <<= 1;
            }
        }
        result
    }

    fn parse(data: &str) -> Self {
        let mut grid = 0u64;
        for (y, line) in data.lines().enumerate() {
            for (x, c) in line.trim().chars().enumerate() {
                if c == '#' {
                    grid |= Grid::mask(x + 1, y + 1);
                }
            }
        }
        Grid(grid)
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 1..=WIDTH {
            for x in 1..=WIDTH {
                let m = Self::mask(x, y);
                let b = self.get_bit(m);
                write!(f, "{}", if b { '#' } else { '.' })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn part1(mut grid: Grid) -> usize {
    // println!("{}", grid);
    let mut set = HashSet::new();
    set.insert(grid.0);
    loop {
        grid.step();
        // println!("{}", grid);
        if set.contains(&grid.0) {
            break;
        }
        set.insert(grid.0);
    }
    grid.biodiversity()
}

struct RecursiveGrid {
    grids: Vec<Grid>,
    min_level: i32,
    max_level: i32,
}

impl RecursiveGrid {
    fn new(mut g: Grid) -> Self {
        g.set_bit(Grid::mask(3, 3), false);
        Self {
            grids: vec![g],
            min_level: 0,
            max_level: 0,
        }
    }
    fn grid_index(level: i32) -> usize {
        if level >= 0 {
            (level * 2) as usize
        } else {
            (level.abs() * 2 - 1) as usize
        }
    }

    fn step(&mut self) {
        let n = (self.max_level - self.min_level + 5) as usize;
        if self.grids.len() < n {
            let n = n - self.grids.len();
            self.grids.extend(vec![Grid(0); n]);
        }
        let old = self.grids.clone();
        self.min_level -= 1;
        self.max_level += 1;
        for level in self.min_level..=self.max_level {
            let i = Self::grid_index(level);
            let prev = old[Self::grid_index(level - 1)];
            let next = old[Self::grid_index(level + 1)];
            self.grids[i].step_recursive(&prev, &next);
        }

        while self.grids[Self::grid_index(self.min_level)].0 == 0 {
            self.min_level += 1;
        }

        while self.grids[Self::grid_index(self.max_level)].0 == 0 {
            self.max_level -= 1;
        }
    }

    fn count_bugs(&self) -> usize {
        self.grids.iter().map(|g| g.0.count_ones()).sum::<u32>() as usize
    }
}

fn part2(grid: Grid, miniutes: usize) -> usize {
    let mut g = RecursiveGrid::new(grid);
    for _ in 0..miniutes {
        g.step();
    }
    // for level in g.min_level..=g.max_level {
    //     let i = RecursiveGrid::grid_index(level);
    //     println!("{}\n{}", level, g.grids[i]);
    // }
    g.count_bugs()
}

pub fn main() {
    let data = std::fs::read_to_string("data/2019/day24").unwrap();
    let grid = Grid::parse(&data);

    println!("day 24 part1: {}", part1(Grid(grid.0)));
    println!("day 24 part2: {}", part2(grid, 200));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "....#
        #..#.
        #..##
        ..#..
        #....";
        let grid = Grid::parse(data);
        assert_eq!(2129920, part1(Grid(grid.0)));
        assert_eq!(99, part2(grid, 10));
    }
}
