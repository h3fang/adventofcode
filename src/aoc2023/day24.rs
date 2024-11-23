use std::ops::{Add, Div, Mul, RangeInclusive, Sub};

#[derive(Debug, Clone, Copy)]
struct Vec3 {
    x: i128,
    y: i128,
    z: i128,
}

impl Vec3 {
    fn from(a: &[i128]) -> Self {
        Self {
            x: a[0],
            y: a[1],
            z: a[2],
        }
    }

    fn dot(&self, rhs: Self) -> i128 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    fn cross(&self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

impl Mul<Vec3> for i128 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Div<i128> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: i128) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

fn parse(data: &str) -> Vec<[Vec3; 2]> {
    data.trim()
        .lines()
        .map(|l| {
            let (p, v) = l.split_once(" @ ").unwrap();
            let p = p
                .split(',')
                .map(|a| a.trim().parse().unwrap())
                .collect::<Vec<_>>();
            let v = v
                .split(',')
                .map(|a| a.trim().parse().unwrap())
                .collect::<Vec<_>>();
            [Vec3::from(&p), Vec3::from(&v)]
        })
        .collect()
}

fn check_intersection(a: &[Vec3; 2], b: &[Vec3; 2], limit: &RangeInclusive<f64>) -> bool {
    let d = b[0] - a[0];
    let det = b[1].x * a[1].y - b[1].y * a[1].x;
    if det == 0 {
        return false;
    }
    let t1 = (d.y * b[1].x - d.x * b[1].y) as f64 / det as f64;
    let t2 = (d.y * a[1].x - d.x * a[1].y) as f64 / det as f64;

    if t1 <= 0.0 || t2 <= 0.0 {
        return false;
    }
    let x = a[0].x as f64 + t1 * a[1].x as f64;
    let y = a[0].y as f64 + t1 * a[1].y as f64;
    limit.contains(&x) && limit.contains(&y)
}

fn part1(hailstones: &[[Vec3; 2]], limit: [i128; 2]) -> usize {
    let limit = limit[0] as f64..=limit[1] as f64;
    let mut result = 0;
    for (i, a) in hailstones.iter().enumerate() {
        for b in &hailstones[i + 1..] {
            if check_intersection(a, b, &limit) {
                result += 1;
            }
        }
    }
    result
}

// https://www.reddit.com/r/adventofcode/comments/18pnycy/comment/kxqjg33/
fn part2(hailstones: &[[Vec3; 2]]) -> i128 {
    let (h0, h1, h2) = (&hailstones[0], &hailstones[1], &hailstones[2]);
    let (p1, v1) = (h1[0] - h0[0], h1[1] - h0[1]);
    let (p2, v2) = (h2[0] - h0[0], h2[1] - h0[1]);
    let t1 = -p1.cross(p2).dot(v2) / v1.cross(p2).dot(v2);
    let t2 = -p1.cross(p2).dot(v1) / p1.cross(v2).dot(v1);
    let (c1, c2) = (h1[0] + t1 * h1[1], h2[0] + t2 * h2[1]);
    let v = (c2 - c1) / (t2 - t1);
    let p = c1 - t1 * v;
    p.x + p.y + p.z
}

pub fn main() {
    let data = std::fs::read_to_string("data/2023/day24").unwrap();
    let hailstones = parse(&data);
    println!(
        "part1: {}",
        part1(&hailstones, [200000000000000, 400000000000000])
    );
    println!("part2: {}", part2(&hailstones));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = r"
19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";
        let hailstones = parse(data);
        assert_eq!(2, part1(&hailstones, [7, 27]));
        assert_eq!(47, part2(&hailstones));
    }
}
