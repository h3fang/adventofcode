const LENGTH: usize = 5;

#[derive(Debug)]
struct Board {
    nums: Vec<i64>,
}

impl Board {
    fn from(lines: &[&str]) -> Self {
        let nums = lines
            .iter()
            .map(|r| {
                r.split_ascii_whitespace()
                    .map(|n| n.parse::<i64>().unwrap())
            })
            .flatten()
            .collect::<Vec<_>>();
        assert_eq!(LENGTH * LENGTH, nums.len());
        Self { nums }
    }

    fn is_row_marked(&self, r: usize) -> bool {
        for c in 0..LENGTH {
            if self.nums[r * LENGTH + c] != i64::MAX {
                return false;
            }
        }
        true
    }

    fn is_col_marked(&self, c: usize) -> bool {
        for r in 0..LENGTH {
            if self.nums[r * LENGTH + c] != i64::MAX {
                return false;
            }
        }
        true
    }

    fn is_bingo(&self) -> bool {
        for c in 0..LENGTH {
            if self.is_col_marked(c) {
                return true;
            }
        }
        for r in 0..LENGTH {
            if self.is_row_marked(r) {
                return true;
            }
        }
        false
    }

    fn unmarked(&self) -> i64 {
        self.nums.iter().filter(|&&n| n != i64::MAX).sum()
    }
}

fn part1(nums: &[i64], boards: &mut [Board]) -> i64 {
    for &n in nums {
        for b in boards.iter_mut() {
            b.nums.iter_mut().for_each(|c| {
                if *c == n {
                    *c = i64::MAX;
                }
            });
            if b.is_bingo() {
                return b.unmarked() * n;
            }
        }
    }
    0
}

fn part2(nums: &[i64], boards: &mut [Board]) -> i64 {
    let mut done = vec![false; boards.len()];
    let mut todo = boards.len();
    for &n in nums {
        for i in 0..boards.len() {
            if done[i] {
                continue;
            }
            boards[i].nums.iter_mut().for_each(|c| {
                if *c == n {
                    *c = i64::MAX;
                }
            });
            if boards[i].is_bingo() {
                if todo == 1 {
                    return boards[i].unmarked() * n;
                } else {
                    done[i] = true;
                    todo -= 1;
                }
            }
        }
    }
    0
}

pub fn main() {
    let data = std::fs::read_to_string("data/2021/day4").unwrap();
    let mut lines = data.lines();
    let nums = lines
        .next()
        .unwrap()
        .trim()
        .split(',')
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut curr = vec![];
    let mut boards = vec![];
    for line in lines {
        if line.is_empty() {
            if !curr.is_empty() {
                boards.push(Board::from(&curr));
                curr.clear();
            }
        } else {
            curr.push(line);
        }
    }
    if !curr.is_empty() {
        boards.push(Board::from(&curr));
    }

    println!("day4 part1: {}", part1(&nums, &mut boards));
    println!("day4 part2: {}", part2(&nums, &mut boards));
}
