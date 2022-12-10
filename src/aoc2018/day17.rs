struct Map {
    grid: Vec<Vec<u8>>,
    spring_x: usize,
}

impl Map {
    fn flow(&mut self) {
        let m = self.grid.len() as i32;
        let n = self.grid[0].len() as i32;
        let mut q = vec![];
        q.push((1, self.spring_x as i32));
        while let Some(&(i, j)) = q.last() {
            if i + 1 == m {
                self.grid[i as usize][j as usize] = b'|';
                q.pop();
                continue;
            }
            match self.grid[i as usize + 1][j as usize] {
                b'.' => {
                    self.grid[i as usize][j as usize] = b'|';
                    q.push((i + 1, j));
                }
                b'|' => {
                    self.grid[i as usize][j as usize] = b'|';
                    q.pop();
                }
                b'#' | b'~' => {
                    q.pop();
                    let mut blocked = true;
                    let mut left = j - 1;
                    while left >= 0 {
                        match self.grid[i as usize][left as usize] {
                            b'.' | b'|' => {}
                            b'#' | b'~' => break,
                            _ => unreachable!(),
                        }
                        match self.grid[i as usize + 1][left as usize] {
                            b'.' | b'|' => {
                                if self.grid[i as usize][left as usize] != b'|' {
                                    self.grid[i as usize][left as usize] = b'|';
                                    q.push((i, left));
                                }
                                blocked = false;
                                break;
                            }
                            b'#' | b'~' => {}
                            _ => unreachable!(),
                        }
                        left -= 1;
                    }
                    let mut right = j + 1;
                    while right < n {
                        match self.grid[i as usize][right as usize] {
                            b'.' | b'|' => {}
                            b'#' | b'~' => break,
                            _ => unreachable!(),
                        }
                        match self.grid[i as usize + 1][right as usize] {
                            b'.' | b'|' => {
                                if self.grid[i as usize][right as usize] != b'|' {
                                    self.grid[i as usize][right as usize] = b'|';
                                    q.push((i, right));
                                }
                                blocked = false;
                                break;
                            }
                            b'#' | b'~' => {}
                            _ => unreachable!(),
                        }
                        right += 1;
                    }

                    let c = if blocked { b'~' } else { b'|' };
                    self.grid[i as usize][left as usize + 1..right as usize]
                        .iter_mut()
                        .for_each(|e| *e = c);
                }
                _ => unreachable!(),
            }
            // print_map(self);
        }
    }
}

fn parse(data: &str) -> Map {
    let walls = data
        .trim()
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(", ").unwrap();
            let (a, n1) = a.split_once('=').unwrap();
            let n1 = n1.parse::<usize>().unwrap();
            let (_, range) = b.split_once('=').unwrap();
            let (left, right) = range.split_once("..").unwrap();
            let left = left.parse::<usize>().unwrap();
            let right = right.parse::<usize>().unwrap();
            (a, n1, left, right)
        })
        .collect::<Vec<_>>();

    let mut min = (usize::MAX, usize::MAX);
    let mut max = (usize::MIN, usize::MIN);
    for &(a, n1, left, right) in &walls {
        if a == "x" {
            min.0 = min.0.min(n1);
            max.0 = max.0.max(n1);
            min.1 = min.1.min(left);
            max.1 = max.1.max(right);
        } else {
            min.1 = min.1.min(n1);
            max.1 = max.1.max(n1);
            min.0 = min.0.min(left);
            max.0 = max.0.max(right);
        }
    }
    let width = max.0 - min.0 + 3;
    let height = max.1 - min.1 + 2;
    let mut grid = vec![vec![b'.'; width]; height];
    for (a, num, left, right) in walls {
        if a == "x" {
            let j = num - min.0 + 1;
            for row in &mut grid[left - min.1 + 1..=right - min.1 + 1] {
                row[j] = b'#';
            }
        } else {
            for j in left..=right {
                grid[num - min.1 + 1][j - min.0 + 1] = b'#';
            }
        }
    }
    let spring_x = 500 - min.0 + 1;
    grid[0][spring_x] = b'+';
    Map { grid, spring_x }
}

// fn print_map(map: &Map) {
//     for r in &map.grid {
//         println!("{}", unsafe { std::str::from_utf8_unchecked(r) });
//     }
// }

fn part1(map: &Map) -> usize {
    map.grid
        .iter()
        .flatten()
        .filter(|&&b| b == b'|' || b == b'~')
        .count()
}

fn part2(map: &Map) -> usize {
    map.grid.iter().flatten().filter(|&&b| b == b'~').count()
}

pub fn main() {
    let data = std::fs::read_to_string("data/2018/day17").unwrap();
    let mut map = parse(&data);
    map.flow();
    println!("part1: {}", part1(&map));
    println!("part2: {}", part2(&map));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "
x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504";
        let mut map = parse(&data);
        map.flow();
        assert_eq!(57, part1(&map));
        assert_eq!(29, part2(&map));
    }
}
