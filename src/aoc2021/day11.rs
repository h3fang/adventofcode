fn step(grid: &mut [Vec<u8>]) -> usize {
    fn flashing(grid: &mut [Vec<u8>], x: usize, y: usize) {
        if grid[y][x] == 10 {
            grid[y][x] = 11;
            let rows = grid.len();
            let cols = grid[0].len();
            for (xn, yn) in [
                (x.wrapping_sub(1), y.wrapping_sub(1)),
                (x, y.wrapping_sub(1)),
                (x + 1, y.wrapping_sub(1)),
                (x.wrapping_sub(1), y),
                (x + 1, y),
                (x.wrapping_sub(1), y + 1),
                (x, y + 1),
                (x + 1, y + 1),
            ] {
                if xn >= cols || yn >= rows {
                    continue;
                }
                if grid[yn][xn] < 10 {
                    grid[yn][xn] += 1;
                }
                flashing(grid, xn, yn);
            }
        }
    }

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] < 10 {
                grid[y][x] += 1;
                flashing(grid, x, y);
            }
        }
    }
    let mut result = 0;
    grid.iter_mut().flatten().for_each(|c| {
        if *c > 9 {
            *c = 0;
            result += 1;
        }
    });
    result
}

fn part1(grid: &[Vec<u8>]) -> usize {
    let mut grid = grid.to_owned();
    (0..100).map(|_| step(&mut grid)).sum()
}

fn part2(grid: &[Vec<u8>]) -> usize {
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
                .map(|e| e - b'0')
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!("day11 part1: {}", part1(&grid));
    println!("day11 part2: {}", part2(&grid));
}
