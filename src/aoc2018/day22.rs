use std::{cmp::Reverse, collections::BinaryHeap};

fn parse(data: &str) -> (u32, (u32, u32)) {
    let mut lines = data.trim().lines();
    let depth = lines
        .next()
        .unwrap()
        .trim_start_matches("depth: ")
        .parse()
        .unwrap();
    let (x, y) = lines
        .next()
        .unwrap()
        .trim_start_matches("target: ")
        .split_once(',')
        .unwrap();
    let x = x.parse().unwrap();
    let y = y.parse().unwrap();
    (depth, (x, y))
}

fn geologic_index((x, y): (u32, u32), row: &[u32], target: (u32, u32)) -> u32 {
    match (x, y) {
        (0, 0) => 0,
        (x, 0) => x * 16807,
        (0, y) => y * 48271,
        (x, y) if (x, y) == target => 0,
        (x, _y) => row[x as usize - 1] * row[x as usize],
    }
}

fn part1(depth: u32, target: (u32, u32)) -> u32 {
    let mut row = vec![0; target.0 as usize + 1];
    let mut sum = 0;
    for y in 0..=target.1 {
        for x in 0..=target.0 {
            let index = geologic_index((x, y), &row, target);
            row[x as usize] = (index + depth) % 20183;
            sum += row[x as usize] % 3;
        }
    }
    sum
}

fn gen_map(depth: u32, target: (u32, u32), scale: u32) -> (Vec<u8>, usize, usize) {
    let w = (target.0.max(target.1) + 1) * scale;
    let mut row = vec![0; w as usize];
    let mut map = Vec::with_capacity((w * w) as usize);
    for y in 0..w {
        for x in 0..w {
            let index = geologic_index((x, y), &row, target);
            row[x as usize] = (index + depth) % 20183;
            map.push((row[x as usize] % 3) as u8);
        }
    }
    (map, w as usize, w as usize)
}

#[inline]
fn manhattan((i1, j1): (u32, u32), (i2, j2): (u32, u32)) -> u32 {
    i1.abs_diff(i2) + j1.abs_diff(j2)
}

fn part2(depth: u32, target: (u32, u32)) -> u32 {
    let (map, w, h) = gen_map(depth, target, 2);
    let mut q = BinaryHeap::new();
    q.push((Reverse(0), (0, 0), 1, 0));
    let mut dist = vec![u32::MAX; w * h * 3];
    let imap = |x: u32, y: u32| y as usize * w + x as usize;
    let idist = |x: u32, y: u32, s: u8| y as usize * (3 * w) + x as usize * 3 + s as usize;
    while let Some((Reverse(f), (x, y), tool, t)) = q.pop() {
        if (x, y) == target {
            if tool == 1 {
                return t;
            } else {
                q.push((Reverse(f + 7), (x, y), 1, t + 7));
                continue;
            }
        }
        if t > dist[idist(x, y, tool)] {
            continue;
        }
        for (x1, y1) in [
            (x + 1, y),
            (x, y + 1),
            (x.saturating_sub(1), y),
            (x, y.saturating_sub(1)),
        ] {
            if (x1, y1) == (x, y) {
                continue;
            }
            let r = map[imap(x, y)];
            let r1 = map[imap(x1, y1)];
            let h = manhattan((x1, y1), target);
            if tool == r1 {
                let tool1 = 3 - r - r1;
                if t + 8 < dist[idist(x1, y1, tool1)] {
                    dist[idist(x1, y1, tool1)] = t + 8;
                    q.push((Reverse(t + 8 + h), (x1, y1), tool1, t + 8));
                }
            } else if t + 1 < dist[idist(x1, y1, tool)] {
                dist[idist(x1, y1, tool)] = t + 1;
                q.push((Reverse(t + 1 + h), (x1, y1), tool, t + 1));
            }
        }
    }
    unreachable!()
}

pub fn main() {
    let data = std::fs::read_to_string("data/2018/day22").unwrap();
    let (depth, target) = parse(&data);
    println!("part1: {}", part1(depth, target));
    println!("part2: {}", part2(depth, target));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let depth = 510;
        let target = (10, 10);
        assert_eq!(114, part1(depth, target));
        assert_eq!(45, part2(depth, target));
    }
}
