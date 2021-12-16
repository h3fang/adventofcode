use ahash::AHashMap as HashMap;
use ahash::AHashSet as HashSet;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    fn displacement(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
            Direction::Right => (1, 0),
            Direction::Left => (-1, 0),
        }
    }
}

#[derive(Debug)]
struct Segment {
    dir: Direction,
    dist: isize,
}

impl From<&str> for Segment {
    fn from(s: &str) -> Self {
        let dir = match s.as_bytes()[0] {
            b'R' => Direction::Right,
            b'L' => Direction::Left,
            b'U' => Direction::Up,
            b'D' => Direction::Down,
            _ => panic!("invalid direction"),
        };
        let dist = s[1..].parse().unwrap();
        Self { dir, dist }
    }
}

fn part1(wires: &[Vec<Segment>]) -> i32 {
    let mut tip = (0, 0);
    let mut w1 = HashSet::new();
    for seg in &wires[0] {
        let ds = seg.dir.displacement();
        for _ in 1..=seg.dist {
            tip.0 += ds.0;
            tip.1 += ds.1;
            w1.insert(tip);
        }
    }

    let mut result = i32::MAX;

    tip = (0, 0);
    for seg in &wires[1] {
        let ds = seg.dir.displacement();
        for _ in 1..=seg.dist {
            tip.0 += ds.0;
            tip.1 += ds.1;
            if w1.contains(&tip) {
                result = result.min(tip.0.abs() + tip.1.abs());
            }
        }
    }
    result
}

fn part2(wires: &[Vec<Segment>]) -> i32 {
    let mut tip = (0, 0);
    let mut steps = 0;
    let mut w1: HashMap<(i32, i32), i32> = HashMap::new();
    for seg in &wires[0] {
        let ds = seg.dir.displacement();
        for _ in 1..=seg.dist {
            tip.0 += ds.0;
            tip.1 += ds.1;
            steps += 1;
            w1.entry(tip).or_insert(steps);
        }
    }

    let mut result = i32::MAX;

    tip = (0, 0);
    steps = 0;
    for seg in &wires[1] {
        let ds = seg.dir.displacement();
        for _ in 1..=seg.dist {
            tip.0 += ds.0;
            tip.1 += ds.1;
            steps += 1;
            if let Some(s) = w1.get(&tip) {
                result = result.min(s + steps);
            }
        }
    }
    result
}

pub fn main() {
    let wires = std::fs::read_to_string("data/2019/day3")
        .unwrap()
        .lines()
        .map(|s| s.split(',').map(|s| s.into()).collect::<Vec<Segment>>())
        .collect::<Vec<_>>();

    println!("day3 part1: {}", part1(&wires));

    println!("day3 part2: {}", part2(&wires));
}
