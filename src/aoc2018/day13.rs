use ahash::{HashMap, HashSet};

#[derive(Clone)]
struct Cart {
    i: usize,
    j: usize,
    c: u8,
    turn: u8,
}

impl Cart {
    fn new(i: usize, j: usize, c: u8, turn: u8) -> Self {
        Self { i, j, c, turn }
    }
}

fn parse(data: &str) -> (Vec<Vec<u8>>, Vec<Cart>) {
    let mut map = data
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect::<Vec<_>>();

    // map should be non-empty and rectangular
    assert!(!map.is_empty());
    let w = map[0].len();
    assert!(map.iter().all(|row| row.len() == w));

    // find carts (and tracks underneath)
    let mut carts = vec![];
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            let c = map[i][j];
            match c {
                b'^' | b'v' => {
                    map[i][j] = b'|';
                    carts.push(Cart::new(i, j, c, 0));
                }
                b'>' | b'<' => {
                    map[i][j] = b'-';
                    carts.push(Cart::new(i, j, c, 0));
                }
                _ => {}
            }
        }
    }

    (map, carts)
}

fn turn(c: u8, slash: u8) -> u8 {
    match c {
        b'^' => match slash {
            b'/' => b'>',
            b'\\' => b'<',
            _ => unreachable!(),
        },
        b'v' => match slash {
            b'/' => b'<',
            b'\\' => b'>',
            _ => unreachable!(),
        },
        b'<' => match slash {
            b'/' => b'v',
            b'\\' => b'^',
            _ => unreachable!(),
        },
        b'>' => match slash {
            b'/' => b'^',
            b'\\' => b'v',
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

fn cross(c: u8, turn: &mut u8) -> u8 {
    let r = match c {
        b'^' => match *turn {
            0 => b'<',
            1 => b'^',
            2 => b'>',
            _ => unreachable!(),
        },
        b'v' => match *turn {
            0 => b'>',
            1 => b'v',
            2 => b'<',
            _ => unreachable!(),
        },
        b'<' => match *turn {
            0 => b'v',
            1 => b'<',
            2 => b'^',
            _ => unreachable!(),
        },
        b'>' => match *turn {
            0 => b'^',
            1 => b'>',
            2 => b'v',
            _ => unreachable!(),
        },
        _ => unreachable!(),
    };
    *turn = (*turn + 1) % 3;
    r
}

fn step(map: &[Vec<u8>], carts: &mut [Cart]) -> Option<(usize, usize)> {
    let mut cart_pos = carts.iter().map(|c| (c.i, c.j)).collect::<HashSet<_>>();
    carts.sort_unstable_by_key(|c| (c.i, c.j));

    for c in carts.iter_mut() {
        let (i, j) = match c.c {
            b'^' => (c.i - 1, c.j),
            b'v' => (c.i + 1, c.j),
            b'<' => (c.i, c.j - 1),
            b'>' => (c.i, c.j + 1),
            _ => unreachable!(),
        };
        if cart_pos.contains(&(i, j)) {
            return Some((i, j));
        } else {
            cart_pos.remove(&(c.i, c.j));
            c.i = i;
            c.j = j;
            cart_pos.insert((i, j));
            match map[i][j] {
                b'/' => c.c = turn(c.c, b'/'),
                b'\\' => c.c = turn(c.c, b'\\'),
                b'+' => c.c = cross(c.c, &mut c.turn),
                _ => {}
            }
        }
    }
    None
}

fn step2(map: &[Vec<u8>], mut carts: Vec<Cart>) -> Vec<Cart> {
    carts.sort_unstable_by_key(|c| (c.i, c.j));
    let mut cart_pos = carts
        .iter()
        .enumerate()
        .map(|(k, c)| ((c.i, c.j), k))
        .collect::<HashMap<_, _>>();

    let mut removed = HashSet::default();

    for (k, c) in carts.iter_mut().enumerate() {
        if removed.contains(&k) {
            continue;
        }
        let (i, j) = match c.c {
            b'^' => (c.i - 1, c.j),
            b'v' => (c.i + 1, c.j),
            b'<' => (c.i, c.j - 1),
            b'>' => (c.i, c.j + 1),
            _ => unreachable!(),
        };
        if let Some(&k2) = cart_pos.get(&(i, j)) {
            removed.insert(k);
            removed.insert(k2);
            cart_pos.remove(&(c.i, c.j));
            cart_pos.remove(&(i, j));
        } else {
            cart_pos.remove(&(c.i, c.j));
            c.i = i;
            c.j = j;
            cart_pos.insert((i, j), k);
            match map[i][j] {
                b'/' => c.c = turn(c.c, b'/'),
                b'\\' => c.c = turn(c.c, b'\\'),
                b'+' => c.c = cross(c.c, &mut c.turn),
                _ => {}
            }
        }
    }
    carts
        .into_iter()
        .enumerate()
        .filter(|(k, _)| !removed.contains(k))
        .map(|(_, c)| c)
        .collect()
}

// fn print_map(map: &[Vec<u8>], carts: &[Cart]) {
//     let mut m = map.to_vec();
//     for c in carts {
//         m[c.i][c.j] = c.c;
//     }
//     m.iter()
//         .for_each(|r| println!("{}", unsafe { std::str::from_utf8_unchecked(r) }));
// }

fn part1(map: &[Vec<u8>], mut carts: Vec<Cart>) -> (usize, usize) {
    loop {
        if let Some((i, j)) = step(map, &mut carts) {
            return (j, i);
        }
    }
}

fn part2(map: &[Vec<u8>], mut carts: Vec<Cart>) -> (usize, usize) {
    loop {
        carts = step2(map, carts);
        // print_map(map, &carts);
        if carts.len() == 1 {
            return (carts[0].j, carts[0].i);
        }
    }
}

pub fn main() {
    let data = std::fs::read_to_string("data/2018/day13").unwrap();
    let (map, carts) = parse(&data);
    let p1 = part1(&map, carts.clone());
    let p2 = part2(&map, carts);
    println!("part1: {},{}", p1.0, p1.1);
    println!("part2: {},{}", p2.0, p2.1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = r#"/->-\        
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
    \------/ "#
            .to_string();
        let (map, carts) = parse(&data);
        // print_map(&map, &carts);
        assert_eq!((7, 3), part1(&map, carts));
    }

    #[test]
    fn case2() {
        let data = r#"/>-<\  
|   |  
| /<+-\
| | | v
\>+</ |
  |   ^
  \<->/"#
            .to_string();
        let (map, carts) = parse(&data);
        // print_map(&map, &carts);
        assert_eq!((6, 4), part2(&map, carts));
    }
}
