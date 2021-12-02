use std::ops::{Add, AddAssign, Sub};

use hashbrown::HashSet;

#[derive(Default, Debug, Clone, Copy)]
struct Vec3 {
    x: i64,
    y: i64,
    z: i64,
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Vec3 {
    fn signum(&self) -> Vec3 {
        Vec3 {
            x: self.x.signum(),
            y: self.y.signum(),
            z: self.z.signum(),
        }
    }

    fn energy(&self) -> i64 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

#[derive(Debug, Clone)]
struct Planet {
    p: Vec3,
    v: Vec3,
}

impl Planet {
    fn new(p: Vec3) -> Self {
        Self {
            p,
            v: Vec3::default(),
        }
    }
}

#[derive(Debug, Clone)]
struct NBody {
    planets: Vec<Planet>,
}

impl NBody {
    fn step(&mut self) {
        for i in 0..self.planets.len() {
            let mut g = Vec3::default();
            for j in 0..self.planets.len() {
                g += (self.planets[j].p - self.planets[i].p).signum();
            }
            self.planets[i].v += g;
        }

        for i in 0..self.planets.len() {
            let v = self.planets[i].v;
            self.planets[i].p += v;
        }
    }

    fn total_energy(&self) -> i64 {
        self.planets
            .iter()
            .map(|p| p.p.energy() * p.v.energy())
            .sum()
    }

    fn state<F>(&self, axis: &F) -> Vec<i64>
    where
        F: Fn(&Vec3) -> i64,
    {
        self.planets
            .iter()
            .map(|p| [axis(&p.p), axis(&p.v)])
            .flatten()
            .collect()
    }
}

fn part1(nbody: &mut NBody) -> i64 {
    for _ in 0..1000 {
        nbody.step();
    }
    nbody.total_energy()
}

fn find_period<F>(nbody: &mut NBody, axis: F) -> usize
where
    F: Fn(&Vec3) -> i64,
{
    let mut s = HashSet::new();
    let state = nbody.state(&axis);
    s.insert(state);

    let mut i = 0;
    loop {
        nbody.step();
        i += 1;
        let state = nbody.state(&axis);
        if s.contains(&state) {
            return i;
        }
        s.insert(state);
    }
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn part2(nbody: &mut NBody) -> usize {
    let p1 = find_period(nbody, |v| v.x);
    let p2 = find_period(nbody, |v| v.y);
    let p3 = find_period(nbody, |v| v.z);
    lcm(lcm(p1, p2), p3)
}

pub fn main() {
    let data = std::fs::read_to_string("data/2019/day12").unwrap();
    let planets = data
        .lines()
        .map(|t| {
            let t = t.trim();
            assert_eq!(b'<', t.as_bytes()[0]);
            assert_eq!(b'>', *t.as_bytes().last().unwrap());
            let p = t[1..t.len() - 1]
                .split(',')
                .map(|part| {
                    part.split('=')
                        .nth(1)
                        .unwrap()
                        .trim()
                        .parse::<i64>()
                        .unwrap()
                })
                .collect::<Vec<_>>();
            Planet::new(Vec3 {
                x: p[0],
                y: p[1],
                z: p[2],
            })
        })
        .collect::<Vec<_>>();

    let mut nbody = NBody { planets };
    println!("day12 part1: {}", part1(&mut nbody));
    println!("day12 part2: {}", part2(&mut nbody));
}
