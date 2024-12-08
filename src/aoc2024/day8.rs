use ahash::{HashMap, HashSet, HashSetExt};

type Freq = HashMap<u8, Vec<(i8, i8)>>;

fn parse(input: &str) -> (usize, usize, Freq) {
    let map: Vec<&[u8]> = input.trim().lines().map(|line| line.as_bytes()).collect();
    let mut freq: HashMap<u8, Vec<_>> = HashMap::default();
    for (i, r) in map.iter().enumerate() {
        for (j, &c) in r.iter().enumerate() {
            if c == b'.' {
                continue;
            }
            freq.entry(c).or_default().push((i as i8, j as i8));
        }
    }
    (map.len(), map[0].len(), freq)
}

fn part1(m: usize, n: usize, freq: &Freq) -> usize {
    let mut antinodes = HashSet::with_capacity(m * n);
    for positions in freq.values() {
        for (k, &(i1, j1)) in positions.iter().enumerate() {
            for &(i2, j2) in &positions[k + 1..] {
                let (di, dj) = (i2 - i1, j2 - j1);
                for (i, j) in [(i1 - di, j1 - dj), (i2 + di, j2 + dj)] {
                    if i >= 0 && j >= 0 && i < m as i8 && j < n as i8 {
                        antinodes.insert((i, j));
                    }
                }
            }
        }
    }
    antinodes.len()
}

fn gcd(a: i8, b: i8) -> i8 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn part2(m: usize, n: usize, freq: &Freq) -> usize {
    let mut antinodes = HashSet::with_capacity(m * n);
    for positions in freq.values() {
        for (k, &(i1, j1)) in positions.iter().enumerate() {
            for &(i2, j2) in &positions[k + 1..] {
                let (di, dj) = (i2 - i1, j2 - j1);
                let g = gcd(di, dj);
                let (di, dj) = (di / g, dj / g);
                antinodes.insert((i1, j1));
                for dir in [-1, 1] {
                    for k in 1.. {
                        let (i, j) = (i1 + dir * k * di, j1 + dir * k * dj);
                        if i >= 0 && j >= 0 && i < m as i8 && j < n as i8 {
                            antinodes.insert((i, j));
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    }
    antinodes.len()
}

pub fn main() {
    let input: String = std::fs::read_to_string("data/2024/day8").unwrap();
    let (m, n, freq) = parse(&input);
    println!("part1: {}", part1(m, n, &freq));
    println!("part2: {}", part2(m, n, &freq));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = "
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        let (m, n, freq) = parse(input);
        assert_eq!(14, part1(m, n, &freq));
        assert_eq!(34, part2(m, n, &freq));
    }

    #[test]
    fn case2() {
        let input = "
T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
..........";
        let (m, n, freq) = parse(input);
        assert_eq!(9, part2(m, n, &freq));
    }
}
