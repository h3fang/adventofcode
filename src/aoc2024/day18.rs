use std::{cmp::Reverse, collections::BinaryHeap};

fn parse(input: &str) -> Vec<(i16, i16)> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(',').unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect()
}

fn part1(bytes: &[(i16, i16)], n: i16) -> u16 {
    let mut map = vec![false; n as usize * n as usize];
    for &(x, y) in bytes {
        map[y as usize * n as usize + x as usize] = true;
    }
    let end = n * n - 1;
    let mut dist = vec![u16::MAX; n as usize * n as usize];
    dist[0] = 0;
    let mut q = BinaryHeap::with_capacity(n as usize * n as usize);
    q.push((Reverse(0), 0));
    while let Some((Reverse(d), k)) = q.pop() {
        if k == end {
            return d;
        }
        if dist[k as usize] < d {
            continue;
        }
        let (i, j) = (k / n, k % n);
        for (i1, j1) in [(i - 1, j), (i, j - 1), (i, j + 1), (i + 1, j)] {
            let k1 = i1 * n + j1;
            if i1 < 0 || j1 < 0 || i1 == n || j1 == n || map[k1 as usize] {
                continue;
            }
            if d + 1 < dist[k1 as usize] {
                dist[k1 as usize] = d + 1;
                q.push((Reverse(d + 1), k1));
            }
        }
    }
    u16::MAX
}

fn connected(bytes: &[(i16, i16)], n: i16) -> bool {
    let mut map = vec![false; n as usize * n as usize];
    for &(x, y) in bytes {
        map[y as usize * n as usize + x as usize] = true;
    }
    let end: i16 = n * n - 1;
    let mut visited = vec![false; n as usize * n as usize];
    visited[0] = true;
    let mut q = Vec::with_capacity(n as usize * n as usize);
    q.push(0);
    while let Some(k) = q.pop() {
        if k == end {
            return true;
        }
        let (i, j) = (k / n, k % n);
        for (i1, j1) in [(i - 1, j), (i, j - 1), (i, j + 1), (i + 1, j)] {
            let k1 = i1 * n + j1;
            if i1 < 0 || j1 < 0 || i1 == n || j1 == n || map[k1 as usize] {
                continue;
            }
            if !visited[k1 as usize] {
                visited[k1 as usize] = true;
                q.push(k1);
            }
        }
    }
    false
}

// Union-find is faster, but this is fast enough.
fn part2(bytes: &[(i16, i16)], n: i16) -> (i16, i16) {
    let (mut l, mut r) = (0, bytes.len() as i32 - 1);
    while l < r {
        let m = (r + l) / 2;
        if connected(&bytes[..=m as usize], n) {
            l = m + 1;
        } else {
            r = m - 1;
        }
    }
    bytes[l as usize]
}

pub fn main() {
    let input: String = std::fs::read_to_string("data/2024/day18").unwrap();
    let bytes = parse(&input);
    println!("part1: {}", part1(&bytes[..1024], 71));
    let (x, y) = part2(&bytes, 71);
    println!("part2: {x},{y}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = "
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
        let bytes = parse(input);
        assert_eq!(22, part1(&bytes[..12], 7));
        assert_eq!((6, 1), part2(&bytes, 7));
    }
}
