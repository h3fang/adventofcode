use ahash::AHashSet as HashSet;

fn parse(data: &str) -> Vec<(i32, i32)> {
    data.lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (x, y) = line.split_once(", ").unwrap();
            (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
        })
        .collect()
}

fn manhattan_distance((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

fn part1(coords: &[(i32, i32)]) -> usize {
    let (min, max) = coords
        .iter()
        .fold(((i32::MAX, i32::MAX), (i32::MIN, i32::MIN)), |a, p| {
            (
                (a.0 .0.min(p.0), a.0 .1.min(p.1)),
                (a.1 .0.max(p.0), a.1 .1.max(p.1)),
            )
        });

    let is_border =
        |x: i32, y: i32| -> bool { x == min.0 || x == max.0 || y == min.1 || y == max.1 };

    let mut grid = vec![vec![-1; (max.1 - min.1) as usize + 1]; (max.0 - min.0) as usize + 1];
    let mut infinite = HashSet::new();

    for x in min.0..=max.0 {
        for y in min.1..=max.1 {
            let i = (x - min.0) as usize;
            let j = (y - min.1) as usize;
            let mut c = 0;
            let mut m = i32::MAX;
            for (k, &p) in coords.iter().enumerate() {
                let d = manhattan_distance(p, (x, y));
                match d.cmp(&m) {
                    std::cmp::Ordering::Less => {
                        m = d;
                        c = k as i32;
                    }
                    std::cmp::Ordering::Equal => c = -1,
                    _ => {}
                }
            }
            if c != -1 {
                grid[i][j] = c;
                if is_border(x, y) {
                    infinite.insert(c);
                }
            }
        }
    }
    (0..coords.len())
        .filter_map(|k| {
            let k = k as i32;
            if infinite.contains(&k) {
                None
            } else {
                Some(grid.iter().flatten().filter(|e| **e == k).count())
            }
        })
        .max()
        .unwrap()
}

fn part2(coords: &[(i32, i32)], threshold: i32) -> usize {
    let (min, max) = coords
        .iter()
        .fold(((i32::MAX, i32::MAX), (i32::MIN, i32::MIN)), |a, p| {
            (
                (a.0 .0.min(p.0), a.0 .1.min(p.1)),
                (a.1 .0.max(p.0), a.1 .1.max(p.1)),
            )
        });

    let mut result = 0;
    for x in min.0..=max.0 {
        for y in min.1..=max.1 {
            let sum = coords
                .iter()
                .map(|&p| manhattan_distance(p, (x, y)))
                .sum::<i32>();
            if sum < threshold {
                result += 1;
            }
        }
    }
    result
}

pub fn main() {
    let data = std::fs::read_to_string("data/2018/day6").unwrap();
    let coords = parse(&data);
    println!("part1: {}", part1(&coords));
    println!("part2: {}", part2(&coords, 10000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "1, 1\n1, 6\n8, 3\n3, 4\n5, 5\n8, 9".to_string();
        let coords = parse(&data);
        assert_eq!(17, part1(&coords));
        assert_eq!(16, part2(&coords, 32));
    }
}
