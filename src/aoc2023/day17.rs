use std::{cmp::Reverse, collections::BinaryHeap};

fn parse(data: &str) -> Vec<&[u8]> {
    data.trim().lines().map(|l| l.as_bytes()).collect()
}

fn mininum_loss(grid: &[&[u8]], min_straight: usize, max_straight: usize) -> u32 {
    let (m, n) = (grid.len() as i16, grid[0].len() as i16);
    let mut losses = vec![vec![[u32::MAX; 4]; n as usize]; m as usize];
    let mut q = BinaryHeap::from([(Reverse(0), 0, 0, usize::MAX)]);
    while let Some((Reverse(loss), i, j, rev)) = q.pop() {
        if (i, j) == (m - 1, n - 1) {
            return loss;
        }
        if rev < 4 {
            if losses[i as usize][j as usize][rev] < u32::MAX {
                continue;
            }
            losses[i as usize][j as usize][rev] = loss;
        }
        for (dir1, (di, dj)) in [(0, 1), (1, 0), (0, -1), (-1, 0)].into_iter().enumerate() {
            if dir1 == rev || (dir1 + 2) % 4 == rev {
                continue;
            }
            let mut straight_loss = 0;
            for k in 1..=max_straight {
                let (i1, j1) = (i + di * k as i16, j + dj * k as i16);
                if i1 < 0 || j1 < 0 || i1 >= m || j1 >= n {
                    break;
                }
                straight_loss += (grid[i1 as usize][j1 as usize] - b'0') as u32;
                if k < min_straight {
                    continue;
                }
                let loss1 = loss + straight_loss;
                if loss1 < losses[i1 as usize][j1 as usize][dir1] {
                    q.push((Reverse(loss1), i1, j1, dir1));
                }
            }
        }
    }
    unreachable!()
}

pub fn main() {
    let data = std::fs::read_to_string("data/2023/day17").unwrap();
    let grid = parse(&data);
    println!("part1: {}", mininum_loss(&grid, 0, 3));
    println!("part2: {}", mininum_loss(&grid, 4, 10));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = r"
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
        let grid = parse(data);
        assert_eq!(102, mininum_loss(&grid, 0, 3));
        assert_eq!(94, mininum_loss(&grid, 4, 10));
    }
}
