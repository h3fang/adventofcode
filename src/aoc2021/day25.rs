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
    fn step(&mut self) -> usize {
        let mut changed = 0;
        let mut next = self.grid.clone();

        let index = |x: usize, y: usize| -> usize { y * self.width + x };

        // east
        for y in 0..self.height {
            for x in 0..self.width {
                let i1 = index(x, y);
                let i2 = index((x + 1) % self.width, y);
                if Cell::East == self.grid[i1] && self.grid[i2] == Cell::Empty {
                    next.swap(i1, i2);
                    changed += 1;
                }
            }
        }

        self.grid = next.clone();

        // south
        for y in 0..self.height {
            for x in 0..self.width {
                let i1 = index(x, y);
                let i2 = index(x, (y + 1) % self.height);
                if Cell::South == next[i1] && next[i2] == Cell::Empty {
                    self.grid.swap(i1, i2);
                    changed += 1;
                }
            }
        }

        changed
    }
}

fn parse(data: &str) -> Grid {
    let mut height = 0;
    let grid = data
        .lines()
        .map(|line| {
            height += 1;
            line.trim().as_bytes().iter().map(|b| match b {
                b'.' => Cell::Empty,
                b'>' => Cell::East,
                b'v' => Cell::South,
                _ => panic!("invalid cell"),
            })
        })
        .flatten()
        .collect::<Vec<_>>();
    Grid {
        width: grid.len() / height,
        height,
        grid,
    }
}

fn part1(grid: &mut Grid) -> usize {
    let mut i = 1;
    loop {
        let changed = grid.step();
        if changed == 0 {
            return i;
        }
        i += 1;
    }
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
        let mut grid = parse(&data);
        assert_eq!(58, part1(&mut grid));
    }
}
