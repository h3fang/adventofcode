use ahash::HashMap;
use arrayvec::ArrayVec;

macro_rules! avec {
    ($($x:expr),+ $(,)?) => {
        {
            let mut a = ArrayVec::new();
            $(
                a.push($x);
            )*
            a
        }
    };
}

fn parse(data: &str) -> &[u8] {
    data.trim().as_bytes()
}

fn rock_height(rock: u8) -> usize {
    match rock {
        0 => 1,
        1 => 3,
        2 => 3,
        3 => 4,
        4 => 2,
        _ => unreachable!(),
    }
}

fn rock_points(rock: u8, (i, j): (usize, usize)) -> ArrayVec<(usize, usize), 5> {
    match rock {
        0 => avec!((i, j), (i, j + 1), (i, j + 2), (i, j + 3)),
        1 => avec!(
            (i + 2, j + 1),
            (i + 1, j),
            (i + 1, j + 1),
            (i + 1, j + 2),
            (i, j + 1),
        ),
        2 => avec!(
            (i + 2, j + 2),
            (i + 1, j + 2),
            (i, j),
            (i, j + 1),
            (i, j + 2),
        ),
        3 => avec!((i + 3, j), (i + 2, j), (i + 1, j), (i, j)),
        4 => avec!((i + 1, j), (i + 1, j + 1), (i, j), (i, j + 1)),
        _ => unreachable!(),
    }
}

fn rock_left(rock: u8, (i, j): (usize, usize)) -> ArrayVec<(usize, usize), 4> {
    match rock {
        0 => avec!((i, j)),
        1 => avec!((i + 2, j + 1), (i + 1, j), (i, j + 1)),
        2 => avec!((i + 2, j + 2), (i + 1, j + 2), (i, j)),
        3 => avec!((i + 3, j), (i + 2, j), (i + 1, j), (i, j)),
        4 => avec!((i + 1, j), (i, j)),
        _ => unreachable!(),
    }
}

fn rock_right(rock: u8, (i, j): (usize, usize)) -> ArrayVec<(usize, usize), 4> {
    match rock {
        0 => avec!((i, j + 3)),
        1 => avec!((i + 2, j + 1), (i + 1, j + 2), (i, j + 1)),
        2 => avec!((i + 2, j + 2), (i + 1, j + 2), (i, j + 2)),
        3 => avec!((i + 3, j), (i + 2, j), (i + 1, j), (i, j)),
        4 => avec!((i + 1, j + 1), (i, j + 1)),
        _ => unreachable!(),
    }
}

fn rock_bottom(rock: u8, (i, j): (usize, usize)) -> ArrayVec<(usize, usize), 4> {
    match rock {
        0 => avec!((i, j), (i, j + 1), (i, j + 2), (i, j + 3)),
        1 => avec!((i + 1, j), (i, j + 1), (i + 1, j + 2)),
        2 => avec!((i, j), (i, j + 1), (i, j + 2)),
        3 => avec!((i, j)),
        4 => avec!((i, j), (i, j + 1)),
        _ => unreachable!(),
    }
}

struct Tower {
    chamber: Vec<[u8; 7]>,
    jets: Vec<u8>,
    next_rock: u8,
    next_jet: usize,
}

impl Tower {
    fn new(jets: &[u8]) -> Self {
        Self {
            chamber: Vec::with_capacity(4096),
            jets: jets.to_vec(),
            next_rock: 0,
            next_jet: 0,
        }
    }

    fn spawn(&mut self) -> (u8, (usize, usize)) {
        let rock = self.next_rock;
        let pos = (self.chamber.len() + 3, 2);
        self.chamber.extend(vec![
            [b'.', b'.', b'.', b'.', b'.', b'.', b'.'];
            3 + rock_height(rock)
        ]);
        self.next_rock = (self.next_rock + 1) % 5;
        (rock, pos)
    }

    fn move_horizontal(&mut self, rock: u8, pos: (usize, usize)) -> (usize, usize) {
        let jet = self.jets[self.next_jet];
        self.next_jet = (self.next_jet + 1) % self.jets.len();
        match jet {
            b'<' => {
                if rock_left(rock, pos)
                    .iter()
                    .any(|&(i, j)| j == 0 || self.chamber[i][j - 1] == b'#')
                {
                    pos
                } else {
                    (pos.0, pos.1 - 1)
                }
            }
            b'>' => {
                if rock_right(rock, pos)
                    .iter()
                    .any(|&(i, j)| j == 6 || self.chamber[i][j + 1] == b'#')
                {
                    pos
                } else {
                    (pos.0, pos.1 + 1)
                }
            }
            _ => unreachable!(),
        }
    }

    fn clear_top(&mut self) {
        while let Some(t) = self.chamber.last() {
            if t != &[b'.', b'.', b'.', b'.', b'.', b'.', b'.'] {
                break;
            }
            self.chamber.pop();
        }
    }

    fn move_down(&mut self, rock: u8, pos: (usize, usize)) -> (usize, usize) {
        if rock_bottom(rock, pos)
            .iter()
            .any(|&(i, j)| i == 0 || self.chamber[i - 1][j] == b'#')
        {
            pos
        } else {
            (pos.0 - 1, pos.1)
        }
    }

    fn fall(&mut self) {
        let (rock, mut pos) = self.spawn();
        loop {
            pos = self.move_horizontal(rock, pos);
            let p = self.move_down(rock, pos);
            if p == pos {
                break;
            }
            pos = p;
        }
        for (i, j) in rock_points(rock, pos) {
            self.chamber[i][j] = b'#';
        }
        self.clear_top();
    }
}

fn part1(tower: &mut Tower) -> usize {
    for _ in 0..2022 {
        tower.fall();
    }
    tower.chamber.len()
}

fn part2(tower: &mut Tower) -> usize {
    const N: usize = 1000000000000;
    // height of the top of the tower for hash
    const M: usize = 100;
    let mut seen = HashMap::default();
    let c = &tower.chamber;
    seen.insert(
        (c[c.len() - M..].to_vec(), tower.next_rock, tower.next_jet),
        (2022, c.len()),
    );
    for i in 2023.. {
        tower.fall();
        let c = &tower.chamber;
        if let Some((i1, h1)) = seen.insert(
            (c[c.len() - M..].to_vec(), tower.next_rock, tower.next_jet),
            (i, c.len()),
        ) {
            let period = i - i1;
            let delta_h = c.len() - h1;
            let rem = (N - i) % period;
            let dh = seen.values().find(|&&(k, _)| k == i1 + rem).unwrap().1 - h1;
            return c.len() + ((N - i) / period) * delta_h + dh;
        }
    }
    unreachable!()
}

pub fn main() {
    let data = std::fs::read_to_string("data/2022/day17").unwrap();
    let jets = parse(&data);
    let mut tower = Tower::new(jets);
    println!("part1: {}", part1(&mut tower));
    println!("part2: {}", part2(&mut tower));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
        let jets = parse(&data);
        let mut tower = Tower::new(jets);
        assert_eq!(3068, part1(&mut tower));
        assert_eq!(1514285714288, part2(&mut tower));
    }
}
