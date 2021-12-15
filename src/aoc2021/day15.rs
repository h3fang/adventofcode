use std::{cmp::Reverse, collections::BinaryHeap};

fn parse(data: &str) -> Vec<Vec<u8>> {
    data.lines()
        .map(|line| line.trim().as_bytes().iter().map(|b| b - b'0').collect())
        .collect()
}

fn dijkstra(risk_map: &[Vec<u8>]) -> i32 {
    let m = risk_map.len() as i32;
    let n = risk_map[0].len() as i32;
    let target = (m - 1, n - 1);
    let mut costs = vec![vec![i32::MAX; n as usize]; m as usize];
    let mut q: BinaryHeap<(Reverse<i32>, (i32, i32))> = BinaryHeap::new();
    q.push((Reverse(0), (0, 0)));
    while let Some((c, pos)) = q.pop() {
        if pos == target {
            return c.0;
        }
        if c.0 > costs[pos.1 as usize][pos.0 as usize] {
            continue;
        }
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let (xn, yn) = (pos.0 + dx, pos.1 + dy);
            if xn < 0 || yn < 0 || xn >= n || yn >= m {
                continue;
            }
            let cn = c.0 + risk_map[yn as usize][xn as usize] as i32;
            if cn < costs[yn as usize][xn as usize] {
                costs[yn as usize][xn as usize] = cn;
                q.push((Reverse(cn), (xn, yn)));
            }
        }
    }
    unreachable!()
}

#[allow(dead_code)]
fn astar(risk_map: &[Vec<u8>]) -> i32 {
    let m = risk_map.len() as i32;
    let n = risk_map[0].len() as i32;
    let target = (m - 1, n - 1);

    let heuristic = |(x, y): (i32, i32)| (target.0 - x) + (target.1 - y);

    let mut gs = vec![vec![i32::MAX; n as usize]; m as usize];
    let mut fs = vec![vec![i32::MAX; n as usize]; m as usize];
    let mut q: BinaryHeap<(Reverse<i32>, (i32, i32))> = BinaryHeap::new();

    gs[0][0] = 0;
    fs[0][0] = 0;
    q.push((Reverse(0), (0, 0)));

    while let Some((f, pos)) = q.pop() {
        if pos == target {
            return f.0;
        }
        if f.0 > fs[pos.1 as usize][pos.0 as usize] {
            continue;
        }
        let g = gs[pos.1 as usize][pos.0 as usize];
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let (xn, yn) = (pos.0 + dx, pos.1 + dy);
            if xn < 0 || yn < 0 || xn >= n || yn >= m {
                continue;
            }
            let g_new = g + risk_map[yn as usize][xn as usize] as i32;
            if g_new < gs[yn as usize][xn as usize] {
                gs[yn as usize][xn as usize] = g_new;
                let f_new = g_new + heuristic((xn, yn));
                fs[yn as usize][xn as usize] = f_new;
                q.push((Reverse(f_new), (xn, yn)));
            }
        }
    }
    unreachable!()
}

fn large_map(risk_map: &[Vec<u8>]) -> Vec<Vec<u8>> {
    const R: usize = 5;
    let m = risk_map.len();
    let n = risk_map[0].len();
    let mut large_map = vec![vec![0u8; n * R]; m * R];
    for r in 0..R {
        for c in 0..R {
            for y in 0..risk_map.len() {
                for x in 0..risk_map[0].len() {
                    let risk = (risk_map[y][x] + r as u8 + c as u8) % 9;
                    large_map[r * m + y][c * n + x] = if risk == 0 { 9 } else { risk };
                }
            }
        }
    }
    large_map
}

pub fn main() {
    let data = std::fs::read_to_string("data/2021/day15").unwrap();
    let risk_map = parse(&data);
    println!("day15 part1: {}", dijkstra(&risk_map));
    println!("day15 part2: {}", dijkstra(&large_map(&risk_map)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_map() {
        let small = vec![vec![8]];
        let large = large_map(&small);
        let expected = vec![
            vec![8, 9, 1, 2, 3],
            vec![9, 1, 2, 3, 4],
            vec![1, 2, 3, 4, 5],
            vec![2, 3, 4, 5, 6],
            vec![3, 4, 5, 6, 7],
        ];
        assert_eq!(expected, large);
    }
}
