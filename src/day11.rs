use anyhow::Result;
use arrayvec::ArrayVec;
use std::{
    fmt, fs,
    io::{self, BufRead},
};

#[derive(Clone, PartialEq)]
struct Grid {
    array: Vec<char>,
    num_rows: usize,
    num_columns: usize,
}

impl Grid {
    fn get(&self, row: usize, column: usize) -> char {
        self.array[row * self.num_columns + column]
    }

    fn set(&mut self, row: usize, column: usize, value: char) {
        self.array[row * self.num_columns + column] = value;
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rows = self
            .array
            .chunks(self.num_columns)
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<_>>();
        write!(f, "{}", rows.join("\n"))
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

fn parse(file_path: &str) -> Result<Grid> {
    let data_file = fs::File::open(file_path)?;
    let mut r = io::BufReader::new(data_file)
        .lines()
        .flatten()
        .map(|line| {
            let mut c: Vec<char> = line.chars().collect();
            c.insert(0, ' ');
            c.push(' ');
            c
        })
        .collect::<Vec<_>>();
    r.insert(0, vec![' '; r[0].len()]);
    r.push(vec![' '; r[0].len()]);
    Ok(Grid {
        num_rows: r.len(),
        num_columns: r[0].len(),
        array: r.into_iter().flatten().collect(),
    })
}

fn neighbors_part1(_seats: &Grid, i: usize, j: usize) -> ArrayVec<(usize, usize), 8> {
    ArrayVec::from([
        (i - 1, j - 1),
        (i - 1, j),
        (i - 1, j + 1),
        (i, j - 1),
        (i, j + 1),
        (i + 1, j - 1),
        (i + 1, j),
        (i + 1, j + 1),
    ])
}

fn neighbors_part2(seats: &Grid, i: usize, j: usize) -> ArrayVec<(usize, usize), 8> {
    let directions = [
        (0, 1),
        (1, 1),
        (1, 0),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];
    directions
        .iter()
        .filter_map(|(dx, dy)| {
            let mut dist = 1;
            loop {
                let m = i as i32 + dist * dx;
                let n = j as i32 + dist * dy;
                if m <= 0
                    || m >= seats.num_rows as i32 - 1
                    || n <= 0
                    || n >= seats.num_columns as i32 - 1
                {
                    return None;
                }
                if seats.get(m as usize, n as usize) != '.' {
                    return Some((m as usize, n as usize));
                }
                dist += 1;
            }
        })
        .collect()
}

fn step(
    seats: &mut Grid,
    threshold: usize,
    neighbor_fn: fn(&Grid, usize, usize) -> ArrayVec<(usize, usize), 8>,
) -> usize {
    let mut changed = 0;
    let r = seats.clone();
    for i in 1..r.num_rows - 1 {
        for j in 1..r.num_columns - 1 {
            let neighbors = neighbor_fn(&r, i, j);
            match r.get(i, j) {
                '#' => {
                    let c = neighbors
                        .iter()
                        .filter(|(m, n)| r.get(*m, *n) == '#')
                        .count();
                    if c >= threshold {
                        seats.set(i, j, 'L');
                        changed += 1;
                    }
                }
                'L' => {
                    if neighbors.iter().all(|(m, n)| r.get(*m, *n) != '#') {
                        seats.set(i, j, '#');
                        changed += 1;
                    }
                }
                _ => {}
            }
        }
    }
    changed
}

fn part(
    seats: &mut Grid,
    threshold: usize,
    neighbor_fn: fn(&Grid, usize, usize) -> ArrayVec<(usize, usize), 8>,
) -> usize {
    while step(seats, threshold, neighbor_fn) > 0 {}
    seats.array.iter().filter(|&c| *c == '#').count()
}

pub fn main(file_path: &str) -> Result<()> {
    let mut seats = parse(file_path)?;

    // part 1
    println!(
        "day 11 part1: {}",
        part(&mut seats.clone(), 4, neighbors_part1)
    );

    // part 2
    println!("day 11 part2: {}", part(&mut seats, 5, neighbors_part2));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neightbors_part2() {
        // step 0
        let mut seats = parse("data/day11-0").unwrap();

        // step 1
        step(&mut seats, 5, neighbors_part2);
        let expected = parse("data/day11-1").unwrap();
        assert_eq!(seats, expected);

        // step 2
        step(&mut seats, 5, neighbors_part2);
        let expected = parse("data/day11-2").unwrap();
        assert_eq!(seats, expected);

        // step 3
        step(&mut seats, 5, neighbors_part2);
        let expected = parse("data/day11-3").unwrap();
        assert_eq!(seats, expected);
    }
}
