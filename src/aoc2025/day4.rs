use ahash::{HashSet, HashSetExt};

fn parse(data: &str) -> Vec<Vec<u8>> {
    data.trim().lines().map(|s| s.as_bytes().to_vec()).collect()
}

fn is_accessible(grid: &[Vec<u8>], i: i32, j: i32) -> bool {
    let (m, n) = (grid.len(), grid[0].len());
    let mut adj = 0;
    for i1 in [i - 1, i, i + 1] {
        for j1 in [j - 1, j, j + 1] {
            if i1 < 0 || i1 == m as i32 || j1 < 0 || j1 == n as i32 || (i1, j1) == (i, j) {
                continue;
            }
            adj += i32::from(grid[i1 as usize][j1 as usize] == b'@');
            if adj == 4 {
                return false;
            }
        }
    }
    true
}

fn accessible_places(grid: &[Vec<u8>]) -> Vec<i32> {
    let (m, n) = (grid.len(), grid[0].len());
    let mut ans = Vec::with_capacity(m * n);
    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell != b'@' {
                continue;
            }
            let (i, j) = (i as i32, j as i32);
            if is_accessible(grid, i, j) {
                ans.push(i * n as i32 + j);
            }
        }
    }
    ans
}

fn part2(mut grid: Vec<Vec<u8>>, mut to_remove: Vec<i32>) -> usize {
    let (m, n) = (grid.len(), grid[0].len());
    let mut ans = 0;
    let mut next = HashSet::with_capacity(m * n);
    while !to_remove.is_empty() {
        ans += to_remove.len();
        for &idx in &to_remove {
            let (i, j) = (idx / n as i32, idx % n as i32);
            grid[i as usize][j as usize] = b'.';
        }

        for idx in to_remove.drain(..) {
            let (i, j) = (idx / n as i32, idx % n as i32);
            for i1 in [i - 1, i, i + 1] {
                for j1 in [j - 1, j, j + 1] {
                    if i1 < 0
                        || i1 == m as i32
                        || j1 < 0
                        || j1 == n as i32
                        || (i1, j1) == (i, j)
                        || grid[i1 as usize][j1 as usize] != b'@'
                    {
                        continue;
                    }
                    if is_accessible(&grid, i1, j1) {
                        next.insert(i1 * n as i32 + j1);
                    }
                }
            }
        }

        to_remove.extend(next.drain());
    }
    ans
}

pub fn main() {
    let data = std::fs::read_to_string("data/2025/day4").unwrap();
    let grid = parse(&data);
    let accessible = accessible_places(&grid);
    println!("part1: {}", accessible.len());
    println!("part2: {}", part2(grid, accessible));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        let grid = parse(data);
        let accessible = accessible_places(&grid);
        assert_eq!(13, accessible.len());
        assert_eq!(43, part2(grid, accessible));
    }
}
