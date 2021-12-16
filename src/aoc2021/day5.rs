use ahash::AHashMap as HashMap;

fn part1(lines: &[Vec<i64>]) -> usize {
    let mut map: HashMap<(i64, i64), usize> = HashMap::new();
    for line in lines {
        let dx = (line[2] - line[0]).signum();
        let dy = (line[3] - line[1]).signum();
        if dx != 0 && dy != 0 {
            continue;
        }
        let mut x = line[0];
        let mut y = line[1];
        *map.entry((x, y)).or_default() += 1;
        while (x, y) != (line[2], line[3]) {
            x += dx;
            y += dy;
            *map.entry((x, y)).or_default() += 1;
        }
    }
    map.values().filter(|p| **p >= 2).count()
}

fn part2(lines: &[Vec<i64>]) -> usize {
    let mut map: HashMap<(i64, i64), usize> = HashMap::new();
    for line in lines {
        let dx = (line[2] - line[0]).signum();
        let dy = (line[3] - line[1]).signum();
        let mut x = line[0];
        let mut y = line[1];
        *map.entry((x, y)).or_default() += 1;
        while (x, y) != (line[2], line[3]) {
            x += dx;
            y += dy;
            *map.entry((x, y)).or_default() += 1;
        }
    }
    map.values().filter(|p| **p >= 2).count()
}

pub fn main() {
    let data = std::fs::read_to_string("data/2021/day5").unwrap();
    let lines = data
        .lines()
        .map(|r| {
            r.split("->")
                .map(|p| p.split(',').map(|n| n.trim().parse::<i64>().unwrap()))
                .flatten()
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!("day5 part1: {}", part1(&lines));
    println!("day5 part2: {}", part2(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2";
        let lines = data
            .lines()
            .map(|r| {
                r.split("->")
                    .map(|p| {
                        p.split(",")
                            .map(|n| n.trim().parse::<i64>().unwrap())
                            .collect::<Vec<_>>()
                    })
                    .flatten()
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        assert_eq!(5, part1(&lines));
    }
}
