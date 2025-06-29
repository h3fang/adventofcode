use ahash::AHashMap as HashMap;
use arrayvec::ArrayVec;
use std::{cmp::Reverse, collections::BinaryHeap, fmt::Display};

fn energy(amphipod: u8) -> usize {
    match amphipod {
        1 => 1,
        2 => 10,
        3 => 100,
        4 => 1000,
        _ => unreachable!(),
    }
}

fn fmt(amphipod: u8) -> char {
    match amphipod {
        0 => '.',
        1 => 'A',
        2 => 'B',
        3 => 'C',
        4 => 'D',
        _ => unreachable!(),
    }
}

#[derive(Debug, Clone, Copy)]
enum Position {
    Hallway(usize),
    Room(usize),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Grid<const D: usize> {
    hallway: [u8; 12],
    rooms: [[u8; D]; 4],
}

impl<const D: usize> Grid<D> {
    const fn amphipod_room(amphipod: u8) -> usize {
        amphipod as usize - 1
    }

    const fn room_amphipod(room: usize) -> u8 {
        room as u8 + 1
    }

    const fn room_exit(idx: usize) -> usize {
        2 * (idx + 1)
    }

    fn room_depth(&self, idx: usize) -> usize {
        self.rooms[idx].iter().position(|c| *c != 0).unwrap_or(D)
    }

    fn is_room_ready(&self, idx: usize) -> bool {
        for i in (0..D).rev() {
            let c = self.rooms[idx][i];
            if c == 0 {
                break;
            }
            if c != Self::room_amphipod(idx) {
                return false;
            }
        }
        true
    }

    fn is_room_done(&self, idx: usize) -> bool {
        self.rooms[idx]
            .iter()
            .all(|c| *c == Self::room_amphipod(idx))
    }

    fn is_done(&self) -> bool {
        (0..self.rooms.len()).all(|i| self.is_room_done(i))
    }

    fn is_hallway_clear(&self, a: usize, b: usize) -> bool {
        (a..=b).all(|i| self.hallway[i] == 0)
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

    fn all_moves(&self) -> ArrayVec<(Position, Position, usize), 28> {
        let mut result = ArrayVec::new();

        // hallway to room
        for i in [0, 1, 3, 5, 7, 9, 10] {
            let c = self.hallway[i];
            if c == 0 {
                continue;
            }
            let from = Position::Hallway(i);
            let room = Self::amphipod_room(c);
            if self.is_room_ready(room) {
                let to = Position::Room(room);
                if let Some(dist) = self.distance(from, to) {
                    result.push((from, to, dist * energy(c)));
                    return result;
                }
            }
        }

        let unfinished = (0..self.rooms.len())
            .filter(|i| !self.is_room_ready(*i))
            .filter_map(|i| {
                let d = self.room_depth(i);
                if d == D {
                    None
                } else {
                    Some((i, self.rooms[i][d]))
                }
            })
            .collect::<ArrayVec<_, 4>>();

        // room to room
        for &(i, c) in &unfinished {
            let from = Position::Room(i);

            let j = Self::amphipod_room(c);
            if self.is_room_ready(j) {
                let to = Position::Room(j);
                if let Some(dist) = self.distance(from, to) {
                    result.push((from, to, dist * energy(c)));
                    return result;
                }
            }
        }

        // room to hallway
        for (i, c) in unfinished {
            let from = Position::Room(i);
            let exit = Self::room_exit(i);
            let depth = self.room_depth(i);

            let mut right = exit - 1;
            while self.hallway[right] == 0 {
                let to = Position::Hallway(right);
                let dist = depth + 1 + exit - right;
                result.push((from, to, dist * energy(c)));
                if right == 0 {
                    break;
                } else if right == 1 {
                    right = 0;
                } else {
                    right -= 2;
                }
            }

            let mut right = exit + 1;
            while self.hallway[right] == 0 {
                let to = Position::Hallway(right);
                let dist = depth + 1 + right - exit;
                result.push((from, to, dist * energy(c)));
                if right == 10 {
                    break;
                } else if right == 9 {
                    right = 10;
                } else {
                    right += 2;
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
                g.hallway[i] = 0;
                c
            }
            Position::Room(i) => {
                let d = g.room_depth(i);
                let c = g.rooms[i][d];
                g.rooms[i][d] = 0;
                c
            }
        };
        match to {
            Position::Hallway(i) => {
                debug_assert_eq!(g.hallway[i], 0);
                g.hallway[i] = c;
            }
            Position::Room(i) => {
                let d = g.room_depth(i);
                g.rooms[i][d - 1] = c;
            }
        }
        g
    }
}

impl<const D: usize> Display for Grid<D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.hallway[..11]
                .iter()
                .map(|c| fmt(*c))
                .collect::<String>()
        )?;
        for i in 0..D {
            let r = self
                .rooms
                .iter()
                .map(|r| fmt(r[i]))
                .collect::<ArrayVec<_, 4>>();
            writeln!(f)?;
            write!(f, " |{}|{}|{}|{}| ", r[0], r[1], r[2], r[3])?;
        }
        Ok(())
    }
}

fn parse<const D: usize>(data: &str) -> Grid<D> {
    let cells: Vec<Vec<char>> = data
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut grid = Grid {
        hallway: [0; 12],
        rooms: [[0; D]; 4],
    };
    if D == 4 {
        grid.rooms[0][1] = 4;
        grid.rooms[0][2] = 4;

        grid.rooms[1][1] = 3;
        grid.rooms[1][2] = 2;

        grid.rooms[2][1] = 2;
        grid.rooms[2][2] = 1;

        grid.rooms[3][1] = 1;
        grid.rooms[3][2] = 3;
    }
    for i in 0..4 {
        let x = 1 + 2 * (i + 1);
        for (j, y) in [(0, 2), (D - 1, 3)] {
            grid.rooms[i][j] = match cells[y][x] {
                'A' => 1,
                'B' => 2,
                'C' => 3,
                'D' => 4,
                x => panic!("invalid amphipod {x}"),
            };
        }
    }
    grid
}

fn solve<const D: usize, const E: usize>(g: Grid<D>) -> usize {
    let mut costs = HashMap::new();
    let mut q = BinaryHeap::new();
    q.push((Reverse(0), g));
    while let Some((cost, g)) = q.pop() {
        if g.is_done() {
            return cost.0;
        }
        let key: &[u32; E] = unsafe { std::mem::transmute(&g) };
        if let Some(c) = costs.get(key) {
            if cost.0 > *c {
                continue;
            }
        }
        for (from, to, delta_cost) in g.all_moves() {
            let next = g.make_move((from, to));
            let key: &[u32; E] = unsafe { std::mem::transmute(&next) };
            let old = costs.get(key).cloned().unwrap_or(usize::MAX);
            let new = cost.0 + delta_cost;
            if new < old {
                costs.insert(*key, new);
                q.push((Reverse(new), next));
            }
        }
    }
    unreachable!()
}

pub fn main() {
    let data = std::fs::read_to_string("data/2021/day23").unwrap();
    println!("day23 part1: {}", solve::<2, 6>(parse::<2>(&data)));
    println!("day23 part2: {}", solve::<4, 8>(parse::<4>(&data)));
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
        assert_eq!(12521, solve::<2, 6>(parse::<2>(data)));
        assert_eq!(44169, solve::<4, 8>(parse::<4>(data)));
    }
}
