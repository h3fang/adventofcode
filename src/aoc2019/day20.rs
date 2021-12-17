use ahash::AHashMap as HashMap;
use arrayvec::ArrayVec;
use std::{cmp::Reverse, collections::BinaryHeap};

type Position = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Ring {
    Inner,
    Outer,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Key {
    name: [u8; 2],
    ring: Ring,
}

impl std::fmt::Debug for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} {}{}",
            self.ring, self.name[0] as char, self.name[1] as char
        )
    }
}

struct Maze {
    grid: Vec<Vec<u8>>,
    portal_keys: HashMap<Position, Key>,
    portal_pos: HashMap<Key, Position>,
    paths: HashMap<Key, Vec<(Key, usize)>>,
}

impl Maze {
    fn from_lines(data: &str) -> Self {
        let grid = data
            .lines()
            .map(|line| line.as_bytes().to_vec())
            .collect::<Vec<_>>();
        let height = grid.len();
        let width = grid[0].len();

        let mut maze = Self {
            grid,
            portal_keys: HashMap::new(),
            portal_pos: HashMap::new(),
            paths: HashMap::new(),
        };

        for y in 1..height - 1 {
            for x in 1..width - 1 {
                if maze.grid[y][x] == b'.' {
                    for (i, (xn, yn)) in [(x - 1, y), (x, y - 1), (x + 1, y), (x, y + 1)]
                        .into_iter()
                        .enumerate()
                    {
                        let a = maze.grid[yn][xn];
                        if a.is_ascii_uppercase() {
                            let xt = xn + xn - x;
                            let yt = yn + yn - y;
                            let b = maze.grid[yt][xt];
                            let name = if i > 1 { [a, b] } else { [b, a] };
                            let key = Key {
                                name,
                                ring: if maze.is_outer(x, y) {
                                    Ring::Outer
                                } else {
                                    Ring::Inner
                                },
                            };
                            maze.portal_keys.insert((x, y), key);
                            maze.portal_pos.insert(key, (x, y));
                        }
                    }
                }
            }
        }

        for (&key, &pos) in &maze.portal_pos {
            maze.paths.insert(key, all_paths(&maze, pos));
        }

        maze
    }

    fn is_outer(&self, x: usize, y: usize) -> bool {
        x <= 2 || y <= 2 || x >= self.grid[0].len() - 3 || y >= self.grid.len() - 3
    }

    fn neighbors(&self, x: usize, y: usize) -> ArrayVec<Position, 4> {
        let mut result = ArrayVec::new();
        for p in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
            let b = self.grid[p.1][p.0];
            if b == b'.' {
                result.push(p);
            }
        }
        result
    }
}

fn all_paths(maze: &Maze, start: Position) -> Vec<(Key, usize)> {
    let mut result = vec![];
    let mut q = BinaryHeap::new();
    let mut costs = HashMap::new();
    q.push((Reverse(0), start));
    costs.insert(start, 0);
    while let Some((c, pos)) = q.pop() {
        if pos != start && maze.portal_keys.contains_key(&pos) {
            result.push((*maze.portal_keys.get(&pos).unwrap(), c.0));
            continue;
        }

        if let Some(&cost) = costs.get(&pos) {
            if c.0 > cost {
                continue;
            }
        }

        for next in maze.neighbors(pos.0, pos.1) {
            let curr = costs.entry(next).or_insert(usize::MAX);
            if c.0 + 1 < *curr {
                *curr = c.0 + 1;
                q.push((Reverse(c.0 + 1), next));
            }
        }
    }
    result
}

fn part1(maze: &Maze) -> usize {
    let mut costs = HashMap::new();
    let mut q = BinaryHeap::new();
    let start = Key {
        name: [b'A'; 2],
        ring: Ring::Outer,
    };
    q.push((Reverse(0), start));
    costs.insert(start, 0usize);
    let end = Key {
        name: [b'Z'; 2],
        ring: Ring::Outer,
    };
    while let Some((c, key)) = q.pop() {
        if key == end {
            return c.0;
        }
        if let Some(&cost) = costs.get(&key) {
            if c.0 > cost {
                continue;
            }
        }
        for &(next, dist) in maze.paths.get(&key).unwrap() {
            let mut cn = c.0 + dist;

            let other_side = if next != start && next != end {
                cn += 1;
                Key {
                    name: next.name,
                    ring: match next.ring {
                        Ring::Inner => Ring::Outer,
                        Ring::Outer => Ring::Inner,
                    },
                }
            } else {
                next
            };
            let curr = costs.entry(other_side).or_insert(usize::MAX);
            if cn < *curr {
                *curr = cn;
                q.push((Reverse(cn), other_side));
            }
        }
    }
    unreachable!()
}

fn part2(maze: &Maze) -> usize {
    let mut costs = HashMap::new();
    let mut q = BinaryHeap::new();
    let start = Key {
        name: [b'A'; 2],
        ring: Ring::Outer,
    };
    q.push((Reverse(0), start, 0u32));
    costs.insert((start, 0u32), 0);
    let end = Key {
        name: [b'Z'; 2],
        ring: Ring::Outer,
    };
    while let Some((c, key, level)) = q.pop() {
        if (key, level) == (end, 0) {
            return c.0;
        }
        if let Some(&cost) = costs.get(&(key, level)) {
            if c.0 > cost {
                continue;
            }
        }
        for &(next, dist) in maze.paths.get(&key).unwrap() {
            if next == start || (level == 0 && next.ring == Ring::Outer && next.name != end.name) {
                continue;
            }

            let mut cn = c.0 + dist;
            let mut level = level;

            let other_side = if next != end {
                cn += 1;
                Key {
                    name: next.name,
                    ring: match next.ring {
                        Ring::Inner => {
                            level += 1;
                            Ring::Outer
                        }
                        Ring::Outer => {
                            level -= 1;
                            Ring::Inner
                        }
                    },
                }
            } else {
                next
            };
            let curr = costs.entry((other_side, level)).or_insert(usize::MAX);
            if cn < *curr {
                *curr = cn;
                q.push((Reverse(cn), other_side, level));
            }
        }
    }
    unreachable!()
}

pub fn main() {
    let data = std::fs::read_to_string("data/2019/day20").unwrap();
    let maze = Maze::from_lines(&data);

    println!("day 20 part1: {}", part1(&maze));
    println!("day 20 part2: {}", part2(&maze));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "         A           
         A           
  #######.#########  
  #######.........#  
  #######.#######.#  
  #######.#######.#  
  #######.#######.#  
  #####  B    ###.#  
BC...##  C    ###.#  
  ##.##       ###.#  
  ##...DE  F  ###.#  
  #####    G  ###.#  
  #########.#####.#  
DE..#######...###.#  
  #.#########.###.#  
FG..#########.....#  
  ###########.#####  
             Z       
             Z       ";
        let maze = Maze::from_lines(&data);
        assert_eq!(23, part1(&maze));
        assert_eq!(26, part2(&maze));
    }

    #[test]
    fn case2() {
        let data = "                   A               
                   A               
  #################.#############  
  #.#...#...................#.#.#  
  #.#.#.###.###.###.#########.#.#  
  #.#.#.......#...#.....#.#.#...#  
  #.#########.###.#####.#.#.###.#  
  #.............#.#.....#.......#  
  ###.###########.###.#####.#.#.#  
  #.....#        A   C    #.#.#.#  
  #######        S   P    #####.#  
  #.#...#                 #......VT
  #.#.#.#                 #.#####  
  #...#.#               YN....#.#  
  #.###.#                 #####.#  
DI....#.#                 #.....#  
  #####.#                 #.###.#  
ZZ......#               QG....#..AS
  ###.###                 #######  
JO..#.#.#                 #.....#  
  #.#.#.#                 ###.#.#  
  #...#..DI             BU....#..LF
  #####.#                 #.#####  
YN......#               VT..#....QG
  #.###.#                 #.###.#  
  #.#...#                 #.....#  
  ###.###    J L     J    #.#.###  
  #.....#    O F     P    #.#...#  
  #.###.#####.#.#####.#####.###.#  
  #...#.#.#...#.....#.....#.#...#  
  #.#####.###.###.#.#.#########.#  
  #...#.#.....#...#.#.#.#.....#.#  
  #.###.#####.###.###.#.#.#######  
  #.#.........#...#.............#  
  #########.###.###.#############  
           B   J   C               
           U   P   P               ";
        let maze = Maze::from_lines(&data);
        assert_eq!(58, part1(&maze));
    }

    #[test]
    fn case3() {
        let data = "             Z L X W       C                 
             Z P Q B       K                 
  ###########.#.#.#.#######.###############  
  #...#.......#.#.......#.#.......#.#.#...#  
  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###  
  #.#...#.#.#...#.#.#...#...#...#.#.......#  
  #.###.#######.###.###.#.###.###.#.#######  
  #...#.......#.#...#...#.............#...#  
  #.#########.#######.#.#######.#######.###  
  #...#.#    F       R I       Z    #.#.#.#  
  #.###.#    D       E C       H    #.#.#.#  
  #.#...#                           #...#.#  
  #.###.#                           #.###.#  
  #.#....OA                       WB..#.#..ZH
  #.###.#                           #.#.#.#  
CJ......#                           #.....#  
  #######                           #######  
  #.#....CK                         #......IC
  #.###.#                           #.###.#  
  #.....#                           #...#.#  
  ###.###                           #.#.#.#  
XF....#.#                         RF..#.#.#  
  #####.#                           #######  
  #......CJ                       NM..#...#  
  ###.#.#                           #.###.#  
RE....#.#                           #......RF
  ###.###        X   X       L      #.#.#.#  
  #.....#        F   Q       P      #.#.#.#  
  ###.###########.###.#######.#########.###  
  #.....#...#.....#.......#...#.....#.#...#  
  #####.#.###.#######.#######.###.###.#.#.#  
  #.......#.......#.#.#.#.#...#...#...#.#.#  
  #####.###.#####.#.#.#.#.###.###.#.###.###  
  #.......#.....#.#...#...............#...#  
  #############.#.#.###.###################  
               A O F   N                     
               A A D   M                     ";
        let maze = Maze::from_lines(&data);
        assert_eq!(396, part2(&maze));
    }
}
