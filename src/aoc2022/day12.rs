use std::collections::VecDeque;

fn parse(data: &str) -> Vec<&[u8]> {
    data.trim().lines().map(|line| line.as_bytes()).collect()
}

fn find_pos(map: &[&[u8]], p: u8) -> (i16, i16) {
    for (i, row) in map.iter().enumerate() {
        for (j, &e) in row.iter().enumerate() {
            if e == p {
                return (i as i16, j as i16);
            }
        }
    }
    unreachable!("could not find position of {p}");
}

fn height(c: u8) -> u8 {
    match c {
        b'S' => b'a',
        b'E' => b'z',
        x => x,
    }
}

fn part1(map: &[&[u8]]) -> u32 {
    let start = find_pos(map, b'S');
    let m = map.len();
    let n = map[0].len();
    let mut q = VecDeque::new();
    let mut visited = vec![vec![false; n]; m];
    visited[start.0 as usize][start.1 as usize] = true;
    q.push_back((start.0, start.1, 0));
    while let Some((i, j, d)) = q.pop_front() {
        let h = height(map[i as usize][j as usize]);
        for (i1, j1) in [(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)] {
            if i1 < 0
                || j1 < 0
                || i1 == m as i16
                || j1 == n as i16
                || visited[i1 as usize][j1 as usize]
            {
                continue;
            }
            let c1 = map[i1 as usize][j1 as usize];
            let h1 = height(c1);
            if h + 1 >= h1 {
                if c1 == b'E' {
                    return d + 1;
                }
                visited[i1 as usize][j1 as usize] = true;
                q.push_back((i1, j1, d + 1));
            }
        }
    }
    unreachable!()
}

fn part2(map: &[&[u8]]) -> u32 {
    let start = find_pos(map, b'E');
    let m = map.len();
    let n = map[0].len();
    let mut q = VecDeque::new();
    let mut visited = vec![vec![false; n]; m];
    visited[start.0 as usize][start.1 as usize] = true;
    q.push_back((start.0, start.1, 0));
    let mut min = u32::MAX;
    while let Some((i, j, d)) = q.pop_front() {
        let h = height(map[i as usize][j as usize]);
        for (i1, j1) in [(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)] {
            if i1 < 0
                || j1 < 0
                || i1 == m as i16
                || j1 == n as i16
                || visited[i1 as usize][j1 as usize]
            {
                continue;
            }
            let c1 = map[i1 as usize][j1 as usize];
            let h1 = height(c1);
            if h1 + 1 >= h {
                if c1 == b'S' || c1 == b'a' {
                    min = min.min(d + 1);
                } else {
                    visited[i1 as usize][j1 as usize] = true;
                    q.push_back((i1, j1, d + 1));
                }
            }
        }
    }
    min
}

pub fn main() {
    let data = std::fs::read_to_string("data/2022/day12").unwrap();
    let map = parse(&data);
    println!("part1: {}", part1(&map));
    println!("part2: {}", part2(&map));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
        let map = parse(data);
        assert_eq!(31, part1(&map));
        assert_eq!(29, part2(&map));
    }
}
