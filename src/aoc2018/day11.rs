fn parse(data: &str) -> i32 {
    data.trim_end().parse().unwrap()
}

fn power_level(serial: i32, x: i32, y: i32) -> i32 {
    let id = x + 10;
    let p = (id * y + serial) * id;
    p % 1000 / 100 - 5
}

fn grid(serial: i32) -> Vec<Vec<i32>> {
    (0..300)
        .map(|x| (0..300).map(|y| power_level(serial, x, y)).collect())
        .collect()
}

fn part1(g: &[Vec<i32>]) -> (i32, i32) {
    (1..300 - 1)
        .flat_map(|x| {
            (1..300 - 1).map(move |y| {
                let power = (x - 1..=x + 1)
                    .flat_map(|x1| (y - 1..=y + 1).map(move |y1| g[x1 as usize][y1 as usize]))
                    .sum::<i32>();
                (power, (x - 1, y - 1))
            })
        })
        .max_by_key(|e| e.0)
        .map(|e| e.1)
        .unwrap()
}

fn part2(g: &[Vec<i32>]) -> (i32, i32, i32) {
    let mut presum = vec![vec![0; 300]; 300];
    presum[0][0] = g[0][0];
    for j in 1..300 {
        presum[0][j] = presum[0][j - 1] + g[0][j];
    }
    for i in 1..300 {
        let mut sum = 0;
        for j in 1..300 {
            sum += g[i][j];
            presum[i][j] = presum[i - 1][j] + sum;
        }
    }

    let query = |x: i32, y: i32| {
        if x < 0 || y < 0 {
            0
        } else {
            presum[x as usize][y as usize]
        }
    };

    let rect = |x: i32, y: i32, k: i32| {
        query(x + k, y + k) - query(x - 1, y + k) - query(x + k, y - 1) + query(x - 1, y - 1)
    };

    let mut max = i32::MIN;
    let mut result = (0, 0, 0);
    for x in 0..300 {
        for y in 0..300 {
            for k in 0..(300 - x).min(300 - y) {
                let s = rect(x, y, k);
                if s > max {
                    max = s;
                    result = (x, y, k + 1);
                }
            }
        }
    }
    result
}

pub fn main() {
    let data = std::fs::read_to_string("data/2018/day11").unwrap();
    let serial = parse(&data);
    let g = grid(serial);
    let p1 = part1(&g);
    let p2 = part2(&g);
    println!("part1: {},{}", p1.0, p1.1);
    println!("part2: {},{},{}", p2.0, p2.1, p2.2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_level() {
        assert_eq!(4, power_level(8, 3, 5));
        assert_eq!(-5, power_level(57, 122, 79));
        assert_eq!(0, power_level(39, 217, 196));
        assert_eq!(4, power_level(71, 101, 153));
    }

    #[test]
    fn case1() {
        let g = grid(18);
        assert_eq!((33, 45), part1(&g));
        assert_eq!((90, 269, 16), part2(&g));
    }

    #[test]
    fn case2() {
        let g = grid(42);
        assert_eq!((21, 61), part1(&g));
        assert_eq!((232, 251, 12), part2(&g));
    }
}
