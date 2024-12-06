use ahash::{HashSet, HashSetExt};

const DIRS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn parse(input: &str) -> Vec<&[u8]> {
    input.trim().lines().map(|line| line.as_bytes()).collect()
}

fn find_guard(map: &[&[u8]]) -> (i32, i32) {
    for (i, r) in map.iter().enumerate() {
        for (j, &c) in r.iter().enumerate() {
            if c == b'^' {
                return (i as i32, j as i32);
            }
        }
    }
    unreachable!()
}

fn part1(map: &[&[u8]], (mut i, mut j): (i32, i32)) -> HashSet<(i32, i32)> {
    let mut d = 0;
    let (m, n) = (map.len() as i32, map[0].len() as i32);
    let mut visited = HashSet::with_capacity((m * n) as usize);
    loop {
        visited.insert((i, j));
        let dir = &DIRS[d as usize];
        let (i1, j1) = (i + dir.0, j + dir.1);
        if i1 < 0 || j1 < 0 || i1 == m || j1 == n {
            return visited;
        }
        if map[i1 as usize][j1 as usize] == b'#' {
            d = (d + 1) % 4;
            let dir = &DIRS[d as usize];
            i += dir.0;
            j += dir.1;
        } else {
            (i, j) = (i1, j1);
        }
    }
}

fn is_loop(map: &[&[u8]], (mut i, mut j): (i32, i32), obstacle: (i32, i32)) -> bool {
    let mut d = 0;
    let (m, n) = (map.len() as i32, map[0].len() as i32);
    let mut visited = vec![false; (m * n * 4) as usize];
    loop {
        let idx = ((i * n + j) * 4 + d) as usize;
        if visited[idx] {
            return true;
        }
        visited[idx] = true;
        loop {
            let (di, dj) = DIRS[d as usize];
            let (i1, j1) = (i + di, j + dj);
            if i1 < 0 || j1 < 0 || i1 == m || j1 == n {
                return false;
            }
            if map[i1 as usize][j1 as usize] == b'#' || (i1, j1) == obstacle {
                d = (d + 1) % 4;
            } else {
                (i, j) = (i1, j1);
                break;
            }
        }
    }
}

fn part2(map: &[&[u8]], start: (i32, i32), path: HashSet<(i32, i32)>) -> usize {
    let mut result = 0;
    for (i, j) in path {
        if (i, j) != start && is_loop(map, start, (i, j)) {
            result += 1;
        }
    }
    result
}

pub fn main() {
    let input: String = std::fs::read_to_string("data/2024/day6").unwrap();
    let map = parse(&input);
    let start = find_guard(&map);
    let path = part1(&map, start);
    println!("part1: {}", path.len());
    println!("part2: {}", part2(&map, start, path));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = "
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        let map = parse(&input);
        let start = find_guard(&map);
        let path = part1(&map, start);
        assert_eq!(41, path.len());
        assert_eq!(6, part2(&map, start, path));
    }
}
