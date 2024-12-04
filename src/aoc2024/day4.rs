fn parse(input: &str) -> Vec<&[u8]> {
    input.trim().lines().map(|line| line.as_bytes()).collect()
}

fn part1(words: &[&[u8]]) -> i32 {
    let (m, n) = (words.len(), words[0].len());
    let mut result = 0;

    const TARGET: &[u8] = b"XMAS";
    for (i, r) in words.iter().enumerate() {
        for (j, &x) in r.iter().enumerate() {
            if x != b'X' {
                continue;
            }
            for (di, dj) in [
                (0, 1),
                (0, -1),
                (1, 0),
                (-1, 0),
                (1, 1),
                (-1, -1),
                (1, -1),
                (-1, 1),
            ] {
                if (1..4).all(|k| {
                    let (i1, j1) = (i as i32 + di * k, j as i32 + dj * k);
                    i1 >= 0
                        && j1 >= 0
                        && i1 < m as i32
                        && j1 < n as i32
                        && words[i1 as usize][j1 as usize] == TARGET[k as usize]
                }) {
                    result += 1;
                }
            }
        }
    }
    result
}

fn is_x_mas(i: usize, j: usize, words: &[&[u8]]) -> bool {
    ((words[i - 1][j - 1] == b'S' && words[i + 1][j + 1] == b'M')
        || (words[i - 1][j - 1] == b'M' && words[i + 1][j + 1] == b'S'))
        && ((words[i + 1][j - 1] == b'S' && words[i - 1][j + 1] == b'M')
            || (words[i + 1][j - 1] == b'M' && words[i - 1][j + 1] == b'S'))
}

fn part2(words: &[&[u8]]) -> i32 {
    let (m, n) = (words.len(), words[0].len());
    let mut result = 0;
    for (i, r) in words.iter().enumerate().skip(1).take(m - 2) {
        for (j, &x) in r.iter().enumerate().skip(1).take(n - 2) {
            if x != b'A' {
                continue;
            }
            if is_x_mas(i, j, words) {
                result += 1;
            }
        }
    }
    result
}

pub fn main() {
    let input: String = std::fs::read_to_string("data/2024/day4").unwrap();
    let words = parse(&input);
    println!("part1: {}", part1(&words));
    println!("part2: {}", part2(&words));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = "
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let words = parse(input);
        assert_eq!(18, part1(&words));
        assert_eq!(9, part2(&words));
    }
}
