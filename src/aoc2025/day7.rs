fn parse(data: &str) -> Vec<&[u8]> {
    data.trim().lines().map(|l| l.as_bytes()).collect()
}

fn part1(lines: &[&[u8]]) -> usize {
    let n = lines[0].len();
    let s = lines[0].iter().position(|&b| b == b'S').unwrap();
    let mut beams = vec![false; n];
    beams[s] = true;
    let mut ans = 0;

    for line in lines[1..].iter() {
        let mut next = vec![false; n];
        for (i, &b) in beams.iter().enumerate() {
            if !b {
                continue;
            }
            if line[i] == b'^' {
                ans += 1;
                if i > 0 {
                    next[i - 1] = true;
                }
                if i + 1 < n {
                    next[i + 1] = true;
                }
            } else {
                next[i] = true;
            }
        }
        beams = next;
    }

    ans
}

fn part2(lines: &[&[u8]]) -> usize {
    let n = lines[0].len();
    let s = lines[0].iter().position(|&b| b == b'S').unwrap();
    let mut beams = vec![0usize; n];
    beams[s] = 1;

    for line in lines[1..].iter() {
        let mut next = vec![0; n];
        for (i, &b) in beams.iter().enumerate() {
            if b == 0 {
                continue;
            }
            if line[i] == b'^' {
                if i > 0 {
                    next[i - 1] += b;
                }
                if i + 1 < n {
                    next[i + 1] += b;
                }
            } else {
                next[i] += b;
            }
        }
        beams = next;
    }

    beams.into_iter().sum()
}

pub fn main() {
    let data = std::fs::read_to_string("data/2025/day7").unwrap();
    let grid = parse(&data);
    println!("part1: {}", part1(&grid));
    println!("part2: {}", part2(&grid));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        let grid = parse(data);
        assert_eq!(21, part1(&grid));
        assert_eq!(40, part2(&grid));
    }
}
