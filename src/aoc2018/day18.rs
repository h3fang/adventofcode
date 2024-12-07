use ahash::HashMap;

fn parse(data: &str) -> Vec<Vec<u8>> {
    let mut m = data
        .trim()
        .lines()
        .map(|line| {
            let mut r = line.as_bytes().to_vec();
            r.insert(0, b' ');
            r.push(b' ');
            r
        })
        .collect::<Vec<_>>();
    m.insert(0, vec![b' '; m[0].len()]);
    m.push(vec![b' '; m[0].len()]);
    m
}

// fn print_map(map: &[Vec<u8>]) {
//     for r in map {
//         println!("{}", unsafe { std::str::from_utf8_unchecked(r) });
//     }
// }

fn neighbors(i: usize, j: usize) -> [(usize, usize); 8] {
    [
        (i - 1, j - 1),
        (i - 1, j),
        (i - 1, j + 1),
        (i, j - 1),
        (i, j + 1),
        (i + 1, j - 1),
        (i + 1, j),
        (i + 1, j + 1),
    ]
}

fn step(map: &mut [Vec<u8>]) {
    let m = map.len();
    let n = map[0].len();
    let original = map.to_vec();
    for (i, row) in map.iter_mut().enumerate().skip(1).take(m - 2) {
        for (j, cell) in row.iter_mut().enumerate().skip(1).take(n - 2) {
            match *cell {
                b'.' => {
                    let mut trees = 0;
                    for (i1, j1) in neighbors(i, j) {
                        trees += i32::from(original[i1][j1] == b'|');
                        if trees == 3 {
                            *cell = b'|';
                            break;
                        }
                    }
                }
                b'|' => {
                    let mut lumberyards = 0;
                    for (i1, j1) in neighbors(i, j) {
                        lumberyards += i32::from(original[i1][j1] == b'#');
                        if lumberyards == 3 {
                            *cell = b'#';
                        }
                    }
                }
                b'#' => {
                    let mut tree = false;
                    let mut lumberyard = false;
                    for (i1, j1) in neighbors(i, j) {
                        tree |= original[i1][j1] == b'|';
                        lumberyard |= original[i1][j1] == b'#';
                        if tree && lumberyard {
                            break;
                        }
                    }
                    if !(tree && lumberyard) {
                        *cell = b'.';
                    }
                }
                _ => unreachable!(),
            }
        }
    }
}

fn resource_value(map: &[Vec<u8>]) -> usize {
    let mut trees = 0;
    let mut lumberyards = 0;
    map.iter().skip(1).take(map.len() - 2).for_each(|row| {
        row.iter()
            .skip(1)
            .take(row.len() - 2)
            .for_each(|c| match c {
                b'|' => trees += 1,
                b'#' => lumberyards += 1,
                _ => {}
            })
    });
    trees * lumberyards
}

fn part1(map: &mut [Vec<u8>]) -> usize {
    for _ in 0..10 {
        step(map);
        // print_map(map);
    }
    resource_value(map)
}

fn part2(mut map: Vec<Vec<u8>>) -> usize {
    let mut seen = HashMap::default();
    let mut values = vec![0; 11];
    let mut i = 10;
    loop {
        i += 1;
        step(&mut map);
        if let Some(&j) = seen.get(&map) {
            let period = i - j;
            let k: usize = (10_0000_0000 - i) % period;
            return values[k + j];
        }
        seen.insert(map.clone(), i);
        values.push(resource_value(&map));
    }
}

pub fn main() {
    let data = std::fs::read_to_string("data/2018/day18").unwrap();
    let mut map = parse(&data);
    println!("part1: {}", part1(&mut map));
    println!("part2: {}", part2(map));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "
.#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|.";
        let mut map = parse(data);
        assert_eq!(1147, part1(&mut map));
    }
}
