use std::str::Chars;

fn code_to_row(s: &str) -> usize {
    binary(s.chars(), 'F', 'B', 0, 127)
}

fn code_to_col(s: &str) -> usize {
    binary(s.chars(), 'L', 'R', 0, 7)
}

fn binary(mut s: Chars, left: char, right: char, low: usize, high: usize) -> usize {
    if let Some(c) = s.next() {
        if c == left {
            binary(s, left, right, low, low + (high - low) / 2)
        } else if c == right {
            binary(s, left, right, low + (high - low) / 2 + 1, high)
        } else {
            panic!("unknown symbol {c}");
        }
    } else {
        low
    }
}

pub fn main() {
    let mut numbers = std::fs::read_to_string("data/2020/day5")
        .unwrap()
        .lines()
        .map(|line| {
            let (row, col) = line.split_at(7);
            let row = code_to_row(row);
            let col = code_to_col(col);
            row * 8 + col
        })
        .collect::<Vec<_>>();
    numbers.sort_unstable();
    let max = numbers.iter().last().unwrap_or(&0);
    let mut id = 0;
    for w in numbers.windows(2) {
        if w[1] - w[0] > 1 {
            id = w[0] + 1;
            break;
        }
    }
    println!("day5 part1: {max}\nday5 part2: {id}");
}
