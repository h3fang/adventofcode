fn parse(data: &str) -> Vec<(u32, u32)> {
    data.trim()
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect()
}

fn part1(tiles: &[(u32, u32)]) -> u64 {
    let mut ans = 0;
    for (i, a) in tiles.iter().enumerate() {
        for b in tiles.iter().skip(i + 1) {
            let w = (a.0.abs_diff(b.0) + 1) as u64;
            let h = (a.1.abs_diff(b.1) + 1) as u64;
            ans = ans.max(w * h);
        }
    }
    ans
}

// Not a generalized solution
fn part2(tiles: &[(u32, u32)]) -> u64 {
    let n = tiles.len();
    let i0 = tiles[..n - 1].partition_point(|p| p.1 > 50000) - 1;

    let mut ans = 0;

    // Top half
    let p0 = tiles[i0];
    let i_max = tiles[..i0 - 1].partition_point(|p| p.0 >= p0.0) - 1;
    let y_max = tiles[i_max].1;
    for p in tiles[..i0 - 1].iter().rev() {
        if p.1 > y_max {
            break;
        }
        let w = p0.0.abs_diff(p.0) as u64 + 1;
        let h = p0.1.abs_diff(p.1) as u64 + 1;
        ans = ans.max(w * h);
    }

    // Bottom half
    let i0 = i0 + 1;
    let p0 = tiles[i0];
    let i_max = tiles[i0 + 1..].partition_point(|p| p.0 <= p0.0) - 1;
    let y_min = tiles[i_max].1;
    for p in tiles[..i0 - 1].iter().rev() {
        if p.1 < y_min {
            break;
        }
        let w = p0.0.abs_diff(p.0) as u64 + 1;
        let h = p0.1.abs_diff(p.1) as u64 + 1;
        ans = ans.max(w * h);
    }

    ans
}

pub fn main() {
    let data = std::fs::read_to_string("data/2025/day9").unwrap();
    let tiles = parse(&data);
    println!("part1: {}", part1(&tiles));
    println!("part2: {}", part2(&tiles));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        let tiles = parse(data);
        assert_eq!(50, part1(&tiles));
    }
}
