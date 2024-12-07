use ahash::HashSet;

fn parse(data: &str) -> HashSet<(i8, i8, i8)> {
    data.trim()
        .lines()
        .map(|line| {
            let mut p = line.split(',');
            let a = p.next().unwrap().parse().unwrap();
            let b = p.next().unwrap().parse().unwrap();
            let c = p.next().unwrap().parse().unwrap();
            (a, b, c)
        })
        .collect()
}

fn neighbors((x, y, z): (i8, i8, i8)) -> [(i8, i8, i8); 6] {
    [
        (x - 1, y, z),
        (x + 1, y, z),
        (x, y - 1, z),
        (x, y + 1, z),
        (x, y, z - 1),
        (x, y, z + 1),
    ]
}

fn part1(scan: &HashSet<(i8, i8, i8)>) -> usize {
    let mut result = scan.len() * 6;
    for &(x, y, z) in scan {
        for p in neighbors((x, y, z)) {
            if scan.contains(&p) {
                result -= 1;
            }
        }
    }
    result
}

fn boundbox(scan: &HashSet<(i8, i8, i8)>) -> ((i8, i8, i8), (i8, i8, i8)) {
    let mut min = (i8::MAX, i8::MAX, i8::MAX);
    let mut max = (i8::MIN, i8::MIN, i8::MIN);
    for p in scan {
        min.0 = min.0.min(p.0);
        min.1 = min.1.min(p.1);
        min.2 = min.2.min(p.2);

        max.0 = max.0.max(p.0);
        max.1 = max.1.max(p.1);
        max.2 = max.2.max(p.2);
    }
    (min, max)
}

fn flood_fill(
    (x, y, z): (i8, i8, i8),
    min: (i8, i8, i8),
    max: (i8, i8, i8),
    scan: &HashSet<(i8, i8, i8)>,
    exterior: &mut HashSet<(i8, i8, i8)>,
) {
    exterior.insert((x, y, z));
    for (x1, y1, z1) in neighbors((x, y, z)) {
        if x1 < min.0 - 1
            || y1 < min.1 - 1
            || z1 < min.2 - 1
            || x1 > max.0 + 1
            || y1 > max.1 + 1
            || z1 > max.2 + 1
            || scan.contains(&(x1, y1, z1))
            || exterior.contains(&(x1, y1, z1))
        {
            continue;
        }
        flood_fill((x1, y1, z1), min, max, scan, exterior);
    }
}

fn part2(scan: &HashSet<(i8, i8, i8)>) -> usize {
    let (min, max) = boundbox(scan);

    let mut exterior = HashSet::default();
    flood_fill((min.0 - 1, min.1, min.2), min, max, scan, &mut exterior);

    let mut result = 0;
    for &(x, y, z) in scan {
        for p in neighbors((x, y, z)) {
            if exterior.contains(&p) {
                result += 1;
            }
        }
    }
    result
}

pub fn main() {
    let data = std::fs::read_to_string("data/2022/day18").unwrap();
    let scan = parse(&data);
    println!("part1: {}", part1(&scan));
    println!("part2: {}", part2(&scan));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "
2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";
        let scan = parse(data);
        assert_eq!(64, part1(&scan));
        assert_eq!(58, part2(&scan));
    }

    #[test]
    fn case2() {
        let data = "
1,1,1
2,1,1";
        let scan = parse(data);
        assert_eq!(10, part1(&scan));
    }
}
