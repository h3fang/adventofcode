use std::collections::HashSet;

use arrayvec::ArrayVec;

fn neighbors(x: usize, y: usize, rows: usize, cols: usize) -> ArrayVec<(usize, usize), 8> {
    let mut r = ArrayVec::new();
    if x > 0 {
        if y > 0 {
            r.push((x - 1, y - 1));
        }
        r.push((x - 1, y));
        if y < rows - 1 {
            r.push((x - 1, y + 1));
        }
    }

    if y > 0 {
        r.push((x, y - 1));
    }
    if y < rows - 1 {
        r.push((x, y + 1));
    }

    if x < cols - 1 {
        if y > 0 {
            r.push((x + 1, y - 1));
        }
        r.push((x + 1, y));
        if y < rows - 1 {
            r.push((x + 1, y + 1));
        }
    }

    r
}

fn step(grid: &mut [Vec<u32>]) -> usize {
    fn recursive(grid: &mut [Vec<u32>], mut flashed: HashSet<(usize, usize)>) {
        let rows = grid.len();
        let cols = grid[0].len();
        let mut flashing = vec![];
        for (y, row) in grid.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if *cell > 9 && !flashed.contains(&(x, y)) {
                    flashing.push((x, y));
                }
            }
        }
        if !flashing.is_empty() {
            for (x, y) in flashing {
                for (xn, yn) in neighbors(x, y, rows, cols) {
                    grid[yn][xn] += 1;
                }
                flashed.insert((x, y));
            }
            recursive(grid, flashed);
        }
    }
    grid.iter_mut().flatten().for_each(|c| *c += 1);
    recursive(grid, HashSet::new());
    let mut result = 0;
    grid.iter_mut().flatten().for_each(|c| {
        if *c > 9 {
            *c = 0;
            result += 1;
        }
    });
    result
}

fn part1(grid: &[Vec<u32>]) -> usize {
    let mut grid = grid.to_owned();
    (0..100).map(|_| step(&mut grid)).sum()
}

fn part2(grid: &[Vec<u32>]) -> usize {
    let mut grid = grid.to_owned();
    let n = grid.len() * grid[0].len();
    let mut i = 1;
    loop {
        if step(&mut grid) == n {
            return i;
        }
        i += 1;
    }
}

pub fn main() {
    let data = std::fs::read_to_string("data/2021/day11").unwrap();

    let grid = data
        .lines()
        .map(|s| {
            s.trim()
                .as_bytes()
                .iter()
                .map(|e| (e - b'0') as u32)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!("day11 part1: {}", part1(&grid));
    println!("day11 part2: {}", part2(&grid));
}
