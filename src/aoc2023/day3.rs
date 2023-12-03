use ahash::HashMap;

fn parse(data: &str) -> Vec<Vec<u8>> {
    data.trim()
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect()
}

fn part1(input: &[Vec<u8>]) -> u32 {
    input
        .iter()
        .enumerate()
        .map(|(i, row)| {
            let (mut j, mut sum) = (0, 0);
            while j < row.len() {
                let (mut j1, mut num) = (j, 0);
                while j1 < row.len() && row[j1].is_ascii_digit() {
                    num = num * 10 + (row[j1] - b'0') as u32;
                    j1 += 1;
                }
                if j1 > j {
                    'outter: for row in &input[i.saturating_sub(1)..(i + 2).min(input.len())] {
                        for &x in &row[j.saturating_sub(1)..=j1.min(row.len() - 1)] {
                            if x.is_ascii_punctuation() && x != b'.' {
                                sum += num;
                                break 'outter;
                            }
                        }
                    }
                    j = j1;
                } else {
                    j += 1;
                }
            }
            sum
        })
        .sum()
}

fn part2(input: &[Vec<u8>]) -> u32 {
    let mut map: HashMap<(u16, u16), Vec<u32>> = HashMap::default();
    input.iter().enumerate().for_each(|(i, row)| {
        let mut j = 0;
        while j < row.len() {
            let (mut j1, mut num) = (j, 0);
            while j1 < row.len() && row[j1].is_ascii_digit() {
                num = num * 10 + (row[j1] - b'0') as u32;
                j1 += 1;
            }
            if j1 > j {
                for (r, row) in input
                    .iter()
                    .enumerate()
                    .take((i + 2).min(input.len()))
                    .skip(i.saturating_sub(1))
                {
                    for (c, &x) in row
                        .iter()
                        .enumerate()
                        .take(j1.min(row.len() - 1) + 1)
                        .skip(j.saturating_sub(1))
                    {
                        if x.is_ascii_punctuation() && x != b'.' {
                            map.entry((r as u16, c as u16)).or_default().push(num);
                        }
                    }
                }
                j = j1;
            } else {
                j += 1;
            }
        }
    });
    map.values()
        .filter(|v| v.len() == 2)
        .map(|v| v[0] * v[1])
        .sum()
}

pub fn main() {
    let data = std::fs::read_to_string("data/2023/day3").unwrap();
    let input = parse(&data);
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let input = parse(data);
        assert_eq!(4361, part1(&input));
        assert_eq!(467835, part2(&input));
    }
}
