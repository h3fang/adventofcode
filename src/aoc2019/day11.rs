use ahash::AHashMap as HashMap;

use crate::day5::Intcode;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn left(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    fn right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    fn delta(&self) -> (i64, i64) {
        match self {
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

struct Robot {
    x: i64,
    y: i64,
    direction: Direction,
}

impl Robot {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            direction: Direction::Up,
        }
    }

    fn action(&mut self, turn: i64) {
        if turn == 1 {
            // turn left
            self.direction = self.direction.left();
        } else {
            // turn right
            self.direction = self.direction.right();
        }
        let (dx, dy) = self.direction.delta();
        self.x += dx;
        self.y += dy;
    }
}

fn part1(codes: &[i64]) -> usize {
    let mut map: HashMap<(i64, i64), i64> = HashMap::new();
    let mut prog = Intcode::new(codes);
    let mut robot = Robot::new();
    while !prog.is_halted() {
        let current = *map.get(&(robot.x, robot.y)).unwrap_or(&0);
        prog.inputs = vec![current];

        // paint
        prog.run();
        let color = prog.output;
        map.insert((robot.x, robot.y), color);

        // turn
        prog.run();
        robot.action(prog.output);
    }
    map.len()
}

fn bounds(map: &HashMap<(i64, i64), i64>) -> (i64, i64, i64, i64) {
    let mut min_x = i64::MAX;
    let mut min_y = i64::MAX;

    let mut max_x = i64::MIN;
    let mut max_y = i64::MIN;

    for (k, v) in map {
        if *v == 1 {
            min_x = min_x.min(k.0);
            min_y = min_y.min(k.1);

            max_x = max_x.max(k.0);
            max_y = max_y.max(k.1);
        }
    }

    (min_x, min_y, max_x, max_y)
}

fn paint(map: &HashMap<(i64, i64), i64>) {
    let bounds = bounds(map);

    let width = (bounds.2 - bounds.0 + 1) as usize;
    let height = (bounds.3 - bounds.1 + 1) as usize;
    let mut img = vec![b' '; width * height];
    for (&(x, y), &v) in map {
        if v == 1 {
            let x = (bounds.2 - x) as usize;
            let y = (bounds.3 - y) as usize;
            img[y * width + x] = b'#';
        }
    }

    img.chunks(width).for_each(|row| {
        println!("{}", unsafe { std::str::from_utf8_unchecked(row) });
    })
}

fn part2(codes: &[i64]) {
    let mut map: HashMap<(i64, i64), i64> = HashMap::new();
    map.insert((0, 0), 1);
    let mut prog = Intcode::new(codes);
    let mut robot = Robot::new();
    while !prog.is_halted() {
        let current = *map.get(&(robot.x, robot.y)).unwrap_or(&0);
        prog.inputs = vec![current];

        // paint
        prog.run();
        let color = prog.output;
        map.insert((robot.x, robot.y), color);

        // turn
        prog.run();
        robot.action(prog.output);
    }

    paint(&map);
}

pub fn main() {
    let codes = std::fs::read_to_string("data/2019/day11")
        .unwrap()
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|t| t.trim().parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    println!("day11 part1: {}", part1(&codes));
    println!("day11 part2:");
    part2(&codes);
}
