use std::collections::VecDeque;

fn part1(heightmap: &[Vec<u8>]) -> (usize, Vec<(usize, usize)>) {
    let h = heightmap.len();
    let w = heightmap[0].len();
    let mut result = 0;
    let mut basins = vec![];
    for y in 1..h - 1 {
        for x in 1..w - 1 {
            if [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
                .iter()
                .all(|p| heightmap[p.1][p.0] > heightmap[y][x])
            {
                result += heightmap[y][x] as usize + 1;
                basins.push((x, y));
            }
        }
    }
    (result, basins)
}

fn size_of_basin(heightmap: &mut [Vec<u8>], x: usize, y: usize) -> usize {
    let mut q = VecDeque::new();
    let mut result = 0;
    q.push_back((x, y));
    while let Some((x, y)) = q.pop_front() {
        if heightmap[y][x] < 9 {
            result += 1;
            heightmap[y][x] = 9;
            for (xn, yn) in [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)] {
                q.push_back((xn, yn));
            }
        }
    }
    result
}

fn part2(heightmap: &mut [Vec<u8>], basins: &[(usize, usize)]) -> usize {
    let mut m1 = 0;
    let mut m2 = 0;
    let mut m3 = 0;
    for &(x, y) in basins {
        let size = size_of_basin(heightmap, x, y);
        if size > m1 {
            m3 = m2;
            m2 = m1;
            m1 = size;
        } else if size > m2 {
            m3 = m2;
            m2 = size;
        } else if size > m3 {
            m3 = size;
        }
    }
    m1 * m2 * m3
}

pub fn main() {
    let data = std::fs::read_to_string("data/2021/day9").unwrap();
    let mut heightmap = data
        .lines()
        .map(|line| {
            let mut row = line.as_bytes().iter().map(|e| e - b'0').collect::<Vec<_>>();
            row.push(10);
            row.insert(0, 10);
            row
        })
        .collect::<Vec<_>>();
    let w = heightmap[0].len();
    heightmap.insert(0, vec![10; w]);
    heightmap.push(vec![10; w]);

    let (p1, basins) = part1(&heightmap);
    println!("day9 part1: {p1}");
    println!("day9 part2: {}", part2(&mut heightmap, &basins));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "2199943210
        3987894921
        9856789892
        8767896789
        9899965678";
        let mut heightmap = data
            .lines()
            .map(|line| {
                let mut row = line
                    .trim()
                    .as_bytes()
                    .iter()
                    .map(|e| e - b'0')
                    .collect::<Vec<_>>();
                row.push(10);
                row.insert(0, 10);
                row
            })
            .collect::<Vec<_>>();
        let w = heightmap[0].len();
        heightmap.insert(0, vec![10; w]);
        heightmap.push(vec![10; w]);

        let (p1, basins) = part1(&heightmap);
        assert_eq!(4, basins.len());
        assert_eq!(15, p1);
        assert_eq!(1134, part2(&mut heightmap, &basins));
    }
}
