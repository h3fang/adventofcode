use std::collections::VecDeque;

fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .trim()
        .lines()
        .map(|line| line.as_bytes().iter().map(|b| b - b'0').collect())
        .collect()
}

fn part1(map: &[Vec<u8>]) -> usize {
    let (m, n) = (map.len(), map[0].len());
    let mut result = 0;
    for (i, r) in map.iter().enumerate() {
        for (j, &c) in r.iter().enumerate() {
            if c != 0 {
                continue;
            }
            let mut tails = vec![false; m * n];
            let mut q = Vec::with_capacity(m * n);
            q.push((i as i8, j as i8, 0));
            while let Some((i, j, h)) = q.pop() {
                for (i1, j1) in [(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)] {
                    if i1 < 0
                        || j1 < 0
                        || i1 == m as i8
                        || j1 == n as i8
                        || map[i1 as usize][j1 as usize] != h + 1
                    {
                        continue;
                    }
                    if h + 1 == 9 {
                        tails[i1 as usize * n + j1 as usize] = true;
                    } else {
                        q.push((i1, j1, h + 1));
                    }
                }
            }
            result += tails.iter().filter(|&&x| x).count();
        }
    }
    result
}

fn part2(map: &[Vec<u8>]) -> usize {
    let (m, n) = (map.len(), map[0].len());
    let mut result = 0;
    let mut q = VecDeque::with_capacity(m * n);
    let mut visited = vec![false; m * n];
    let mut ways = vec![0; m * n];
    for (i, r) in map.iter().enumerate() {
        for (j, &c) in r.iter().enumerate() {
            if c != 0 {
                continue;
            }
            q.push_back((i as i8, j as i8, 0));
            visited.iter_mut().for_each(|e| *e = false);
            ways.iter_mut().for_each(|e| *e = 0);
            ways[i * n + j] = 1;
            while let Some((i, j, h)) = q.pop_front() {
                for (i1, j1) in [(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)] {
                    if i1 < 0
                        || j1 < 0
                        || i1 == m as i8
                        || j1 == n as i8
                        || map[i1 as usize][j1 as usize] != h + 1
                    {
                        continue;
                    }
                    if h + 1 == 9 {
                        result += ways[i as usize * n + j as usize];
                    } else {
                        let idx1 = i1 as usize * n + j1 as usize;
                        ways[idx1] += ways[i as usize * n + j as usize];
                        if !visited[idx1] {
                            q.push_back((i1, j1, h + 1));
                            visited[idx1] = true;
                        }
                    }
                }
            }
        }
    }
    result
}

pub fn main() {
    let input: String = std::fs::read_to_string("data/2024/day10").unwrap();
    let map = parse(&input);
    println!("part1: {}", part1(&map));
    println!("part2: {}", part2(&map));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = "
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        let map = parse(input);
        assert_eq!(36, part1(&map));
    }

    #[test]
    fn case2() {
        let input = "
012345
123456
234567
345678
4X6789
56789X";
        let map = parse(input);
        assert_eq!(227, part2(&map));
    }

    #[test]
    fn case3() {
        let input = "
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        let map = parse(input);
        assert_eq!(81, part2(&map));
    }
}
