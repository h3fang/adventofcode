use std::collections::VecDeque;

use rayon::prelude::*;

fn parse(data: &str) -> Vec<&[u8]> {
    data.trim().lines().map(|l| l.as_bytes()).collect()
}

fn bounce(grid: &[&[u8]], start: (i32, i32, i32)) -> usize {
    let (m, n) = (grid.len(), grid[0].len());
    let mut visited = vec![vec![[false; 4]; n]; m];
    let mut q = VecDeque::from([start]);
    while let Some((i, j, d)) = q.pop_front() {
        if i < 0
            || j < 0
            || i >= m as i32
            || j >= n as i32
            || visited[i as usize][j as usize][d as usize]
        {
            continue;
        }
        visited[i as usize][j as usize][d as usize] = true;
        match grid[i as usize][j as usize] {
            b'.' => {
                let (di, dj) = [(0, 1), (1, 0), (0, -1), (-1, 0)][d as usize];
                q.push_back((i + di, j + dj, d));
            }
            b'-' => match d {
                0 => q.push_back((i, j + 1, 0)),
                2 => q.push_back((i, j - 1, 2)),
                1 | 3 => {
                    q.push_back((i, j + 1, 0));
                    q.push_back((i, j - 1, 2));
                }
                _ => unreachable!(),
            },
            b'|' => match d {
                1 => q.push_back((i + 1, j, 1)),
                3 => q.push_back((i - 1, j, 3)),
                0 | 2 => {
                    q.push_back((i + 1, j, 1));
                    q.push_back((i - 1, j, 3));
                }
                _ => unreachable!(),
            },
            b'/' => match d {
                0 => q.push_back((i - 1, j, 3)),
                1 => q.push_back((i, j - 1, 2)),
                2 => q.push_back((i + 1, j, 1)),
                3 => q.push_back((i, j + 1, 0)),
                _ => unreachable!(),
            },
            b'\\' => match d {
                0 => q.push_back((i + 1, j, 1)),
                1 => q.push_back((i, j + 1, 0)),
                2 => q.push_back((i - 1, j, 3)),
                3 => q.push_back((i, j - 1, 2)),
                _ => unreachable!(),
            },
            _ => {}
        }
    }
    visited
        .iter()
        .flatten()
        .filter(|c| c.iter().any(|e| *e))
        .count()
}

fn part1(grid: &[&[u8]]) -> usize {
    bounce(grid, (0, 0, 0))
}

fn part2(grid: &[&[u8]]) -> usize {
    let (m, n) = (grid.len() as i32, grid[0].len() as i32);
    let m1 = (0..m)
        .into_par_iter()
        .map(|i| bounce(grid, (i, 0, 0)).max(bounce(grid, (i, n - 1, 2))))
        .max()
        .unwrap();
    let m2 = (0..n)
        .into_par_iter()
        .map(|j| bounce(grid, (0, j, 1)).max(bounce(grid, (m - 1, j, 3))))
        .max()
        .unwrap();
    m1.max(m2)
}

pub fn main() {
    let data = std::fs::read_to_string("data/2023/day16").unwrap();
    let grid = parse(&data);
    println!("part1: {}", part1(&grid));
    println!("part2: {}", part2(&grid));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = r"
.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
        let grid = parse(data);
        assert_eq!(46, part1(&grid));
        assert_eq!(51, part2(&grid));
    }
}
