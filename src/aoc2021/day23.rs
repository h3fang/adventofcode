use ahash::AHashMap as HashMap;
use arrayvec::ArrayVec;
use std::{cmp::Reverse, collections::BinaryHeap, fmt::Display};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Cell {
    Empty,
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl Cell {
    fn energy(&self) -> usize {
        match self {
            Cell::Amber => 1,
            Cell::Bronze => 10,
            Cell::Copper => 100,
            Cell::Desert => 1000,
            Cell::Empty => unreachable!(),
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Cell::Empty => '.',
                Cell::Amber => 'A',
                Cell::Bronze => 'B',
                Cell::Copper => 'C',
                Cell::Desert => 'D',
            }
        )
    }
}

#[derive(Debug, Clone, Copy)]
enum Position {
    Hallway(usize),
    Room(usize),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Grid {
    room_size: usize,
    hallway: [Cell; 11],
    rooms: ArrayVec<ArrayVec<Cell, 4>, 4>,
}

impl Grid {
    fn amphipod_room(amphipod: &Cell) -> usize {
        match amphipod {
            Cell::Amber => 0,
            Cell::Bronze => 1,
            Cell::Copper => 2,
            Cell::Desert => 3,
            Cell::Empty => unreachable!(),
        }
    }

    fn room_amphipod(idx: usize) -> Cell {
        match idx {
            0 => Cell::Amber,
            1 => Cell::Bronze,
            2 => Cell::Copper,
            3 => Cell::Desert,
            _ => unreachable!(),
        }
    }

    fn room_depth(&self, idx: usize) -> usize {
        self.room_size - self.rooms[idx].len()
    }

    fn is_room_ready(&self, idx: usize) -> bool {
        let amp = Self::room_amphipod(idx);
        self.rooms[idx].iter().all(|c| *c == amp)
    }

    fn is_room_done(&self, idx: usize) -> bool {
        self.rooms[idx].len() == self.room_size
            && self.rooms[idx]
                .iter()
                .all(|c| *c == Self::room_amphipod(idx))
    }

    fn is_done(&self) -> bool {
        (0..self.rooms.len()).all(|i| self.is_room_done(i))
    }

    fn room_exit(idx: usize) -> usize {
        2 * (idx + 1)
    }

    fn is_hallway_clear(&self, a: usize, b: usize) -> bool {
        (a..=b).all(|i| self.hallway[i] == Cell::Empty)
    }

    fn hallway_distance(&self, a: usize, b: usize) -> Option<usize> {
        if self.is_hallway_clear(a, b) {
            Some(b - a)
        } else {
            None
        }
    }

    fn distance(&self, from: Position, to: Position) -> Option<usize> {
        match (from, to) {
            (Position::Hallway(_), Position::Hallway(_)) => unreachable!(),
            (Position::Hallway(ai), Position::Room(b)) => {
                let bi = Self::room_exit(b);
                debug_assert!(ai != bi);
                let dist = if ai < bi {
                    self.hallway_distance(ai + 1, bi)
                } else {
                    self.hallway_distance(bi, ai - 1)
                };
                dist.map(|dist| dist + 1 + self.room_depth(b))
            }
            (Position::Room(a), Position::Hallway(bi)) => {
                let ai = Self::room_exit(a);
                debug_assert!(ai != bi);
                self.hallway_distance(ai.min(bi), bi.max(ai))
                    .map(|dist| dist + self.room_depth(a) + 1)
            }
            (Position::Room(a), Position::Room(b)) => {
                debug_assert!(a != b);
                let (a, b) = if a < b { (a, b) } else { (b, a) };
                let ai = Self::room_exit(a);
                let bi = Self::room_exit(b);
                self.hallway_distance(ai, bi)
                    .map(|dist| dist + self.room_depth(a) + 1 + self.room_depth(b))
            }
        }
    }

    fn all_moves(&self) -> Vec<(Position, Position, usize)> {
        let mut result = vec![];

        // from hallway
        for (i, c) in self.hallway.iter().enumerate() {
            if c == &Cell::Empty {
                continue;
            }
            let from = Position::Hallway(i);
            // hallway to room
            let room = Self::amphipod_room(c);
            if self.is_room_ready(room) {
                let to = Position::Room(room);
                if let Some(dist) = self.distance(from, to) {
                    result.push((from, to, dist * c.energy()));
                }
            }
        }

        // from room
        for (i, ci) in self.rooms.iter().enumerate() {
            if self.is_room_ready(i) {
                continue;
            }
            if let Some(c) = ci.last() {
                let from = Position::Room(i);

                // room to room
                let j = Self::amphipod_room(c);
                if self.is_room_ready(j) {
                    let to = Position::Room(j);
                    if let Some(dist) = self.distance(from, to) {
                        result.push((from, to, dist * c.energy()));
                        // Since it's able to move to room, there is no need
                        // to try to move into the hallway.
                        continue;
                    }
                }

                // room to hallway
                for i in [0, 1, 3, 5, 7, 9, 10] {
                    if self.hallway[i] != Cell::Empty {
                        continue;
                    }
                    let to = Position::Hallway(i);
                    if let Some(dist) = self.distance(from, to) {
                        result.push((from, to, dist * c.energy()));
                    }
                }
            }
        }

        result
    }

    fn make_move(&self, (from, to): (Position, Position)) -> Self {
        let mut g = self.clone();
        let c = match from {
            Position::Hallway(i) => {
                let c = g.hallway[i];
                g.hallway[i] = Cell::Empty;
                c
            }
            Position::Room(i) => g.rooms[i].pop().unwrap(),
        };
        match to {
            Position::Hallway(i) => {
                debug_assert_eq!(g.hallway[i as usize], Cell::Empty);
                g.hallway[i] = c;
            }
            Position::Room(i) => {
                g.rooms[i].push(c);
            }
        }
        g
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.hallway
                .iter()
                .map(|c| c.to_string())
                .collect::<String>()
        )?;
        for i in (0..self.room_size).rev() {
            let r = self
                .rooms
                .iter()
                .map(|r| {
                    r.get(i)
                        .map(|c| c.to_string())
                        .unwrap_or_else(|| ".".to_string())
                })
                .collect::<Vec<_>>();
            writeln!(f)?;
            write!(f, " |{}|{}|{}|{}| ", r[0], r[1], r[2], r[3])?;
        }
        Ok(())
    }
}

fn parse(data: &str) -> Grid {
    let cells: Vec<Vec<char>> = data
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut grid = Grid {
        room_size: 2,
        hallway: [Cell::Empty; 11],
        rooms: ArrayVec::from_iter((0..4).map(|_| ArrayVec::new())),
    };
    for i in 0..4 {
        let x = 1 + 2 * (i + 1);
        for y in (2..=3).rev() {
            grid.rooms[i].push(match cells[y][x] {
                'A' => Cell::Amber,
                'B' => Cell::Bronze,
                'C' => Cell::Copper,
                'D' => Cell::Desert,
                x => panic!("invalid amphipod {}", x),
            });
        }
    }
    grid
}

fn solve(g: Grid) -> usize {
    let mut costs: HashMap<Grid, usize> = HashMap::new();
    let mut q = BinaryHeap::new();
    q.push((Reverse(0), g));
    while let Some((cost, g)) = q.pop() {
        if g.is_done() {
            return cost.0;
        }
        if let Some(c) = costs.get(&g) {
            if cost.0 > *c {
                continue;
            }
        }
        for (from, to, delta_cost) in g.all_moves() {
            let next = g.make_move((from, to));
            let old = costs.get(&next).cloned().unwrap_or(usize::MAX);
            let new = cost.0 + delta_cost;
            if new < old {
                costs.insert(next.clone(), new);
                q.push((Reverse(new), next));
            }
        }
    }
    unreachable!()
}

fn part2(mut g: Grid) -> usize {
    g.rooms[0].insert(1, Cell::Desert);
    g.rooms[0].insert(1, Cell::Desert);

    g.rooms[1].insert(1, Cell::Copper);
    g.rooms[1].insert(1, Cell::Bronze);

    g.rooms[2].insert(1, Cell::Bronze);
    g.rooms[2].insert(1, Cell::Amber);

    g.rooms[3].insert(1, Cell::Amber);
    g.rooms[3].insert(1, Cell::Copper);

    g.room_size = 4;
    solve(g)
}

pub fn main() {
    let data = std::fs::read_to_string("data/2021/day23").unwrap();
    let g = parse(&data);
    println!("day23 part1: {}", solve(g.clone()));
    println!("day23 part2: {}", part2(g));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "#############
#...........#
###B#C#B#D###
  #A#D#C#A#  
  #########  ";
        let g = parse(&data);
        // assert_eq!(12521, solve(g.clone()));
        assert_eq!(44169, part2(g));
    }
}
