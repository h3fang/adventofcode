use std::collections::VecDeque;

use ahash::{HashMap, HashSet};

#[derive(Clone, PartialEq)]
enum Kind {
    Elf,
    Goblin,
}

impl From<u8> for Kind {
    fn from(k: u8) -> Self {
        match k {
            b'G' => Kind::Goblin,
            b'E' => Kind::Elf,
            _ => panic!(),
        }
    }
}

impl Kind {
    fn enemy(&self) -> Self {
        match self {
            Kind::Elf => Kind::Goblin,
            Kind::Goblin => Kind::Elf,
        }
    }

    fn to_u8(&self) -> u8 {
        match self {
            Kind::Elf => b'E',
            Kind::Goblin => b'G',
        }
    }
}

#[derive(Clone)]
struct Unit {
    kind: Kind,
    hp: u8,
}

#[derive(Clone)]
struct Map {
    grid: Vec<Vec<u8>>,
    units: HashMap<(i8, i8), Unit>,
    turns: usize,
    elv_attack: u8,
    elv_died: bool,
}

impl Map {
    fn has_enemy(&self, enemy: Kind) -> bool {
        self.units.values().any(|u| u.kind == enemy)
    }

    fn find_enemy_in_range(&self, (i, j): (i8, i8)) -> Option<(i8, i8)> {
        let enemy = self.units.get(&(i, j)).unwrap().kind.enemy();
        let mut min = u8::MAX;
        let mut result = None;
        for (i1, j1) in [(i - 1, j), (i, j - 1), (i, j + 1), (i + 1, j)] {
            if i1 < 0 || j1 < 0 || i1 == self.grid.len() as i8 || j1 == self.grid[0].len() as i8 {
                continue;
            }
            if let Some(u) = self.units.get(&(i1, j1)) {
                if u.kind == enemy && u.hp < min {
                    min = u.hp;
                    result = Some((i1, j1));
                }
            }
        }
        result
    }

    fn find_path(&self, (i0, j0): (i8, i8)) -> Option<(i8, i8)> {
        let enemy = self.units.get(&(i0, j0)).unwrap().kind.enemy().to_u8();
        let mut q = VecDeque::new();
        q.push_back((0, None, (i0, j0)));
        let mut visited = HashSet::default();
        let mut candidates = vec![];
        let mut min = i32::MAX;
        let m = self.grid.len() as i8;
        let n = self.grid[0].len() as i8;
        while let Some((d, second, (i, j))) = q.pop_front() {
            for (i1, j1) in [(i - 1, j), (i, j - 1), (i, j + 1), (i + 1, j)] {
                if i1 < 0 || j1 < 0 || i == m || j == n {
                    continue;
                }
                let c = self.grid[i1 as usize][j1 as usize];
                if c == enemy {
                    match d.cmp(&min) {
                        std::cmp::Ordering::Less => {
                            candidates.clear();
                            min = d;
                            candidates.push(second.unwrap());
                        }
                        std::cmp::Ordering::Equal => candidates.push(second.unwrap()),
                        std::cmp::Ordering::Greater => {
                            q.clear();
                            break;
                        }
                    }
                }
                if visited.contains(&(i, j)) {
                    continue;
                }
                if self.grid[i1 as usize][j1 as usize] == b'.' {
                    let second = match second {
                        Some(x) => Some(x),
                        None => Some((i1, j1)),
                    };
                    q.push_back((d + 1, second, (i1, j1)));
                }
            }
            visited.insert((i, j));
        }
        candidates.sort_unstable();
        candidates.first().cloned()
    }

    fn move_to(&mut self, (i, j): (i8, i8), (i1, j1): (i8, i8)) {
        let unit = self.units.remove(&(i, j)).unwrap();
        self.units.insert((i1, j1), unit);
        let c = self.grid[i as usize][j as usize];
        self.grid[i as usize][j as usize] = b'.';
        self.grid[i1 as usize][j1 as usize] = c;
    }

    fn attack(&mut self, (i_t, j_t): (i8, i8)) {
        let u = self.units.get_mut(&(i_t, j_t)).unwrap();
        let attack = match u.kind {
            Kind::Elf => 3,
            Kind::Goblin => self.elv_attack,
        };
        if u.hp <= attack {
            if u.kind == Kind::Elf {
                self.elv_died = true;
            }
            self.units.remove(&(i_t, j_t));
            self.grid[i_t as usize][j_t as usize] = b'.';
        } else {
            u.hp -= attack;
        }
    }

    fn play_round(&mut self) -> bool {
        let mut order = self.units.keys().cloned().collect::<Vec<_>>();
        if order.is_empty() {
            return false;
        }
        order.sort_unstable();
        for (i, j) in order {
            // check if the unit is still alive
            if let Some(u) = self.units.get(&(i, j)) {
                if !self.has_enemy(u.kind.enemy()) {
                    return false;
                }
            } else {
                continue;
            }
            if let Some((i_t, j_t)) = self.find_enemy_in_range((i, j)) {
                self.attack((i_t, j_t));
            } else if let Some((i1, j1)) = self.find_path((i, j)) {
                self.move_to((i, j), (i1, j1));
                if let Some((i_t, j_t)) = self.find_enemy_in_range((i1, j1)) {
                    self.attack((i_t, j_t));
                }
            }
        }
        self.turns += 1;
        true
    }

    fn combat(&mut self) -> bool {
        while self.play_round() {
            if self.elv_died {
                return false;
            }
        }
        true
    }

    fn outcome(&self) -> usize {
        self.turns * self.units.values().map(|u| u.hp as usize).sum::<usize>()
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, r) in self.grid.iter().enumerate() {
            let row = unsafe { std::str::from_utf8_unchecked(r) };
            let mut units = String::new();
            for (j, &c) in r.iter().enumerate() {
                if c == b'G' || c == b'E' {
                    let u = self.units.get(&(i as i8, j as i8)).unwrap();
                    units.push_str(&format!("{}({}) ", c as char, u.hp));
                }
            }
            writeln!(f, "{row} {units}")?;
        }
        Ok(())
    }
}

fn parse(data: &str) -> Map {
    let grid = data
        .trim()
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<_>>();
    let units = grid
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter().enumerate().filter_map(move |(j, c)| match c {
                b'G' | b'E' => {
                    let kind = Kind::from(*c);
                    Some(((i as i8, j as i8), Unit { kind, hp: 200 }))
                }
                _ => None,
            })
        })
        .collect();
    Map {
        grid,
        units,
        turns: 0,
        elv_attack: 3,
        elv_died: false,
    }
}

fn part1(mut map: Map) -> usize {
    while map.play_round() {}
    map.outcome()
}

fn part2(map: Map) -> usize {
    let mut left = 4;
    let mut right = 200;
    let mut result = 0;
    while left < right {
        let mut m = map.clone();
        m.elv_attack = left + (right - left) / 2;
        if m.combat() {
            right = m.elv_attack;
            result = m.outcome();
        } else {
            left = m.elv_attack + 1;
        }
    }
    result
}

pub fn main() {
    let data = std::fs::read_to_string("data/2018/day15").unwrap();
    let map = parse(&data);
    println!("part1: {}", part1(map.clone()));
    println!("part2: {}", part2(map));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_path() {
        let data = "
#######
#.E...#
#.....#
#...G.#
#######"
            .to_string();
        let map = parse(&data);
        assert_eq!(Some((1, 3)), map.find_path((1, 2)));
    }

    #[test]
    fn test_movement() {
        let data = "
#########
#G..G..G#
#.......#
#.......#
#G..E..G#
#.......#
#.......#
#G..G..G#
#########"
            .to_string();
        let mut map = parse(&data);
        for _ in 0..3 {
            map.play_round();
        }
        let data = "
#########
#.......#
#..GGG..#
#..GEG..#
#G..G...#
#......G#
#.......#
#.......#
#########"
            .to_string();
        let expected = parse(&data);
        assert_eq!(expected.grid, map.grid);
    }

    #[test]
    fn case1() {
        let data = "
#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######"
            .to_string();
        let map = parse(&data);
        assert_eq!(27730, part1(map.clone()));
        assert_eq!(4988, part2(map));
    }

    #[test]
    fn case2() {
        let data = "
#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######"
            .to_string();
        let map = parse(&data);
        assert_eq!(36334, part1(map));
    }

    #[test]
    fn case3() {
        let data = "
#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######"
            .to_string();
        let mut map = parse(&data);
        while map.play_round() {}
        assert_eq!(39514, map.outcome());
    }

    #[test]
    fn case4() {
        let data = "
#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######"
            .to_string();
        let map = parse(&data);
        assert_eq!(27755, part1(map.clone()));
        assert_eq!(3478, part2(map));
    }

    #[test]
    fn case5() {
        let data = "
#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######"
            .to_string();
        let map = parse(&data);
        assert_eq!(28944, part1(map.clone()));
        assert_eq!(6474, part2(map));
    }

    #[test]
    fn case6() {
        let data = "
#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########"
            .to_string();
        let map = parse(&data);
        assert_eq!(18740, part1(map.clone()));
        assert_eq!(1140, part2(map));
    }
}
