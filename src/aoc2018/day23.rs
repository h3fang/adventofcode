use std::collections::BinaryHeap;

use arrayvec::ArrayVec;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    const ZERO: Self = Point::new(0, 0, 0);

    const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Cuboid {
    min: Point,
    max: Point,
}

#[derive(Clone, Copy)]
struct Bot {
    pos: Point,
    radius: i32,
}

fn parse(data: &str) -> Vec<Bot> {
    data.trim()
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(">, r=").unwrap();
            let mut pos = a.trim_start_matches("pos=<").split(',');
            let x = pos.next().unwrap().parse().unwrap();
            let y = pos.next().unwrap().parse().unwrap();
            let z = pos.next().unwrap().parse().unwrap();
            let radius = b.parse().unwrap();
            Bot {
                pos: Point { x, y, z },
                radius,
            }
        })
        .collect()
}

#[inline]
fn manhattan(a: Point, b: Point) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs() + (a.z - b.z).abs()
}

fn part1(nanobots: &[Bot]) -> usize {
    let strongest = nanobots
        .iter()
        .enumerate()
        .max_by_key(|e| e.1.radius)
        .map(|e| e.0)
        .unwrap();
    let s = &nanobots[strongest];
    nanobots
        .iter()
        .filter(|&n| manhattan(s.pos, n.pos) <= s.radius)
        .count()
}

fn point_in_cuboid(p: Point, c: Cuboid) -> bool {
    p.x >= c.min.x
        && p.x <= c.max.x
        && p.y >= c.min.y
        && p.y <= c.max.y
        && p.z >= c.min.z
        && p.z <= c.max.z
}

fn point_in_octahedron(p: Point, b: Bot) -> bool {
    manhattan(p, b.pos) <= b.radius
}

fn bot_intersect_cuboid(b: Bot, c: Cuboid) -> bool {
    let (x, y, z, r) = (b.pos.x, b.pos.y, b.pos.z, b.radius);
    let octahedron_points = [
        Point::new(x - r, y, z),
        Point::new(x + r, y, z),
        Point::new(x, y - r, z),
        Point::new(x, y + r, z),
        Point::new(x, y, z - r),
        Point::new(x, y, z + r),
    ];
    if octahedron_points.into_iter().any(|p| point_in_cuboid(p, c)) {
        return true;
    }
    let cuboid_points = [
        c.min,
        c.max,
        Point::new(c.max.x, c.min.y, c.min.z),
        Point::new(c.min.x, c.max.y, c.min.z),
        Point::new(c.min.x, c.min.y, c.max.z),
        Point::new(c.max.x, c.max.y, c.min.z),
        Point::new(c.max.x, c.min.y, c.max.z),
        Point::new(c.min.x, c.max.y, c.max.z),
    ];
    cuboid_points.into_iter().any(|p| point_in_octahedron(p, b))
}

fn divide(c: Cuboid) -> ArrayVec<Cuboid, 8> {
    let (x1, y1, z1) = (c.min.x, c.min.y, c.min.z);
    let (x2, y2, z2) = (c.max.x, c.max.y, c.max.z);
    let (xm, ym, zm) = ((x1 + x2) / 2, (y1 + y2) / 2, (z1 + z2) / 2);

    let mut xs: ArrayVec<(i32, i32), 2> = ArrayVec::new();
    xs.push((x1, xm));
    if x2 != x1 {
        xs.push((xm + 1, x2));
    }

    let mut ys: ArrayVec<(i32, i32), 2> = ArrayVec::new();
    ys.push((y1, ym));
    if y2 != y1 {
        ys.push((ym + 1, y2));
    }

    let mut zs: ArrayVec<(i32, i32), 2> = ArrayVec::new();
    zs.push((z1, zm));
    if z2 != z1 {
        zs.push((zm + 1, z2));
    }

    let mut result = ArrayVec::new();
    for &(x1, x2) in &xs {
        for &(y1, y2) in &ys {
            for &(z1, z2) in &zs {
                result.push(Cuboid {
                    min: Point::new(x1, y1, z1),
                    max: Point::new(x2, y2, z2),
                });
            }
        }
    }
    result
}

fn part2(nanobots: &[Bot]) -> i32 {
    let mut q = BinaryHeap::new();
    let b = Cuboid {
        min: Point::new(i32::MIN / 4, i32::MIN / 4, i32::MIN / 4),
        max: Point::new(i32::MAX / 4, i32::MAX / 4, i32::MAX / 4),
    };
    q.push((nanobots.len(), b));
    let (mut max, mut dist) = (0, 0);
    while let Some((n, b)) = q.pop() {
        if n < max {
            break;
        }
        if b.max == b.min {
            match n.cmp(&max) {
                std::cmp::Ordering::Less => {}
                std::cmp::Ordering::Equal => {
                    dist = manhattan(b.min, Point::ZERO).min(dist);
                }
                std::cmp::Ordering::Greater => {
                    max = n;
                    dist = manhattan(b.min, Point::ZERO);
                }
            }
        } else {
            for b1 in divide(b) {
                let n1 = nanobots
                    .iter()
                    .filter(|&&o| bot_intersect_cuboid(o, b1))
                    .count();
                if n1 > 0 && n1 >= max {
                    q.push((n1, b1));
                }
            }
        }
    }
    dist
}

pub fn main() {
    let data = std::fs::read_to_string("data/2018/day23").unwrap();
    let nanobots = parse(&data);
    println!("part1: {}", part1(&nanobots));
    println!("part2: {}", part2(&nanobots));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "
pos=<0,0,0>, r=4
pos=<1,0,0>, r=1
pos=<4,0,0>, r=3
pos=<0,2,0>, r=1
pos=<0,5,0>, r=3
pos=<0,0,3>, r=1
pos=<1,1,1>, r=1
pos=<1,1,2>, r=1
pos=<1,3,1>, r=1";
        let nanobots = parse(data);
        assert_eq!(7, part1(&nanobots));
    }

    #[test]
    fn case2() {
        let data = "
pos=<10,12,12>, r=2
pos=<12,14,12>, r=2
pos=<16,12,12>, r=4
pos=<14,14,14>, r=6
pos=<50,50,50>, r=200
pos=<10,10,10>, r=5";
        let nanobots = parse(data);
        assert_eq!(36, part2(&nanobots));
    }

    #[test]
    fn case3() {
        let data = std::fs::read_to_string("data/2018/day23").unwrap();
        let nanobots = parse(&data);
        assert_eq!(602, part1(&nanobots));
        assert_eq!(110620102, part2(&nanobots));
    }
}
