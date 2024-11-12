use std::collections::VecDeque;

fn parse(data: &str) -> Vec<&[u8]> {
    data.trim().lines().map(|l| l.as_bytes()).collect()
}

fn find_start(garden: &[&[u8]]) -> (usize, usize) {
    garden
        .iter()
        .enumerate()
        .find_map(|(i, row)| {
            row.iter()
                .enumerate()
                .find_map(|(j, e)| if *e == b'S' { Some(j) } else { None })
                .map(|j| (i, j))
        })
        .unwrap()
}

fn bfs(garden: &[&[u8]], (i0, j0): (usize, usize), steps: usize) -> Vec<usize> {
    let (m, n) = (garden.len(), garden[0].len());
    let mut q = VecDeque::default();
    let mut dist = vec![usize::MAX; m * n];
    dist[i0 * n + j0] = 0;
    q.push_back((i0 as i32, j0 as i32, 0));
    while let Some((i, j, d)) = q.pop_front() {
        for (i1, j1) in [(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)] {
            if i1 < 0
                || j1 < 0
                || i1 >= m as i32
                || j1 >= n as i32
                || garden[i1 as usize][j1 as usize] == b'#'
            {
                continue;
            }
            if d < steps && dist[i1 as usize * n + j1 as usize] == usize::MAX {
                dist[i1 as usize * n + j1 as usize] = d + 1;
                q.push_back((i1, j1, d + 1));
            }
        }
    }
    dist
}

fn part1(garden: &[&[u8]], steps: usize) -> usize {
    let (i0, j0) = find_start(garden);
    let dist = bfs(garden, (i0, j0), steps);
    dist.into_iter()
        .filter(|&x| x != usize::MAX && x % 2 == steps % 2)
        .count()
}

// not a generalized solution
// general idea comes from https://github.com/villuna/aoc23/wiki/A-Geometric-solution-to-advent-of-code-2023,-day-21
// but the calculation for even_corners are wrong (for most inputs, like the author's, it happens to be the right answer)
// even/odd corners have to be calculated from the side of the center, not the other side
// correct methods for even_corners can be found from https://www.youtube.com/watch?v=9UOMZSL0JTg
fn part2(garden: &[&[u8]], steps: usize) -> usize {
    let m = garden.len();
    let (mut odd, mut even, mut odd_corners, mut even_corners) = (0, 0, 0, 0);
    let (i0, j0) = find_start(garden);
    let dist = bfs(garden, (i0, j0), m);
    for x in dist {
        if x == usize::MAX {
            continue;
        }
        if x % 2 == 0 {
            even += 1;
        } else {
            odd += 1;
            if x > 65 {
                odd_corners += 1;
            }
        }
    }

    for start in [(m - 1, 0), (m - 1, m - 1), (0, 0), (0, m - 1)] {
        let dist = bfs(garden, start, m / 2 - 1);
        even_corners += dist
            .into_iter()
            .filter(|&x| x != usize::MAX && x % 2 == 0)
            .count();
    }

    let n = steps / m;
    (n + 1) * (n + 1) * odd + n * n * even - (n + 1) * odd_corners + n * even_corners
}

pub fn main() {
    let data = std::fs::read_to_string("data/2023/day21").unwrap();
    let garden = parse(&data);
    println!("part1: {}", part1(&garden, 64));
    println!("part2: {}", part2(&garden, 26501365));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = r"
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
        let garden = parse(data);
        assert_eq!(16, part1(&garden, 6));
        // assert_eq!(16, part2(&plan, 6));
        // assert_eq!(50, part2(&plan, 10));
        // assert_eq!(1594, part2(&plan, 50));
        // assert_eq!(6536, part2(&plan, 100));
        // assert_eq!(167004, part2(&plan, 500));
        // assert_eq!(668697, part2(&plan, 1000));
        // assert_eq!(16733044, part2(&plan, 5000));
    }

    #[test]
    fn case2() {
        let data = std::fs::read_to_string("data/2023/day21").unwrap();
        let garden = parse(&data);
        assert_eq!(3605, part1(&garden, 64));
        assert_eq!(596734624269210, part2(&garden, 26501365));
    }
}
