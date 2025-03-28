use rayon::prelude::*;

#[derive(Clone, PartialEq)]
enum Cell {
    Empty,
    East,
    South,
}

struct Grid {
    width: usize,
    height: usize,
    grid: Vec<Cell>,
}

impl Grid {
    fn step(&mut self) -> bool {
        let mut changed = false;
        let mut next = self.grid.clone();

        // east
        let pn = ((&mut next) as *mut Vec<Cell>) as usize;
        let pc = ((&mut changed) as *mut bool) as usize;
        (0..self.height).into_par_iter().for_each(|y| {
            let next = pn as *mut Vec<Cell>;
            let changed = pc as *mut bool;
            for x in 0..self.width {
                let i1 = y * self.width + x;
                let i2 = if x + 1 == self.width { i1 - x } else { i1 + 1 };
                if Cell::East == self.grid[i1] && self.grid[i2] == Cell::Empty {
                    unsafe {
                        (*next).swap(i1, i2);
                        *changed = true;
                    }
                }
            }
        });

        self.grid = next.clone();

        let pn = ((&mut self.grid) as *mut Vec<Cell>) as usize;
        let pc = ((&mut changed) as *mut bool) as usize;
        (0..self.height).into_par_iter().for_each(|y| {
            let grid = pn as *mut Vec<Cell>;
            let changed = pc as *mut bool;
            for x in 0..self.width {
                let i1 = y * self.width + x;
                let i2 = if y + 1 == self.height {
                    x
                } else {
                    i1 + self.width
                };
                if Cell::South == next[i1] && next[i2] == Cell::Empty {
                    unsafe {
                        (*grid).swap(i1, i2);
                        *changed = true;
                    }
                }
            }
        });

        changed
    }
}

fn parse(data: &str) -> Grid {
    let mut height = 0;
    let grid = data
        .lines()
        .flat_map(|line| {
            height += 1;
            line.trim().as_bytes().iter().map(|b| match b {
                b'.' => Cell::Empty,
                b'>' => Cell::East,
                b'v' => Cell::South,
                _ => panic!("invalid cell"),
            })
        })
        .collect::<Vec<_>>();
    Grid {
        width: grid.len() / height,
        height,
        grid,
    }
}

fn part1(grid: &mut Grid) -> usize {
    let mut i = 1;
    while grid.step() {
        i += 1;
    }
    i
}

pub fn main() {
    let data = std::fs::read_to_string("data/2021/day25").unwrap();
    let mut grid = parse(&data);
    println!("day25 part1: {}", part1(&mut grid));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "v...>>.vv>
        .vv>>.vv..
        >>.>v>...v
        >>v>>.>.v.
        v>v.vv.v..
        >.>>..v...
        .vv..>.>v.
        v.v..>>v.v
        ....v..v.>";
        let mut grid = parse(data);
        assert_eq!(58, part1(&mut grid));
    }
}
