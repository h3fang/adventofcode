use std::f64::consts::{FRAC_PI_2, TAU};

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn line_of_sight(map: &[&[u8]], (x0, y0): (i64, i64), (x1, y1): (i64, i64)) -> bool {
    let mut dx = x1 - x0;
    let mut dy = y1 - y0;
    if dx == 0 || dy == 0 {
        dx = dx.signum();
        dy = dy.signum();
    } else {
        let divisor = gcd(dx.abs(), dy.abs());
        dx /= divisor;
        dy /= divisor;
    }
    let (mut x, mut y) = (x0 + dx, y0 + dy);
    while (x, y) != (x1, y1) {
        if map[y as usize][x as usize] == b'#' {
            return false;
        }
        x += dx;
        y += dy;
    }
    true
}

fn can_observe(map: &[&[u8]], (x0, y0): (usize, usize)) -> usize {
    let mut result = 0;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == b'#'
                && (x, y) != (x0, y0)
                && line_of_sight(map, (x0 as i64, y0 as i64), (x as i64, y as i64))
            {
                result += 1;
            }
        }
    }
    result
}

fn part1(map: &[&[u8]]) -> (usize, (usize, usize)) {
    let mut best = 0;
    let mut best_pos = (0, 0);
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == b'#' {
                let n = can_observe(map, (x, y));
                if n > best {
                    best = n;
                    best_pos = (x, y);
                }
            }
        }
    }
    (best, best_pos)
}

#[derive(PartialEq, Eq)]
struct Position {
    x: i64,
    y: i64,
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.is_on_same_ray(other) {
            self.square_length().cmp(&other.square_length())
        } else {
            let theta1 = self.angle();
            let theta2 = other.angle();
            assert!(theta1 != theta2);
            theta1.partial_cmp(&theta2).unwrap()
        }
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Position {
    #[inline]
    fn square_length(&self) -> i64 {
        self.x * self.x + self.y * self.y
    }

    #[inline]
    fn angle(&self) -> f64 {
        (-(self.y as f64).atan2(self.x as f64) + FRAC_PI_2).rem_euclid(TAU)
    }

    #[inline]
    fn is_on_same_ray(&self, other: &Self) -> bool {
        self.x.signum() == other.x.signum()
            && self.y.signum() == other.y.signum()
            && self.x * other.y == self.y * other.x
    }
}

fn part2(map: &[&[u8]], (x0, y0): (usize, usize), n: usize) -> i64 {
    let mut asteroids = vec![];
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == b'#' && (x, y) != (x0, y0) {
                asteroids.push(Position {
                    x: x as i64 - x0 as i64,
                    y: y0 as i64 - y as i64,
                });
            }
        }
    }
    asteroids.sort_unstable();
    let mut i = 0;
    let mut prev = Position { x: 0, y: 0 };
    for _ in 0..n {
        prev = asteroids.remove(i);
        while i < asteroids.len() && asteroids[i].is_on_same_ray(&prev) {
            i += 1;
            if i == asteroids.len() {
                i = 0;
            }
        }
    }
    (prev.x + x0 as i64) * 100 + y0 as i64 - prev.y
}

pub fn main() {
    let data = std::fs::read_to_string("data/2019/day10").unwrap();
    let map = data.lines().map(|row| row.as_bytes()).collect::<Vec<_>>();

    let (num, pos) = part1(&map);
    println!("day10 part1: {}", num);
    println!("day10 part2: {}", part2(&map, pos, 200));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = ".#..##.###...#######
        ##.############..##.
        .#.######.########.#
        .###.#######.####.#.
        #####.##.#.##.###.##
        ..#####..#.#########
        ####################
        #.####....###.#.#.##
        ##.#################
        #####.##.###..####..
        ..######..##.#######
        ####.##.####...##..#
        .#####..#.######.###
        ##...#.##########...
        #.##########.#######
        .####.#.###.###.#.##
        ....##.##.###..#####
        .#.#.###########.###
        #.#.#.#####.####.###
        ###.##.####.##.#..##";
        let map = data
            .lines()
            .map(|row| row.trim().as_bytes())
            .collect::<Vec<_>>();

        let (num, pos) = part1(&map);
        assert_eq!(210, num);
        assert_eq!((11, 13), pos);
        assert_eq!(1016, part2(&map, pos, 100));
        assert_eq!(906, part2(&map, pos, 199));
        assert_eq!(802, part2(&map, pos, 200));
        assert_eq!(1009, part2(&map, pos, 201));
    }
}
