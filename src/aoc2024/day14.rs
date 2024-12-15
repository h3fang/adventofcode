use std::cmp::Ordering;

fn parse(input: &str) -> Vec<[i32; 4]> {
    input
        .trim()
        .lines()
        .map(|x| {
            let (p, v) = x.split_once(' ').unwrap();
            let (px, py) = p.trim_start_matches("p=").split_once(',').unwrap();
            let (vx, vy) = v.trim_start_matches("v=").split_once(',').unwrap();
            [
                px.parse().unwrap(),
                py.parse().unwrap(),
                vx.parse().unwrap(),
                vy.parse().unwrap(),
            ]
        })
        .collect()
}

fn part1(robots: &[[i32; 4]], width: i32, height: i32) -> usize {
    let mut c = [0; 4];
    for r in robots.iter() {
        let w = ((r[0] + r[2] * 100) % width + width) % width;
        let h = ((r[1] + r[3] * 100) % height + height) % height;
        match (w.cmp(&(&width / 2)), h.cmp(&(&height / 2))) {
            (Ordering::Less, Ordering::Less) => c[0] += 1,
            (Ordering::Less, Ordering::Greater) => c[1] += 1,
            (Ordering::Greater, Ordering::Less) => c[2] += 1,
            (Ordering::Greater, Ordering::Greater) => c[3] += 1,
            _ => {}
        }
    }
    c.iter().product()
}

fn consecutive_pixels(r: &[bool]) -> usize {
    let (mut max, mut curr) = (0, 0);
    for &x in r {
        if x {
            curr += 1;
            max = curr.max(curr);
        } else {
            curr = 0;
        }
    }
    max
}

fn extended_gcd(a: i32, b: i32) -> (i32, i32, i32) {
    let (mut r0, mut r1) = (a, b);
    let (mut s0, mut s1) = (1, 0);
    let (mut t0, mut t1) = (0, 1);
    while r1 != 0 {
        let q = r0 / r1;
        let old = (r1, s1, t1);
        (r1, s1, t1) = (r0 - q * r1, s0 - q * s1, t0 - q * t1);
        (r0, s0, t0) = old;
    }
    (r0, s0, t0)
}

fn mod_inv(x: i32, n: i32) -> i32 {
    let (g, x, _) = extended_gcd(x, n);
    assert_eq!(g, 1);
    (x % n + n) % n
}

fn chinese_remainder(residues: &[i32], mods: &[i32]) -> i32 {
    let n: i32 = mods.iter().product();
    residues
        .iter()
        .zip(mods)
        .map(|(&r, &m)| {
            let ni = n / m;
            r * mod_inv(ni, m) * ni
        })
        .sum::<i32>()
        % n
}

fn part2(robots: &[[i32; 4]], width: i32, height: i32) -> i32 {
    // [2, 72] can be found by examine the first 100 seconds
    let k = chinese_remainder(&[2, 72], &[width, height]);
    let mut img = vec![false; width as usize * height as usize];
    for r in robots {
        let w = ((r[0] + r[2] * k) % width + width) % width;
        let h = ((r[1] + r[3] * k) % height + height) % height;
        img[(h * width + w) as usize] = true;
    }
    if img
        .chunks(width as usize)
        .all(|r| consecutive_pixels(r) <= 8)
    {
        unreachable!();
    }

    // for r in img.chunks(width as usize) {
    //     let row: String = r.iter().map(|&x| if x { '#' } else { ' ' }).collect();
    //     println!("{row}");
    // }

    k
}

pub fn main() {
    let input: String = std::fs::read_to_string("data/2024/day14").unwrap();
    let robots = parse(&input);
    println!("part1: {}", part1(&robots, 101, 103));
    println!("part2: {}", part2(&robots, 101, 103));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = "
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        let robots = parse(input);
        assert_eq!(12, part1(&robots, 11, 7));
    }

    #[test]
    fn egcd() {
        assert_eq!((2, -9, 47), extended_gcd(240, 46));
        assert_eq!((1, 4, -31), extended_gcd(101, 13));
        assert_eq!((1, 13, -9), extended_gcd(25, 36));
    }
}
