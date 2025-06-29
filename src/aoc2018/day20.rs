use std::collections::VecDeque;

use ahash::{HashMap, HashSet};

fn parse(data: &str) -> &[u8] {
    let s = data.trim().as_bytes();
    &s[1..s.len() - 1]
}

fn reveal_position(m: &mut HashMap<(i16, i16), u8>, (x, y): (i16, i16), b: u8) {
    m.insert((x, y), b);
    for x1 in x - 1..=x + 1 {
        for y1 in y - 1..=y + 1 {
            if (x1, y1) == (x, y) {
                continue;
            }
            m.entry((x1, y1)).or_insert(b'#');
        }
    }
}

fn explore(regex: &[u8]) -> HashMap<(i16, i16), u8> {
    let mut m = HashMap::default();
    reveal_position(&mut m, (0, 0), b'X');

    fn dfs<'a>(
        mut r: &'a [u8],
        m: &mut HashMap<(i16, i16), u8>,
        start: HashSet<(i16, i16)>,
    ) -> (&'a [u8], HashSet<(i16, i16)>) {
        if r.is_empty() {
            return (r, start);
        }
        let mut result = HashSet::default();
        let mut curr = start.clone();
        while let Some(&b) = r.first() {
            match b {
                b'(' => {
                    let (rem, brs) = dfs(&r[1..], m, curr);
                    r = rem;
                    curr = brs;
                }
                b')' => {
                    r = &r[1..];
                    break;
                }
                b'|' => {
                    result.extend(curr);
                    curr = start.clone();
                    r = &r[1..];
                }
                b'W' | b'E' | b'N' | b'S' => {
                    let (dx, dy, c) = match b {
                        b'W' => (-1, 0, b'|'),
                        b'E' => (1, 0, b'|'),
                        b'N' => (0, 1, b'-'),
                        b'S' => (0, -1, b'-'),
                        _ => unreachable!(),
                    };
                    curr = curr
                        .into_iter()
                        .map(|(x, y)| {
                            reveal_position(m, (x + dx, y + dy), c);
                            reveal_position(m, (x + 2 * dx, y + 2 * dy), b'.');
                            (x + 2 * dx, y + 2 * dy)
                        })
                        .collect();
                    r = &r[1..];
                }
                _ => unreachable!(),
            }
        }
        result.extend(curr);
        (r, result)
    }
    dfs(regex, &mut m, [(0, 0)].into_iter().collect());
    m
}

#[allow(unused)]
fn get_map_boundbox(m: &HashMap<(i16, i16), u8>) -> ((i16, i16), (i16, i16)) {
    let mut min = (i16::MAX, i16::MAX);
    let mut max = (i16::MIN, i16::MIN);
    for &(x, y) in m.keys() {
        min.0 = min.0.min(x);
        min.1 = min.1.min(y);
        max.0 = max.0.max(x);
        max.1 = max.1.max(y);
    }
    (min, max)
}

#[allow(unused)]
fn draw_map(m: &HashMap<(i16, i16), u8>) -> String {
    let (min, max) = get_map_boundbox(m);
    // one extra column width for '\n'
    let width = (max.0 - min.0) as usize + 2;
    let height = (max.1 - min.1) as usize + 1;
    let mut map = vec![b'#'; width * height];
    for i in 0..height {
        map[(i + 1) * width - 1] = b'\n';
        for j in 0..width - 1 {
            let x = j as i16 + min.0;
            let y = max.1 - i as i16;
            if let Some(&b) = m.get(&(x, y)) {
                map[i * width + j] = b;
            }
        }
    }
    map.pop();
    unsafe { String::from_utf8_unchecked(map) }
}

fn solve(map: &HashMap<(i16, i16), u8>) -> (usize, usize) {
    let mut part1 = 0;
    let mut q = VecDeque::new();
    let mut visited = HashSet::default();
    q.push_back((0, 0, 0));
    visited.insert((0, 0));
    let mut rooms_within_1000 = 0;
    while let Some((x, y, d)) = q.pop_front() {
        part1 = part1.max(d);
        for (x1, y1) in [(x - 2, y), (x + 2, y), (x, y + 2), (x, y - 2)] {
            let door = map
                .get(&((x + x1) / 2, (y + y1) / 2))
                .cloned()
                .unwrap_or(b'#');
            if (door == b'|' || door == b'-') && !visited.contains(&(x1, y1)) {
                if d + 1 == 1000 && rooms_within_1000 == 0 {
                    rooms_within_1000 = visited.len();
                }
                q.push_back((x1, y1, d + 1));
                visited.insert((x1, y1));
            }
        }
    }
    let part2 = map.values().filter(|e| **e == b'.').count() + 1 - rooms_within_1000;
    (part1, part2)
}

pub fn main() {
    let data = std::fs::read_to_string("data/2018/day20").unwrap();
    let regex = parse(&data);
    let map = explore(regex);
    let (p1, p2) = solve(&map);
    println!("part1: {p1}");
    println!("part2: {p2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "^WNE$";
        let expected = "
#####
#.|.#
#-###
#.|X#
#####";
        let regex = parse(data);
        let map = explore(regex);
        assert_eq!(expected.trim(), draw_map(&map));
        assert_eq!(3, solve(&map).0);
    }

    #[test]
    fn case2() {
        let data = "^ENWWW(NEEE|SSE(EE|N))$";
        let expected = "
#########
#.|.|.|.#
#-#######
#.|.|.|.#
#-#####-#
#.#.#X|.#
#-#-#####
#.|.|.|.#
#########";
        let regex = parse(data);
        let map = explore(regex);
        assert_eq!(expected.trim(), draw_map(&map));
        assert_eq!(10, solve(&map).0);
    }

    #[test]
    fn case3() {
        let data = "^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$";
        let expected = "
###########
#.|.#.|.#.#
#-###-#-#-#
#.|.|.#.#.#
#-#####-#-#
#.#.#X|.#.#
#-#-#####-#
#.#.|.|.|.#
#-###-###-#
#.|.|.#.|.#
###########";
        let regex = parse(data);
        let map = explore(regex);
        assert_eq!(expected.trim(), draw_map(&map));
        assert_eq!(18, solve(&map).0);
    }

    #[test]
    fn case4() {
        let data = "^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$";
        let expected = "
#############
#.|.|.|.|.|.#
#-#####-###-#
#.#.|.#.#.#.#
#-#-###-#-#-#
#.#.#.|.#.|.#
#-#-#-#####-#
#.#.#.#X|.#.#
#-#-#-###-#-#
#.|.#.|.#.#.#
###-#-###-#-#
#.|.#.|.|.#.#
#############";
        let regex = parse(data);
        let map = explore(regex);
        assert_eq!(expected.trim(), draw_map(&map));
        assert_eq!(23, solve(&map).0);
    }

    #[test]
    fn case5() {
        let data = "^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$";
        let expected = "
###############
#.|.|.|.#.|.|.#
#-###-###-#-#-#
#.|.#.|.|.#.#.#
#-#########-#-#
#.#.|.|.|.|.#.#
#-#-#########-#
#.#.#.|X#.|.#.#
###-#-###-#-#-#
#.|.#.#.|.#.|.#
#-###-#####-###
#.|.#.|.|.#.#.#
#-#-#####-#-#-#
#.#.|.|.|.#.|.#
###############";
        let regex = parse(data);
        let map = explore(regex);
        assert_eq!(expected.trim(), draw_map(&map));
        assert_eq!(31, solve(&map).0);
    }
}
