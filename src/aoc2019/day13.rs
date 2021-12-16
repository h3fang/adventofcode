use ahash::AHashMap as HashMap;

use crate::day5::Intcode;

#[derive(PartialEq, Clone, Copy)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl From<i64> for Tile {
    fn from(n: i64) -> Self {
        match n {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::Paddle,
            4 => Tile::Ball,
            x => panic!("invalid tile: {}", x),
        }
    }
}

impl From<&Tile> for u8 {
    fn from(t: &Tile) -> Self {
        match t {
            Tile::Empty => b' ',
            Tile::Wall => b'X',
            Tile::Block => b'#',
            Tile::Paddle => b'-',
            Tile::Ball => b'O',
        }
    }
}

struct Game {
    width: usize,
    grid: Vec<Tile>,
    ball: (i64, i64),
    paddle: (i64, i64),
    prog: Intcode,
    score: i64,
}

impl Game {
    fn step(&mut self) {
        loop {
            self.prog.run();
            let x = self.prog.output;
            self.prog.run();
            let y = self.prog.output;
            self.prog.run();
            let z = self.prog.output;

            if x == -1 && y == 0 {
                self.score = z;
                break;
            } else {
                match Tile::from(z) {
                    Tile::Paddle => self.paddle = (x, y),
                    Tile::Ball => self.ball = (x, y),
                    _ => {}
                }
                self.grid[x as usize + y as usize * self.width] = Tile::from(z);
            }
            self.prog
                .inputs
                .push((self.ball.0 - self.paddle.0).signum());
        }
    }

    fn is_gameover(&self) -> bool {
        self.grid.iter().filter(|t| **t == Tile::Block).count() == 0
    }
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.grid.chunks(self.width) {
            let row = row.iter().map(|t| t.into()).collect::<Vec<u8>>();
            writeln!(f, "{}", unsafe { std::str::from_utf8_unchecked(&row) })?;
        }
        Ok(())
    }
}

fn part1(codes: &[i64]) -> HashMap<(i64, i64), Tile> {
    let mut map: HashMap<(i64, i64), Tile> = HashMap::new();
    let mut prog = Intcode::new(codes);
    while !prog.is_halted() {
        prog.run();
        let x = prog.output;
        prog.run();
        let y = prog.output;
        prog.run();
        let tile = prog.output;
        map.insert((x, y), Tile::from(tile));
    }
    map
}

fn build(map: &HashMap<(i64, i64), Tile>, codes: &[i64]) -> Game {
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

    assert_eq!((0, 0), (min_x, min_y));

    let height = (max_y - min_y + 1) as usize;
    let width = (max_x - min_x + 1) as usize;
    let mut grid = vec![Tile::Empty; width * height];
    let mut ball = (0, 0);
    let mut paddle = (0, 0);

    let index = |x: i64, y: i64| y as usize * width + x as usize;

    for (&(x, y), &v) in map {
        grid[index(x, y)] = v;
        match v {
            Tile::Paddle => {
                paddle = (x, y);
            }
            Tile::Ball => {
                ball = (x, y);
            }
            _ => {}
        }
    }

    let prog = Intcode::new(codes);

    Game {
        width,
        grid,
        paddle,
        ball,
        prog,
        score: 0,
    }
}

fn part2(codes: &[i64], map: HashMap<(i64, i64), Tile>) -> i64 {
    let mut game = build(&map, codes);
    // println!("{}", game);
    while !game.is_gameover() {
        game.step();
        // println!("{}", game);
    }
    game.score
}

pub fn main() {
    let mut codes = std::fs::read_to_string("data/2019/day13")
        .unwrap()
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|t| t.trim().parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let map = part1(&codes);
    let blocks = map.values().filter(|t| **t == Tile::Block).count();
    println!("day13 part1: {}", blocks);
    codes[0] = 2;
    println!("day13 part2: {}", part2(&codes, map));
}
