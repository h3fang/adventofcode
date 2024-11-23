use ahash::{HashMap, HashSet};
use arrayvec::ArrayVec;

fn parse(data: &str) -> Vec<&[u8]> {
    data.trim().lines().map(|l| l.as_bytes()).collect()
}

fn dfs(g: &HashMap<u32, ArrayVec<(u32, i32), 4>>, end: u32, curr: u32, visited: u64) -> isize {
    if curr == end {
        return 0;
    }
    let visited = visited | (1 << curr);
    let mut result = isize::MIN;
    for &(i, d) in &g[&curr] {
        if visited & (1 << i) > 0 {
            continue;
        }
        let r = dfs(g, end, i, visited);
        if r != isize::MIN {
            result = result.max(r + d as isize);
        }
    }
    result
}

fn solve(map: &[&[u8]], part2: bool) -> isize {
    let (m, n) = (map.len() as i32, map[0].len() as i32);
    let j_start = map[0].iter().position(|e| *e == b'.').unwrap() as i32;
    let j_end = map.last().unwrap().iter().position(|e| *e == b'.').unwrap() as i32;

    let mut points: HashMap<(i32, i32), u32> = [((0, j_start), 0), ((m - 1, j_end), 1)]
        .into_iter()
        .collect();
    for i in 0..m {
        for j in 0..n {
            if map[i as usize][j as usize] == b'#' {
                continue;
            }
            let mut neighbors = 0;
            for (i1, j1) in [(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)] {
                if i1 >= 0 && j1 >= 0 && i1 < m && j1 < n && map[i1 as usize][j1 as usize] != b'#' {
                    neighbors += 1;
                }
            }
            if neighbors > 2 {
                points.insert((i, j), points.len() as u32);
            }
        }
    }

    assert!(points.len() <= 64);

    let mut g: HashMap<u32, ArrayVec<(u32, i32), 4>> = HashMap::default();

    for (&(i0, j0), &k) in &points {
        let mut visited = HashSet::default();
        visited.insert((i0, j0));
        let mut q = vec![(i0, j0, 0)];
        while let Some((i, j, d)) = q.pop() {
            if d != 0 {
                if let Some(&k1) = points.get(&(i, j)) {
                    g.entry(k).or_default().push((k1, d));
                    continue;
                }
            }

            let mut next: ArrayVec<_, 4> = ArrayVec::new();
            if part2 {
                next.extend([(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)]);
            } else {
                match map[i as usize][j as usize] {
                    b'.' => next.extend([(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)]),
                    b'^' => next.push((i - 1, j)),
                    b'>' => next.push((i, j + 1)),
                    b'v' => next.push((i + 1, j)),
                    b'<' => next.push((i, j - 1)),
                    _ => unreachable!(),
                }
            }
            for (i1, j1) in next {
                if i1 >= 0
                    && j1 >= 0
                    && i1 < m
                    && j1 < n
                    && map[i1 as usize][j1 as usize] != b'#'
                    && !visited.contains(&(i1, j1))
                {
                    q.push((i1, j1, d + 1));
                    visited.insert((i1, j1));
                }
            }
        }
    }
    dfs(&g, 1, 0, 0)
}

pub fn main() {
    let data = std::fs::read_to_string("data/2023/day23").unwrap();
    let map = parse(&data);
    println!("part1: {}", solve(&map, false));
    println!("part2: {}", solve(&map, true));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = r"
#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";
        let map = parse(data);
        assert_eq!(94, solve(&map, false));
        assert_eq!(154, solve(&map, true));
    }
}
