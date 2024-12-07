use std::collections::VecDeque;

fn parse(data: &str) -> Vec<Vec<u8>> {
    data.trim()
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect()
}

fn connected(grid: &[Vec<u8>], (i, j): (usize, usize)) -> Vec<(i32, i32)> {
    let c = grid[i][j];
    let (i, j) = (i as i32, j as i32);
    match c {
        b'|' => vec![(i - 1, j), (i + 1, j)],
        b'-' => vec![(i, j - 1), (i, j + 1)],
        b'L' => vec![(i - 1, j), (i, j + 1)],
        b'J' => vec![(i - 1, j), (i, j - 1)],
        b'7' => vec![(i, j - 1), (i + 1, j)],
        b'F' => vec![(i, j + 1), (i + 1, j)],
        _ => vec![],
    }
}

fn find_type_of_start(grid: &mut [Vec<u8>], (i, j): (usize, usize)) -> bool {
    let (m, n) = (grid.len() as i32, grid[0].len() as i32);
    for c in [b'|', b'-', b'L', b'J', b'7', b'F'] {
        grid[i][j] = c;
        if connected(grid, (i, j)).into_iter().all(|(i1, j1)| {
            i1 >= 0
                && j1 >= 0
                && i1 < m
                && j1 < n
                && connected(grid, (i1 as usize, j1 as usize)).contains(&(i as i32, j as i32))
        }) {
            return true;
        }
    }
    false
}

fn part1(grid: &mut [Vec<u8>], dist: &mut [Vec<u32>]) -> u32 {
    let (mut i0, mut j0) = (usize::MAX, usize::MAX);
    for (i, row) in grid.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c == b'S' {
                (i0, j0) = (i, j);
                break;
            }
        }
    }

    assert!(find_type_of_start(grid, (i0, j0)));

    dist[i0][j0] = 0;
    let mut q = VecDeque::from([(i0, j0, 0)]);
    let mut max = 0;
    while let Some((i, j, d)) = q.pop_front() {
        max = d;
        connected(grid, (i, j)).into_iter().for_each(|(i, j)| {
            let (i, j) = (i as usize, j as usize);
            if dist[i][j] == u32::MAX {
                dist[i][j] = d + 1;
                q.push_back((i, j, d + 1));
            }
        });
    }
    max
}

fn part2(grid: &[Vec<u8>], dist: &[Vec<u32>]) -> usize {
    let mut result: usize = 0;
    for (i, row) in dist.iter().enumerate() {
        for (j, &d) in row.iter().enumerate() {
            if d != u32::MAX {
                continue;
            }
            let (mut i1, mut j1, mut crosses) = (i, j, 0);
            loop {
                if dist[i1][j1] != u32::MAX && grid[i1][j1] != b'7' && grid[i1][j1] != b'L' {
                    crosses += 1;
                }
                if i1 == 0 || j1 == 0 {
                    break;
                }
                i1 -= 1;
                j1 -= 1;
            }
            result += crosses % 2;
        }
    }
    result
}

pub fn main() {
    let data = std::fs::read_to_string("data/2023/day10").unwrap();
    let mut grid = parse(&data);
    let mut dist = vec![vec![u32::MAX; grid[0].len()]; grid.len()];
    println!("part1: {}", part1(&mut grid, &mut dist));
    println!("part2: {}", part2(&grid, &dist));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "
.....
.S-7.
.|.|.
.L-J.
.....
";
        let mut grid = parse(data);
        let mut dist = vec![vec![u32::MAX; grid[0].len()]; grid.len()];
        assert_eq!(4, part1(&mut grid, &mut dist));
    }

    #[test]
    fn case2() {
        let data = "
..F7.
.FJ|.
SJ.L7
|F--J
LJ...
";
        let mut grid = parse(data);
        let mut dist = vec![vec![u32::MAX; grid[0].len()]; grid.len()];
        assert_eq!(8, part1(&mut grid, &mut dist));
    }

    #[test]
    fn case3() {
        let data = "
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
";
        let mut grid = parse(data);
        let mut dist = vec![vec![u32::MAX; grid[0].len()]; grid.len()];
        part1(&mut grid, &mut dist);
        assert_eq!(4, part2(&grid, &dist));
    }

    #[test]
    fn case4() {
        let data = "
..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........
";
        let mut grid = parse(data);
        let mut dist = vec![vec![u32::MAX; grid[0].len()]; grid.len()];
        part1(&mut grid, &mut dist);
        assert_eq!(4, part2(&grid, &dist));
    }

    #[test]
    fn case5() {
        let data = "
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
";
        let mut grid = parse(data);
        let mut dist = vec![vec![u32::MAX; grid[0].len()]; grid.len()];
        part1(&mut grid, &mut dist);
        assert_eq!(8, part2(&grid, &dist));
    }

    #[test]
    fn case6() {
        let data = "
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
";
        let mut grid = parse(data);
        let mut dist = vec![vec![u32::MAX; grid[0].len()]; grid.len()];
        part1(&mut grid, &mut dist);
        assert_eq!(10, part2(&grid, &dist));
    }
}
