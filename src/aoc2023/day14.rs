use ahash::HashMap;

fn parse(data: &str) -> Vec<Vec<u8>> {
    data.trim()
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect()
}

fn load_on_north_support_beams(rotated_grid: &[Vec<u8>]) -> usize {
    let mut result = 0;
    for r in rotated_grid {
        for (j, &c) in r.iter().enumerate() {
            if c == b'O' {
                result += j + 1;
            }
        }
    }
    result
}

fn tilt(grid: &mut [Vec<u8>]) {
    let n = grid[0].len();
    for r in grid {
        let mut empty = n - 1;
        for j in (0..n).rev() {
            match r[j] {
                b'#' => empty = j.saturating_sub(1),
                b'O' => {
                    r.swap(j, empty);
                    empty = empty.saturating_sub(1);
                }
                _ => {}
            }
        }
    }
}

fn part1(mut grid: Vec<Vec<u8>>) -> usize {
    tilt(&mut grid);
    load_on_north_support_beams(&grid)
}

fn transpose(grid: &mut [Vec<u8>]) {
    let (m, n) = (grid.len(), grid[0].len());
    assert!(m == n);
    #[allow(clippy::needless_range_loop)]
    for i in 0..m {
        for j in i + 1..m {
            (grid[i][j], grid[j][i]) = (grid[j][i], grid[i][j]);
        }
    }
}

fn flip_upside_down(grid: &mut [Vec<u8>]) {
    grid.reverse();
}

fn rotate_90_clockwise(grid: &mut [Vec<u8>]) {
    flip_upside_down(grid);
    transpose(grid);
}

fn part2(mut grid: Vec<Vec<u8>>) -> usize {
    let mut seen = HashMap::default();
    for i in 1.. {
        for _ in 0..4 {
            tilt(&mut grid);
            rotate_90_clockwise(&mut grid);
        }
        let load = load_on_north_support_beams(&grid);

        if let Some(&(j, _)) = seen.get(&grid) {
            let period = i - j;
            let k = j + (10_0000_0000 - i) % period;
            return seen.values().find(|e| e.0 == k).map(|e| e.1).unwrap();
        }
        seen.insert(grid.clone(), (i, load));
    }
    unreachable!()
}

pub fn main() {
    let data = std::fs::read_to_string("data/2023/day14").unwrap();
    let mut grid = parse(&data);
    rotate_90_clockwise(&mut grid);
    println!("part1: {}", part1(grid.clone()));
    println!("part2: {}", part2(grid));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
";
        let mut grid = parse(data);
        rotate_90_clockwise(&mut grid);
        assert_eq!(136, part1(grid.clone()));
        assert_eq!(64, part2(grid));
    }
}
