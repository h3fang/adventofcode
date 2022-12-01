use std::str::FromStr;

struct Rect {
    id: u16,
    x: u16,
    y: u16,
    w: u16,
    h: u16,
}

#[derive(Debug)]
struct InvalidRectangle;

impl FromStr for Rect {
    type Err = InvalidRectangle;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, r) = s.split_once(" @ ").ok_or(InvalidRectangle)?;
        let (xy, wh) = r.split_once(": ").ok_or(InvalidRectangle)?;
        let (x, y) = xy.split_once(',').ok_or(InvalidRectangle)?;
        let (w, h) = wh.split_once('x').ok_or(InvalidRectangle)?;

        let id = id[1..].parse().map_err(|_| InvalidRectangle)?;
        let x = x.parse().map_err(|_| InvalidRectangle)?;
        let y = y.parse().map_err(|_| InvalidRectangle)?;
        let w = w.parse().map_err(|_| InvalidRectangle)?;
        let h = h.parse().map_err(|_| InvalidRectangle)?;
        Ok(Rect { id, x, y, w, h })
    }
}

fn part1(rects: &[Rect], fabric: &mut [[u8; 1000]]) -> usize {
    for r in rects {
        for i in r.x..(r.x + r.w) {
            for j in r.y..(r.y + r.h) {
                fabric[i as usize][j as usize] += 1;
            }
        }
    }
    fabric.iter().flatten().filter(|&&e| e >= 2).count()
}

fn part2(rects: &[Rect], fabric: &mut [[u8; 1000]]) -> u16 {
    for r in rects {
        if (r.x..(r.x + r.w))
            .all(|i| (r.y..(r.y + r.h)).all(|j| fabric[i as usize][j as usize] == 1))
        {
            return r.id;
        }
    }
    unreachable!()
}

pub fn main() {
    let data = std::fs::read_to_string("data/2018/day3").unwrap();
    let rects: Vec<Rect> = data.lines().map(|line| line.parse().unwrap()).collect();
    let mut fabric = [[0u8; 1000]; 1000];
    println!("part1: {}", part1(&rects, &mut fabric));
    println!("part2: {}", part2(&rects, &mut fabric));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2";
        let rects: Vec<Rect> = data.lines().map(|line| line.parse().unwrap()).collect();
        let mut fabric = [[0u8; 1000]; 1000];
        assert_eq!(4, part1(&rects, &mut fabric));
        assert_eq!(3, part2(&rects, &mut fabric));
    }
}
