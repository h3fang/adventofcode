use ahash::AHashMap as HashMap;
use ahash::AHashSet as HashSet;
use std::{collections::BinaryHeap, ops::BitOr};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn neighbors(&self) -> [Self; 4] {
        [
            Self::new(self.x - 1, self.y),
            Self::new(self.x + 1, self.y),
            Self::new(self.x, self.y - 1),
            Self::new(self.x, self.y + 1),
        ]
    }
}

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct KeyCollection(u32);

impl KeyCollection {
    fn index(key: &u8) -> usize {
        if key.is_ascii_uppercase() {
            (key - b'A') as usize
        } else {
            (key - b'a') as usize
        }
    }

    fn insert(&mut self, key: u8) {
        self.0 |= 1 << Self::index(&key);
    }

    fn contains(&self, key: &u8) -> bool {
        self.0 & (1 << Self::index(key)) > 0
    }
}

impl BitOr for KeyCollection {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

struct Vault {
    grid: Vec<Vec<u8>>,
    key_pos: HashMap<u8, Position>,
    keys: KeyCollection,
    entrances: Vec<Position>,
}

impl Vault {
    fn from_grid(grid: &[Vec<u8>]) -> Self {
        let mut grid = grid.to_vec();
        let mut key_pos = HashMap::new();
        let mut keys = KeyCollection::default();
        let mut entrances = vec![];
        for (y, row) in grid.iter_mut().enumerate() {
            for (x, b) in row.iter_mut().enumerate() {
                match b {
                    b'#' | b'.' => {}
                    b'@' => {
                        *b = b'.';
                        entrances.push(Position::new(x, y));
                    }
                    b if b.is_ascii_lowercase() => {
                        key_pos.insert(*b, Position::new(x, y));
                        keys.insert(*b);
                    }
                    _ => {}
                }
            }
        }
        Self {
            grid,
            key_pos,
            keys,
            entrances,
        }
    }
}

#[derive(Debug, Clone)]
struct Route {
    dest: u8,
    dist: usize,
    keys: KeyCollection,
    doors: KeyCollection,
}

fn all_paths(vault: &Vault, from: Position) -> Vec<Route> {
    let mut q = BinaryHeap::new();
    let mut closed = HashSet::new();
    let mut result = vec![];
    q.push((
        0i64,
        from,
        KeyCollection::default(),
        KeyCollection::default(),
    ));
    while let Some((dist, curr, mut keys, mut doors)) = q.pop() {
        let dist = -dist;
        closed.insert(curr);
        for p in curr.neighbors() {
            let cell = vault.grid[p.y][p.x];
            if cell != b'#' && !closed.contains(&p) {
                if cell.is_ascii_lowercase() {
                    keys.insert(cell);
                    result.push(Route {
                        dest: cell,
                        dist: dist as usize + 1,
                        keys,
                        doors,
                    });
                } else if cell.is_ascii_uppercase() {
                    doors.insert(cell);
                }
                q.push((-(dist + 1), p, keys, doors));
            }
        }
    }
    result
}

fn shortest_path(vault: &Vault) -> usize {
    let mut paths = HashMap::new();
    let mut cost: HashMap<(KeyCollection, Vec<Position>), i64> = HashMap::new();
    let mut q = BinaryHeap::new();
    q.push((0i64, (KeyCollection::default(), vault.entrances.clone())));

    while let Some((dist, (collected, positions))) = q.pop() {
        let dist = -dist;
        if collected == vault.keys {
            return dist as usize;
        }
        for (i, &curr) in positions.iter().enumerate() {
            paths.entry(curr).or_insert_with(|| all_paths(vault, curr));
            let ps = paths.get(&curr).unwrap().to_vec();
            for r in &ps {
                if collected.contains(&r.dest) || (r.doors.0 & collected.0) != r.doors.0 {
                    continue;
                }

                let collected = collected | r.keys;
                let dest = *vault.key_pos.get(&r.dest).unwrap();
                let mut positions = positions.clone();
                positions[i] = dest;
                let state = (collected, positions);
                if (dist + r.dist as i64) < *cost.get(&state).unwrap_or(&i64::MAX) {
                    cost.insert(state.clone(), dist + r.dist as i64);
                    q.push((-(dist + r.dist as i64), state));
                }
            }
        }
    }

    0
}

fn part1(grid: &[Vec<u8>]) -> (usize, Vault) {
    let vault = Vault::from_grid(grid);
    let p1 = shortest_path(&vault);
    (p1, vault)
}

fn part2(mut vault: Vault) -> usize {
    let e = vault.entrances[0];
    vault.entrances = vec![
        Position::new(e.x - 1, e.y - 1),
        Position::new(e.x - 1, e.y + 1),
        Position::new(e.x + 1, e.y - 1),
        Position::new(e.x + 1, e.y + 1),
    ];
    vault.grid[e.y][e.x] = b'#';
    vault.grid[e.y + 1][e.x] = b'#';
    vault.grid[e.y - 1][e.x] = b'#';
    vault.grid[e.y][e.x + 1] = b'#';
    vault.grid[e.y][e.x - 1] = b'#';

    shortest_path(&vault)
}

pub fn main() {
    let data = std::fs::read_to_string("data/2019/day18").unwrap();
    let grid = data
        .trim()
        .lines()
        .map(|row| row.trim().as_bytes().to_vec())
        .collect::<Vec<_>>();
    let (p1, vault) = part1(&grid);
    println!("day 18 part1: {p1}");
    println!("day 18 part2: {}", part2(vault));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "#########
        #b.A.@.a#
        #########";
        let grid = data
            .trim()
            .lines()
            .map(|row| row.trim().as_bytes().to_vec())
            .collect::<Vec<_>>();
        assert_eq!(8, part1(&grid).0);
    }

    #[test]
    fn case2() {
        let data = "########################
        #f.D.E.e.C.b.A.@.a.B.c.#
        ######################.#
        #d.....................#
        ########################";
        let grid = data
            .trim()
            .lines()
            .map(|row| row.trim().as_bytes().to_vec())
            .collect::<Vec<_>>();
        assert_eq!(86, part1(&grid).0);
    }

    #[test]
    fn case3() {
        let data = "#################
        #i.G..c...e..H.p#
        ########.########
        #j.A..b...f..D.o#
        ########@########
        #k.E..a...g..B.n#
        ########.########
        #l.F..d...h..C.m#
        #################";
        let grid = data
            .trim()
            .lines()
            .map(|row| row.trim().as_bytes().to_vec())
            .collect::<Vec<_>>();
        assert_eq!(136, part1(&grid).0);
    }
}
