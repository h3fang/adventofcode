fn parse(data: &str) -> Vec<Vec<Vec<u8>>> {
    data.trim()
        .lines()
        .collect::<Vec<_>>()
        .split(|l| l.is_empty())
        .map(|lines| lines.iter().map(|l| l.as_bytes().to_vec()).collect())
        .collect()
}

fn find_horizontal_line_reflection(pattern: &[Vec<u8>], exclude: usize) -> Option<usize> {
    let m = pattern.len();
    for i in 0..(m - 1) {
        if i == exclude {
            continue;
        }
        let height = i.min(m - i - 2);
        if (0..=height).all(|k| pattern[i - k] == pattern[i + 1 + k]) {
            return Some(i);
        }
    }
    None
}

fn find_vertical_line_reflection(pattern: &[Vec<u8>], exclude: usize) -> Option<usize> {
    let n = pattern[0].len();
    for j in 0..(n - 1) {
        if j == exclude {
            continue;
        }
        let width = j.min(n - j - 2);
        if pattern
            .iter()
            .all(|p| (0..=width).all(|k| p[j - k] == p[j + 1 + k]))
        {
            return Some(j);
        }
    }
    None
}

fn part1(patterns: &[Vec<Vec<u8>>]) -> Vec<usize> {
    patterns
        .iter()
        .map(|pattern| {
            if let Some(i) = find_horizontal_line_reflection(pattern, usize::MAX) {
                100 * (i + 1)
            } else if let Some(j) = find_vertical_line_reflection(pattern, usize::MAX) {
                j + 1
            } else {
                unreachable!()
            }
        })
        .collect()
}

fn find_smudge((reflection_line, mut pattern): (usize, Vec<Vec<u8>>)) -> usize {
    for i in 0..pattern.len() {
        for j in 0..pattern[0].len() {
            let cell = pattern[i][j];
            pattern[i][j] = if cell == b'.' { b'#' } else { b'.' };
            let exclude = if reflection_line >= 100 {
                reflection_line / 100 - 1
            } else {
                usize::MAX
            };
            if let Some(i) = find_horizontal_line_reflection(&pattern, exclude) {
                let r = 100 * (i + 1);
                if r != reflection_line {
                    return r;
                }
            }
            let exclude = if reflection_line < 100 {
                reflection_line - 1
            } else {
                usize::MAX
            };
            if let Some(j) = find_vertical_line_reflection(&pattern, exclude) {
                let r = j + 1;
                if r != reflection_line {
                    return r;
                }
            }
            pattern[i][j] = cell;
        }
    }
    unreachable!()
}

fn part2(reflection_lines: Vec<usize>, patterns: Vec<Vec<Vec<u8>>>) -> usize {
    reflection_lines
        .into_iter()
        .zip(patterns)
        .map(find_smudge)
        .sum()
}

pub fn main() {
    let data = std::fs::read_to_string("data/2023/day13").unwrap();
    let patterns = parse(&data);
    let reflection_lines = part1(&patterns);
    println!("part1: {}", reflection_lines.iter().sum::<usize>());
    println!("part2: {}", part2(reflection_lines, patterns));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";
        let patterns = parse(data);
        let reflection_lines = part1(&patterns);
        assert_eq!(405, reflection_lines.iter().sum::<usize>());
        assert_eq!(400, part2(reflection_lines, patterns));
    }
}
