use std::cmp::Ordering;

fn parse(input: &str) -> Vec<[i16; 4]> {
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

fn part1(robots: &[[i16; 4]], width: i32, height: i32) -> usize {
    let mut c = [0; 4];
    for r in robots.iter() {
        let w = ((r[0] as i32 + r[2] as i32 * 100) % width + width) % width;
        let h = ((r[1] as i32 + r[3] as i32 * 100) % height + height) % height;
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

fn part2(mut robots: Vec<[i16; 4]>, width: i16, height: i16) -> i16 {
    let mut img = vec![false; width as usize * height as usize];
    for i in 0..width * height {
        robots.iter_mut().for_each(|r| {
            r[0] = (r[0] + r[2] + width) % width;
            r[1] = (r[1] + r[3] + height) % height;
        });

        img.fill(false);
        for r in &robots {
            img[(r[1] * width + r[0]) as usize] = true;
        }
        if img
            .chunks(width as usize)
            .any(|r| consecutive_pixels(r) > 8)
        {
            // println!("{}", i + 1);
            // for r in img.chunks(width as usize) {
            //     let row: String = r.iter().map(|&x| if x { '#' } else { ' ' }).collect();
            //     println!("{row}");
            // }
            return i + 1;
        }
    }
    unreachable!()
}

pub fn main() {
    let input: String = std::fs::read_to_string("data/2024/day14").unwrap();
    let robots = parse(&input);
    println!("part1: {}", part1(&robots, 101, 103));
    println!("part2: {}", part2(robots, 101, 103));
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
}
