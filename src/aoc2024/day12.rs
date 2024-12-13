use ahash::HashSet;

fn parse(input: &str) -> Vec<&[u8]> {
    input.trim().lines().map(|line| line.as_bytes()).collect()
}

const DIRS: [(i16, i16); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn is_corner(i: i16, j: i16, d: usize, region: &HashSet<(i16, i16)>) -> bool {
    let forward = (i + DIRS[(d) % 4].0, j + DIRS[(d) % 4].1);
    let right = (i + DIRS[(d + 1) % 4].0, j + DIRS[(d + 1) % 4].1);
    let forward_right = (
        i + DIRS[d].0 + DIRS[(d + 1) % 4].0,
        j + DIRS[d].1 + DIRS[(d + 1) % 4].1,
    );
    if !region.contains(&forward) && !region.contains(&right) {
        return true;
    }

    region.contains(&forward) && region.contains(&right) && !region.contains(&forward_right)
}

fn solve(map: &[&[u8]]) -> (usize, usize) {
    let (m, n) = (map.len(), map[0].len());

    let valid = |i: i16, j: i16| i >= 0 && j >= 0 && i < m as i16 && j < n as i16;
    let mut visited = vec![false; m * n];
    let (mut p1, mut p2) = (0, 0);
    for (i, r) in map.iter().enumerate() {
        for (j, &c) in r.iter().enumerate() {
            if visited[i * n + j] {
                continue;
            }
            let mut q = Vec::with_capacity(m * n);
            q.push((i as i16, j as i16));
            let mut region = HashSet::default();
            region.insert((i as i16, j as i16));
            let mut perimeter = 4;
            while let Some((i, j)) = q.pop() {
                visited[i as usize * n + j as usize] = true;
                for (i1, j1) in [(i + 1, j), (i - 1, j), (i, j + 1), (i, j - 1)] {
                    if !valid(i1, j1)
                        || map[i1 as usize][j1 as usize] != c
                        || region.contains(&(i1, j1))
                    {
                        continue;
                    }
                    region.insert((i1, j1));
                    perimeter += 4;
                    for (i2, j2) in [(i1 + 1, j1), (i1 - 1, j1), (i1, j1 + 1), (i1, j1 - 1)] {
                        if valid(i2, j2) && region.contains(&(i2, j2)) {
                            perimeter -= 2;
                        }
                    }
                    q.push((i1, j1));
                }
            }
            let corners: usize = region
                .iter()
                .map(|&(i, j)| (0..4).filter(|&d| is_corner(i, j, d, &region)).count())
                .sum();
            p1 += region.len() * perimeter;
            p2 += region.len() * corners;
        }
    }
    (p1, p2)
}

pub fn main() {
    let input: String = std::fs::read_to_string("data/2024/day12").unwrap();
    let map = parse(&input);
    let (p1, p2) = solve(&map);
    println!("part1: {p1}");
    println!("part2: {p2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = "
AAAA
BBCD
BBCC
EEEC";
        let map = parse(input);
        let (p1, p2) = solve(&map);
        assert_eq!(140, p1);
        assert_eq!(80, p2);
    }

    #[test]
    fn case2() {
        let input = "
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
        let map = parse(input);
        let (p1, p2) = solve(&map);
        assert_eq!(772, p1);
        assert_eq!(436, p2);
    }

    #[test]
    fn case3() {
        let input = "
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        let map = parse(input);
        let (p1, p2) = solve(&map);
        assert_eq!(1930, p1);
        assert_eq!(1206, p2);
    }

    #[test]
    fn case4() {
        let input = "
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";
        let map = parse(input);
        let (_, p2) = solve(&map);
        assert_eq!(236, p2);
    }

    #[test]
    fn case5() {
        let input = "
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";
        let map = parse(input);
        let (_, p2) = solve(&map);
        assert_eq!(368, p2);
    }
}
