use ahash::AHashMap as HashMap;
use ahash::AHashSet as HashSet;
use std::collections::{BinaryHeap, VecDeque};

use crate::day5::Intcode;

#[derive(PartialEq, Clone, Copy)]
enum Tile {
    Empty,
    Wall,
    Tank,
}

impl From<i64> for Tile {
    fn from(n: i64) -> Self {
        match n {
            0 => Tile::Wall,
            1 => Tile::Empty,
            2 => Tile::Tank,
            x => panic!("invalid tile: {}", x),
        }
    }
}

impl From<&Tile> for u8 {
    fn from(t: &Tile) -> Self {
        match t {
            Tile::Empty => b' ',
            Tile::Wall => b'+',
            Tile::Tank => b'O',
        }
    }
}

struct Map {
    width: usize,
    height: usize,
    grid: Vec<Tile>,
    robot: (i64, i64),
    tank: (i64, i64),
}

impl Map {
    fn explore(codes: &[i64]) -> HashMap<(i64, i64), Tile> {
        fn dfs(prog: &mut Intcode, map: &mut HashMap<(i64, i64), Tile>, (x, y): (i64, i64)) {
            for (dir, cmd) in [((0, -1), 1), ((0, 1), 2), ((-1, 0), 3), ((1, 0), 4)] {
                let next = (x + dir.0, y + dir.1);
                if map.contains_key(&next) {
                    continue;
                }
                prog.inputs.push_back(cmd);
                prog.run();
                let t = Tile::from(prog.outputs.pop_front().unwrap());
                map.insert(next, t);
                if t != Tile::Wall {
                    dfs(prog, map, next);
                    prog.inputs.push_back(match cmd {
                        1 => 2,
                        2 => 1,
                        3 => 4,
                        4 => 3,
                        _ => panic!("impossible"),
                    });
                    prog.run();
                    prog.outputs.pop_front();
                }
            }
        }

        let mut prog = Intcode::new(codes);
        let mut map = HashMap::new();
        map.insert((0, 0), Tile::Empty);
        dfs(&mut prog, &mut map, (0, 0));
        map
    }

    fn build(codes: &[i64]) -> Self {
        let map = Self::explore(codes);
        let mut min_x = i64::MAX;
        let mut min_y = i64::MAX;
        let mut max_x = i64::MIN;
        let mut max_y = i64::MIN;

        for &k in map.keys() {
            min_x = min_x.min(k.0);
            min_y = min_y.min(k.1);

            max_x = max_x.max(k.0);
            max_y = max_y.max(k.1);
        }

        let height = (max_y - min_y + 1) as usize;
        let width = (max_x - min_x + 1) as usize;
        let mut grid = vec![Tile::Empty; width * height];
        let robot = (-min_x, -min_y);
        let mut tank = (0, 0);

        let index = |x: i64, y: i64| (y - min_y) as usize * width + (x - min_x) as usize;

        for ((x, y), v) in map {
            grid[index(x, y)] = v;
            if v == Tile::Tank {
                tank = (x - min_x, y - min_y);
            }
        }

        Self {
            width,
            height,
            grid,
            robot,
            tank,
        }
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.grid.chunks(self.width) {
            let row = row.iter().map(|t| t.into()).collect::<Vec<u8>>();
            writeln!(f, "{}", unsafe { std::str::from_utf8_unchecked(&row) })?;
        }
        Ok(())
    }
}

fn part1(map: &Map) -> i64 {
    let mut q = BinaryHeap::new();
    let mut closed = HashSet::new();
    q.push((0i64, map.robot));
    while let Some((dist, (x, y))) = q.pop() {
        let dist = -dist;
        if (x, y) == map.tank {
            return dist;
        }
        closed.insert((x, y));

        for (dx, dy) in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
            let (x_n, y_n) = (x + dx, y + dy);
            if x_n < 0
                || y_n < 0
                || x_n >= map.width as i64
                || y_n >= map.height as i64
                || map.grid[y_n as usize * map.width + x_n as usize] == Tile::Wall
                || closed.contains(&(x_n, y_n))
            {
                continue;
            }
            q.push((-(dist + 1), (x_n, y_n)));
        }
    }
    -1
}

fn part2(map: &Map) -> i64 {
    let mut q = VecDeque::new();
    let mut closed = HashSet::new();
    let mut result = 0;
    q.push_back(map.tank);
    closed.insert(map.tank);
    while !q.is_empty() {
        let n = q.len();
        for _ in 0..n {
            let (x, y) = q.pop_front().unwrap();
            for (dx, dy) in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
                let (x_n, y_n) = (x + dx, y + dy);
                if x_n < 0
                    || y_n < 0
                    || x_n >= map.width as i64
                    || y_n >= map.height as i64
                    || map.grid[y_n as usize * map.width + x_n as usize] == Tile::Wall
                    || closed.contains(&(x_n, y_n))
                {
                    continue;
                }
                q.push_back((x_n, y_n));
                closed.insert((x_n, y_n));
            }
        }
        result += 1;
    }
    result - 1
}

pub fn main() {
    let codes = std::fs::read_to_string("data/2019/day15")
        .unwrap()
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|t| t.trim().parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let map = Map::build(&codes);
    // println!("{}", map);
    println!("day15 part1: {}", part1(&map));
    println!("day15 part2: {}", part2(&map));
}
