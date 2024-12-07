struct Scan {
    rocks: Vec<Vec<(i16, i16)>>,
    min: (i16, i16),
    max: (i16, i16),
}

fn parse(data: &str) -> Scan {
    let mut min = (i16::MAX, i16::MAX);
    let mut max = (i16::MIN, i16::MIN);
    let rocks = data
        .trim()
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|p| {
                    let (a, b) = p.split_once(',').unwrap();
                    let x = a.parse().unwrap();
                    let y = b.parse().unwrap();
                    min.0 = min.0.min(x);
                    min.1 = min.1.min(y);
                    max.0 = max.0.max(x);
                    max.1 = max.1.max(y);
                    (x, y)
                })
                .collect()
        })
        .collect();
    Scan { rocks, min, max }
}

fn fill_rocks<F: Fn(i16) -> usize>(
    map: &mut [Vec<u8>],
    rocks: &[Vec<(i16, i16)>],
    i: fn(i16) -> usize,
    j: F,
) -> usize {
    for r in rocks {
        let mut c = r[0];
        map[i(c.1)][j(c.0)] = b'#';
        for &p in &r[1..] {
            for x in c.0.min(p.0)..=c.0.max(p.0) {
                for y in c.1.min(p.1)..=c.1.max(p.1) {
                    map[i(y)][j(x)] = b'#';
                }
            }
            c = p;
        }
    }
    let source_j = j(500);
    map[0][source_j] = b'+';
    source_j
}

fn build_map1(s: &Scan) -> (Vec<Vec<u8>>, usize) {
    let height = (s.max.1 + 1) as usize;
    let width = (s.max.0 - s.min.0 + 1) as usize;
    let mut map = vec![vec![b'.'; width]; height];
    let j = |x: i16| (x - s.min.0) as usize;
    let i = |y: i16| y as usize;
    let source_j = fill_rocks(&mut map, &s.rocks, i, j);
    (map, source_j)
}

fn build_map2(s: &Scan) -> (Vec<Vec<u8>>, usize) {
    let height = (s.max.1 + 3) as usize;
    let width = (s.max.0 - s.min.0 + 1) as usize + 2 * height;
    let mut map = vec![vec![b'.'; width]; height];
    map[height - 1].iter_mut().for_each(|c| *c = b'#');
    let j = |x: i16| (x - s.min.0) as usize + height;
    let i = |y: i16| y as usize;
    let source_j = fill_rocks(&mut map, &s.rocks, i, j);
    (map, source_j)
}

// fn print_map(map: &[Vec<u8>]) {
//     for r in map {
//         println!("{}", unsafe { std::str::from_utf8_unchecked(r) });
//     }
// }

fn flow(mut map: Vec<Vec<u8>>, source_j: usize) -> usize {
    let m = map.len();
    let n = map[0].len();
    let mut result = 0;
    while map[0][source_j] == b'+' {
        let (mut i, mut j) = (0, source_j);
        loop {
            if i + 1 == m {
                return result;
            }
            if map[i + 1][j] == b'.' {
                i += 1;
                continue;
            }
            if j == 0 {
                return result;
            }
            if map[i + 1][j - 1] == b'.' {
                i += 1;
                j -= 1;
                continue;
            }
            if j + 1 == n {
                return result;
            }
            if map[i + 1][j + 1] == b'.' {
                i += 1;
                j += 1;
                continue;
            }
            map[i][j] = b'o';
            result += 1;
            // print_map(&map);
            break;
        }
    }
    result
}

fn part1(s: &Scan) -> usize {
    let (map, source_j) = build_map1(s);
    flow(map, source_j)
}

fn part2(s: &Scan) -> usize {
    let (map, source_j) = build_map2(s);
    flow(map, source_j)
}

pub fn main() {
    let data = std::fs::read_to_string("data/2022/day14").unwrap();
    let scan = parse(&data);
    println!("part1: {}", part1(&scan));
    println!("part2: {}", part2(&scan));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
        let scan = parse(data);
        assert_eq!(24, part1(&scan));
        assert_eq!(93, part2(&scan));
    }
}
