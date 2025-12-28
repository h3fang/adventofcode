use ahash::{HashSet, HashSetExt};

fn parse(data: &str) -> Vec<&[u8]> {
    data.trim().lines().map(|s| s.as_bytes()).collect()
}

fn part1(grid: &[&[u8]]) -> usize {
    let mut ans = 0;
    let (m, n) = (grid.len(), grid[0].len());
    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell != b'@' {
                continue;
            }
            let (i, j) = (i as i32, j as i32);
            let mut adjacent_papers = 0;
            for i1 in [i - 1, i, i + 1] {
                for j1 in [j - 1, j, j + 1] {
                    if i1 < 0 || i1 == m as i32 || j1 < 0 || j1 == n as i32 || (i1, j1) == (i, j) {
                        continue;
                    }
                    adjacent_papers += i32::from(grid[i1 as usize][j1 as usize] == b'@');
                }
            }
            ans += usize::from(adjacent_papers < 4);
        }
    }
    ans
}

fn part2(grid: &[&[u8]]) -> usize {
    let (m, n) = (grid.len(), grid[0].len());
    let mut grid = grid.iter().map(|r| r.to_vec()).collect::<Vec<_>>();
    let mut ans = 0;
    let mut papers = HashSet::with_capacity(m * n);
    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == b'@' {
                papers.insert((i * n + j) as u16);
            }
        }
    }
    let mut removed = Vec::with_capacity(papers.len());
    loop {
        for &idx in &papers {
            let (i, j) = ((idx as usize / n) as i16, ((idx as usize) % n) as i16);
            let mut adjacent_papers = 0;
            for i1 in [i - 1, i, i + 1] {
                for j1 in [j - 1, j, j + 1] {
                    if i1 < 0 || i1 == m as i16 || j1 < 0 || j1 == n as i16 || (i1, j1) == (i, j) {
                        continue;
                    }
                    adjacent_papers += i32::from(grid[i1 as usize][j1 as usize] == b'@');
                }
            }
            if adjacent_papers < 4 {
                removed.push((i, j));
            }
        }
        if removed.is_empty() {
            break;
        }
        ans += removed.len();
        for (i, j) in removed.drain(..) {
            papers.remove(&((i * n as i16 + j) as u16));
            grid[i as usize][j as usize] = b'.';
        }
    }
    ans
}

pub fn main() {
    let data = std::fs::read_to_string("data/2025/day4").unwrap();
    let grid = parse(&data);
    println!("part1: {}", part1(&grid));
    println!("part2: {}", part2(&grid));
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
        assert_eq!(13, part1(&grid));
        assert_eq!(43, part2(&grid));
    }
}
