use arrayvec::ArrayVec;

const CYCLES: i64 = 6;

#[derive(Clone)]
struct Grid {
    array: Vec<char>,
    size: (i64, i64, i64),
    cycles: i64,
}

impl Grid {
    fn new(sx: i64, sy: i64, sz: i64) -> Self {
        let array = vec!['.'; (sx * sy * sz) as usize];
        Self {
            array,
            size: (sx, sy, sz),
            cycles: 0,
        }
    }

    #[inline]
    fn index(&self, i: i64, j: i64, k: i64) -> usize {
        (i * self.size.1 * self.size.2 + j * self.size.2 + k) as usize
    }

    #[inline]
    fn get(&self, i: i64, j: i64, k: i64) -> char {
        let idx = self.index(i, j, k);
        self.array[idx]
    }

    #[inline]
    fn set(&mut self, i: i64, j: i64, k: i64, value: char) {
        let idx = self.index(i, j, k);
        self.array[idx] = value;
    }

    fn neighbors(&self, i: i64, j: i64, k: i64) -> ArrayVec<(i64, i64, i64), 26> {
        ArrayVec::from([
            (i - 1, j - 1, k - 1),
            (i - 1, j - 1, k),
            (i - 1, j - 1, k + 1),
            (i - 1, j, k - 1),
            (i - 1, j, k),
            (i - 1, j, k + 1),
            (i - 1, j + 1, k - 1),
            (i - 1, j + 1, k),
            (i - 1, j + 1, k + 1),
            (i, j - 1, k - 1),
            (i, j - 1, k),
            (i, j - 1, k + 1),
            (i, j, k - 1),
            (i, j, k + 1),
            (i, j + 1, k - 1),
            (i, j + 1, k),
            (i, j + 1, k + 1),
            (i + 1, j - 1, k - 1),
            (i + 1, j - 1, k),
            (i + 1, j - 1, k + 1),
            (i + 1, j, k - 1),
            (i + 1, j, k),
            (i + 1, j, k + 1),
            (i + 1, j + 1, k - 1),
            (i + 1, j + 1, k),
            (i + 1, j + 1, k + 1),
        ])
    }

    fn cycle(&mut self) {
        let grid = self.clone();
        for i in 1..grid.size.0 - 1 {
            for j in 1..grid.size.1 - 1 {
                for k in 1..2 * CYCLES + 2 {
                    match grid.get(i, j, k) {
                        '#' => {
                            let active = grid
                                .neighbors(i, j, k)
                                .iter()
                                .filter(|(i, j, k)| grid.get(*i, *j, *k) == '#')
                                .count();
                            if active != 2 && active != 3 {
                                self.set(i, j, k, '.');
                            }
                        }
                        '.' => {
                            let active = grid
                                .neighbors(i, j, k)
                                .iter()
                                .filter(|(i, j, k)| grid.get(*i, *j, *k) == '#')
                                .count();
                            if active == 3 {
                                self.set(i, j, k, '#');
                            }
                        }
                        _ => panic!("invalid cell"),
                    }
                }
            }
        }
        self.cycles += 1;
    }
}

#[derive(Clone)]
struct Grid4 {
    array: Vec<char>,
    size: (i64, i64, i64, i64),
    cycles: i64,
}

impl Grid4 {
    fn new(sx: i64, sy: i64, sz: i64, sw: i64) -> Self {
        let array = vec!['.'; (sx * sy * sz * sw) as usize];
        Self {
            array,
            size: (sx, sy, sz, sw),
            cycles: 0,
        }
    }

    #[inline]
    fn index(&self, i: i64, j: i64, k: i64, w: i64) -> usize {
        (i * self.size.1 * self.size.2 * self.size.3
            + j * self.size.2 * self.size.3
            + k * self.size.3
            + w) as usize
    }

    fn get(&self, i: i64, j: i64, k: i64, w: i64) -> char {
        let idx = self.index(i, j, k, w);
        if idx >= self.array.len() {
            println!("({i},{j},{k},{w})");
        }
        self.array[idx]
    }

    fn set(&mut self, i: i64, j: i64, k: i64, w: i64, value: char) {
        let idx = self.index(i, j, k, w);
        if idx >= self.array.len() {
            println!("({i},{j},{k},{w})");
        }
        self.array[idx] = value;
    }

    fn neighbors(&self, i: i64, j: i64, k: i64, w: i64) -> ArrayVec<(i64, i64, i64, i64), 80> {
        ArrayVec::from([
            (i - 1, j - 1, k - 1, w - 1),
            (i - 1, j - 1, k - 1, w),
            (i - 1, j - 1, k - 1, w + 1),
            (i - 1, j - 1, k, w - 1),
            (i - 1, j - 1, k, w),
            (i - 1, j - 1, k, w + 1),
            (i - 1, j - 1, k + 1, w - 1),
            (i - 1, j - 1, k + 1, w),
            (i - 1, j - 1, k + 1, w + 1),
            (i - 1, j, k - 1, w - 1),
            (i - 1, j, k - 1, w),
            (i - 1, j, k - 1, w + 1),
            (i - 1, j, k, w - 1),
            (i - 1, j, k, w),
            (i - 1, j, k, w + 1),
            (i - 1, j, k + 1, w - 1),
            (i - 1, j, k + 1, w),
            (i - 1, j, k + 1, w + 1),
            (i - 1, j + 1, k - 1, w - 1),
            (i - 1, j + 1, k - 1, w),
            (i - 1, j + 1, k - 1, w + 1),
            (i - 1, j + 1, k, w - 1),
            (i - 1, j + 1, k, w),
            (i - 1, j + 1, k, w + 1),
            (i - 1, j + 1, k + 1, w - 1),
            (i - 1, j + 1, k + 1, w),
            (i - 1, j + 1, k + 1, w + 1),
            (i, j - 1, k - 1, w - 1),
            (i, j - 1, k - 1, w),
            (i, j - 1, k - 1, w + 1),
            (i, j - 1, k, w - 1),
            (i, j - 1, k, w),
            (i, j - 1, k, w + 1),
            (i, j - 1, k + 1, w - 1),
            (i, j - 1, k + 1, w),
            (i, j - 1, k + 1, w + 1),
            (i, j, k - 1, w - 1),
            (i, j, k - 1, w),
            (i, j, k - 1, w + 1),
            (i, j, k, w - 1),
            (i, j, k, w + 1),
            (i, j, k + 1, w - 1),
            (i, j, k + 1, w),
            (i, j, k + 1, w + 1),
            (i, j + 1, k - 1, w - 1),
            (i, j + 1, k - 1, w),
            (i, j + 1, k - 1, w + 1),
            (i, j + 1, k, w - 1),
            (i, j + 1, k, w),
            (i, j + 1, k, w + 1),
            (i, j + 1, k + 1, w - 1),
            (i, j + 1, k + 1, w),
            (i, j + 1, k + 1, w + 1),
            (i + 1, j - 1, k - 1, w - 1),
            (i + 1, j - 1, k - 1, w),
            (i + 1, j - 1, k - 1, w + 1),
            (i + 1, j - 1, k, w - 1),
            (i + 1, j - 1, k, w),
            (i + 1, j - 1, k, w + 1),
            (i + 1, j - 1, k + 1, w - 1),
            (i + 1, j - 1, k + 1, w),
            (i + 1, j - 1, k + 1, w + 1),
            (i + 1, j, k - 1, w - 1),
            (i + 1, j, k - 1, w),
            (i + 1, j, k - 1, w + 1),
            (i + 1, j, k, w - 1),
            (i + 1, j, k, w),
            (i + 1, j, k, w + 1),
            (i + 1, j, k + 1, w - 1),
            (i + 1, j, k + 1, w),
            (i + 1, j, k + 1, w + 1),
            (i + 1, j + 1, k - 1, w - 1),
            (i + 1, j + 1, k - 1, w),
            (i + 1, j + 1, k - 1, w + 1),
            (i + 1, j + 1, k, w - 1),
            (i + 1, j + 1, k, w),
            (i + 1, j + 1, k, w + 1),
            (i + 1, j + 1, k + 1, w - 1),
            (i + 1, j + 1, k + 1, w),
            (i + 1, j + 1, k + 1, w + 1),
        ])
    }

    fn cycle(&mut self) {
        let grid = self.clone();
        let m = CYCLES - self.cycles;
        for i in m..grid.size.0 - m {
            for j in m..grid.size.1 - m {
                for k in m..grid.size.2 - m {
                    for w in m..=1 + CYCLES {
                        match grid.get(i, j, k, w) {
                            '#' => {
                                let active = grid
                                    .neighbors(i, j, k, w)
                                    .iter()
                                    .filter(|(i, j, k, w)| grid.get(*i, *j, *k, *w) == '#')
                                    .count();
                                if active != 2 && active != 3 {
                                    self.set(i, j, k, w, '.');
                                }
                            }
                            '.' => {
                                let active = grid
                                    .neighbors(i, j, k, w)
                                    .iter()
                                    .filter(|(i, j, k, w)| grid.get(*i, *j, *k, *w) == '#')
                                    .count();
                                if active == 3 {
                                    self.set(i, j, k, w, '#');
                                }
                            }
                            _ => panic!("invalid cell"),
                        }
                    }
                    let mirror = self.get(i, j, k, CYCLES);
                    self.set(i, j, k, 2 + CYCLES, mirror);
                }
            }
        }
        self.cycles += 1;
    }
}

fn parse(content: &str) -> (Grid, Grid4) {
    let layer = content
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let m = layer.len() as i64;
    let n = layer[0].len() as i64;
    let mut grid = Grid::new(m + 2 + 2 * CYCLES, n + 2 + 2 * CYCLES, 1 + 2 + 2 * CYCLES);
    let mut grid4 = Grid4::new(
        m + 2 + 2 * CYCLES,
        n + 2 + 2 * CYCLES,
        1 + 2 + 2 * CYCLES,
        1 + 2 + 2 * CYCLES,
    );
    for (i, row) in layer.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            grid.set(i as i64 + 1 + CYCLES, j as i64 + 1 + CYCLES, 1 + CYCLES, *c);
            grid4.set(
                i as i64 + 1 + CYCLES,
                j as i64 + 1 + CYCLES,
                1 + CYCLES,
                1 + CYCLES,
                *c,
            );
        }
    }

    (grid, grid4)
}

fn part1(grid: &mut Grid) -> usize {
    for _ in 0..CYCLES {
        grid.cycle();
    }
    grid.array.iter().filter(|c| **c == '#').count()
}

fn part2(grid: &mut Grid4) -> usize {
    for _ in 0..CYCLES {
        grid.cycle();
    }
    let mut middle = 0;
    let mut half = 0;
    for i in 1..grid.size.0 - 1 {
        for j in 1..grid.size.1 - 1 {
            for k in 1..grid.size.2 - 1 {
                for w in 1..1 + CYCLES {
                    if grid.get(i, j, k, w) == '#' {
                        half += 1;
                    }
                }
                if grid.get(i, j, k, 1 + CYCLES) == '#' {
                    middle += 1;
                }
            }
        }
    }
    middle + 2 * half
}

pub fn main() {
    let (mut grid, mut grid4) = parse(&std::fs::read_to_string("data/2020/day17").unwrap());

    // part 1
    println!("day 17 part1: {}", part1(&mut grid));

    // part 2
    println!("day 17 part2: {}", part2(&mut grid4));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small1() {
        let (mut grid, mut grid4) = parse(&std::fs::read_to_string("data/2020/day17-1").unwrap());
        assert_eq!(112, part1(&mut grid));
        assert_eq!(848, part2(&mut grid4));
    }
}
