fn parse(input: &str) -> (Vec<&[u8]>, &[u8]) {
    let (map, movements) = input.trim().split_once("\n\n").unwrap();
    let map = map.lines().map(|l| l.as_bytes()).collect();
    (map, movements.as_bytes())
}

fn find_robot(map: &[Vec<u8>]) -> (i32, i32) {
    for (i, r) in map.iter().enumerate() {
        for (j, c) in r.iter().enumerate() {
            if *c == b'@' {
                return (i as i32, j as i32);
            }
        }
    }
    unreachable!()
}

fn try_push(i0: i32, j0: i32, di: i32, dj: i32, map: &mut [Vec<u8>]) -> (i32, i32) {
    let (mut i, mut j) = (i0, j0);
    loop {
        i += di;
        j += dj;
        let c = map[i as usize][j as usize];
        if c == b'.' {
            while (i - di, j - dj) != (i0, j0) {
                map[i as usize][j as usize] = map[(i - di) as usize][(j - dj) as usize];
                i -= di;
                j -= dj;
            }
            map[i as usize][j as usize] = b'@';
            map[i0 as usize][j0 as usize] = b'.';
            return (i0 + di, j0 + dj);
        }
        if c == b'#' {
            return (i0, j0);
        }
    }
}

// fn print_map(map: &[Vec<u8>]) {
//     for r in map {
//         eprintln!("{}", std::str::from_utf8(r).unwrap());
//     }
// }

fn dir(b: u8) -> (i32, i32) {
    match b {
        b'^' => (-1, 0),
        b'>' => (0, 1),
        b'<' => (0, -1),
        b'v' => (1, 0),
        _ => unreachable!(),
    }
}

fn sum_of_coordinates(map: &[Vec<u8>]) -> usize {
    let mut result = 0;
    for (i, r) in map.iter().enumerate() {
        for (j, &c) in r.iter().enumerate() {
            if c == b'O' || c == b'[' {
                result += i * 100 + j;
            }
        }
    }
    result
}

fn part1(map: &[&[u8]], movements: &[u8]) -> usize {
    let mut map: Vec<Vec<u8>> = map.iter().map(|r| r.to_vec()).collect();
    let (mut i0, mut j0) = find_robot(&map);
    for &b in movements {
        if b == b'\n' {
            continue;
        }
        let (di, dj) = dir(b);
        (i0, j0) = try_push(i0, j0, di, dj, &mut map);
    }
    sum_of_coordinates(&map)
}

fn try_push2(i0: i32, j0: i32, di: i32, dj: i32, map: &[Vec<u8>]) -> bool {
    let (i, j) = (i0 + di, j0 + dj);
    match map[i as usize][j as usize] {
        b'[' => match dj {
            0 => try_push2(i, j, di, dj, map) && try_push2(i, j + 1, di, dj, map),
            1 => try_push2(i, j + 1, di, dj, map),
            -1 => try_push2(i, j, di, dj, map),
            _ => unreachable!(),
        },
        b']' => match dj {
            0 => try_push2(i, j, di, dj, map) && try_push2(i, j - 1, di, dj, map),
            1 => try_push2(i, j, di, dj, map),
            -1 => try_push2(i, j - 1, di, dj, map),
            _ => unreachable!(),
        },
        b'.' => true,
        b'#' => false,
        _ => unreachable!(),
    }
}

fn push(i0: i32, j0: i32, di: i32, dj: i32, map: &mut [Vec<u8>]) {
    let (i, j) = (i0 + di, j0 + dj);
    let pushed = match map[i as usize][j as usize] {
        b'[' => {
            match dj {
                0 => {
                    push(i, j, di, dj, map);
                    push(i, j + 1, di, dj, map);
                }
                1 => {
                    push(i, j + 1, di, dj, map);
                    map[i as usize][(j + 1) as usize] = b'[';
                }
                -1 => push(i, j, di, dj, map),
                _ => unreachable!(),
            }
            true
        }
        b']' => {
            match dj {
                0 => {
                    push(i, j, di, dj, map);
                    push(i, j - 1, di, dj, map);
                }
                1 => push(i, j, di, dj, map),
                -1 => {
                    push(i, j - 1, di, dj, map);
                    map[i as usize][(j - 1) as usize] = b']';
                }
                _ => unreachable!(),
            }
            true
        }
        b'.' => true,
        b'#' => false,
        _ => unreachable!(),
    };
    if pushed {
        map[i as usize][j as usize] = map[i0 as usize][j0 as usize];
        map[i0 as usize][j0 as usize] = b'.';
    }
}

fn part2(map: &[&[u8]], movements: &[u8]) -> usize {
    let mut map: Vec<Vec<u8>> = map
        .iter()
        .map(|r| {
            r.iter()
                .flat_map(|&b| match b {
                    b'#' => [b'#', b'#'],
                    b'O' => [b'[', b']'],
                    b'.' => [b'.', b'.'],
                    b'@' => [b'@', b'.'],
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();
    let (mut i0, mut j0) = find_robot(&map);
    for &b in movements {
        if b == b'\n' {
            continue;
        }
        let (di, dj) = dir(b);
        if try_push2(i0, j0, di, dj, &map) {
            push(i0, j0, di, dj, &mut map);
            i0 += di;
            j0 += dj;
        }
    }
    sum_of_coordinates(&map)
}

pub fn main() {
    let input: String = std::fs::read_to_string("data/2024/day15").unwrap();
    let (map, movements) = parse(&input);
    println!("part1: {}", part1(&map, movements));
    println!("part2: {}", part2(&map, movements));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = "
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
        let (map, movements) = parse(input);
        assert_eq!(10092, part1(&map, movements));
        assert_eq!(9021, part2(&map, movements));
    }

    #[test]
    fn case2() {
        let input = "
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
        let (map, movements) = parse(input);
        assert_eq!(2028, part1(&map, movements));
    }

    #[test]
    fn case3() {
        let input = "
#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";
        let (map, movements) = parse(input);
        assert_eq!(618, part2(&map, movements));
    }
}
