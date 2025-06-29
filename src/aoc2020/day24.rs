use ahash::AHashMap as HashMap;

const NEIGHBORS: [(i32, i32); 6] = [(1, 0), (-1, 0), (0, -1), (1, -1), (-1, 1), (0, 1)];

#[derive(Debug)]
enum Direction {
    SW,
    SE,
    E,
    NE,
    NW,
    W,
}

#[derive(Debug, Clone, PartialEq)]
enum Face {
    Black,
    White,
}

impl Face {
    fn flip(&self) -> Self {
        match self {
            Face::Black => Face::White,
            Face::White => Face::Black,
        }
    }
}

fn parse(content: &str) -> Vec<Vec<Direction>> {
    content
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut chars = line.chars();
            let mut dirs = Vec::new();
            while let Some(c) = chars.next() {
                dirs.push(match c {
                    's' => match chars.next().unwrap() {
                        'w' => Direction::SW,
                        'e' => Direction::SE,
                        x => panic!("invalid direction symbol after s: {x}"),
                    },
                    'n' => match chars.next().unwrap() {
                        'w' => Direction::NW,
                        'e' => Direction::NE,
                        x => panic!("invalid direction symbol after n: {x}"),
                    },
                    'e' => Direction::E,
                    'w' => Direction::W,
                    x => panic!("invalid direction symbol: {x}"),
                });
            }
            dirs
        })
        .collect()
}

fn part1(directions: &[Vec<Direction>]) -> (usize, HashMap<(i32, i32), Face>) {
    let mut tiles: HashMap<(i32, i32), Face> = HashMap::new();
    directions.iter().for_each(|dirs| {
        let (mut x, mut y) = (0, 0);
        dirs.iter().for_each(|d| match d {
            Direction::E => x += 1,
            Direction::W => x -= 1,
            Direction::SW => {
                x -= 1;
                y += 1
            }
            Direction::SE => y += 1,
            Direction::NW => y -= 1,
            Direction::NE => {
                x += 1;
                y -= 1;
            }
        });
        let e = tiles.entry((x, y)).or_insert(Face::White);
        *e = e.flip();
    });
    let n_black = tiles.values().filter(|t| **t == Face::Black).count();
    (n_black, tiles)
}

fn part2(tiles: &mut HashMap<(i32, i32), Face>) -> usize {
    for _ in 0..100 {
        for ((x, y), face) in tiles.clone() {
            if face == Face::Black {
                NEIGHBORS.iter().for_each(|(dx, dy)| {
                    let n = (x + dx, y + dy);
                    tiles.entry(n).or_insert(Face::White);
                })
            }
        }

        let old = tiles.to_owned();
        for ((x, y), face) in &old {
            match &face {
                Face::White => {
                    let n_black = NEIGHBORS
                        .iter()
                        .filter(|(dx, dy)| {
                            let n = (x + dx, y + dy);
                            old.get(&n) == Some(&Face::Black)
                        })
                        .count();
                    if n_black == 2 {
                        tiles.insert((*x, *y), Face::Black);
                    }
                }
                Face::Black => {
                    let n_black = NEIGHBORS
                        .iter()
                        .filter(|(dx, dy)| {
                            let n = (x + dx, y + dy);
                            old.get(&n) == Some(&Face::Black)
                        })
                        .count();
                    if n_black == 0 || n_black > 2 {
                        tiles.insert((*x, *y), Face::White);
                    }
                }
            }
        }
    }

    tiles.values().filter(|t| **t == Face::Black).count()
}

pub fn main() {
    let content = std::fs::read_to_string("data/2020/day24").unwrap();
    let directions = parse(&content);

    // part 1
    let (n_black, mut tiles) = part1(&directions);
    println!("day 24 part1: {n_black}");

    // part 2
    println!("day 24 part2: {}", part2(&mut tiles));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input1() {
        let content = "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";

        let directions = parse(content);

        // part 1
        let (n_black, mut tiles) = part1(&directions);
        assert_eq!(10, n_black);

        // part 2
        assert_eq!(2208, part2(&mut tiles));
    }
}
