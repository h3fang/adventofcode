fn parse(data: &str) -> Vec<Vec<i8>> {
    data.trim()
        .lines()
        .map(|line| line.as_bytes().iter().map(|b| (b - b'0') as i8).collect())
        .collect()
}

fn part1(map: &[Vec<i8>]) -> usize {
    let m = map.len();
    let n = map[0].len();
    let mut visible = vec![vec![false; n]; m];
    for (i, row) in map.iter().enumerate() {
        // left to right
        let mut prev = -1;
        for (j, &h) in row.iter().enumerate() {
            if h > prev {
                prev = h;
                visible[i][j] = true;
            }
        }

        // right to left
        prev = -1;
        for (j, &h) in row.iter().enumerate().rev() {
            if h > prev {
                prev = h;
                visible[i][j] = true;
            }
        }
    }

    for j in 0..n {
        // top to bottom
        let mut prev = -1;
        for i in 0..m {
            let h = map[i][j];
            if h > prev {
                prev = h;
                visible[i][j] = true;
            }
        }
        // bottom to top
        prev = -1;
        for i in (0..m).rev() {
            let h = map[i][j];
            if h > prev {
                prev = h;
                visible[i][j] = true;
            }
        }
    }
    visible.iter().flatten().filter(|&&v| v).count()
}

fn part2(map: &[Vec<i8>]) -> usize {
    let m = map.len();
    let n = map[0].len();
    map.iter()
        .enumerate()
        .skip(1)
        .take(m - 2)
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .skip(1)
                .take(n - 2)
                .map(move |(j, tree)| {
                    let mut top = 0;
                    for row in map[0..i].iter().rev() {
                        match row[j].cmp(tree) {
                            std::cmp::Ordering::Less => top += 1,
                            _ => {
                                top += 1;
                                break;
                            }
                        }
                    }

                    let mut bottom = 0;
                    for row in &map[i + 1..] {
                        match row[j].cmp(tree) {
                            std::cmp::Ordering::Less => bottom += 1,
                            _ => {
                                bottom += 1;
                                break;
                            }
                        }
                    }

                    let mut left = 0;
                    for j1 in (0..j).rev() {
                        match map[i][j1].cmp(tree) {
                            std::cmp::Ordering::Less => left += 1,
                            _ => {
                                left += 1;
                                break;
                            }
                        }
                    }

                    let mut right = 0;
                    for &e in &map[i][j + 1..n] {
                        match e.cmp(tree) {
                            std::cmp::Ordering::Less => right += 1,
                            _ => {
                                right += 1;
                                break;
                            }
                        }
                    }
                    left * right * top * bottom
                })
        })
        .max()
        .unwrap()
}

pub fn main() {
    let data = std::fs::read_to_string("data/2022/day8").unwrap();
    let map = parse(&data);
    println!("part1: {}", part1(&map));
    println!("part2: {}", part2(&map));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "
30373
25512
65332
33549
35390";
        let map = parse(data);
        assert_eq!(21, part1(&map));
        assert_eq!(8, part2(&map));
    }
}
