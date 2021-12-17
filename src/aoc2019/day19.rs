use crate::day5::Intcode;

fn solve(codes: &[i64]) -> (i64, i64) {
    fn is_pulled(codes: &[i64], x: i64, y: i64) -> bool {
        let mut prog = Intcode::new(codes);
        prog.inputs = vec![y, x];
        prog.run();
        prog.output == 1
    }
    fn upper_bound(codes: &[i64], y: i64, mut left: i64, mut right: i64) -> i64 {
        while left < right {
            let mid = (left + right + 1) / 2;
            if is_pulled(codes, mid, y) {
                left = mid;
            } else {
                right = mid - 1;
            }
        }
        right
    }
    let mut p1 = 0;
    let mut start = 0;
    let mut min_x = 0;
    let mut max_x = 0;

    // let mut img = vec![];
    for y in 0..50 {
        let left = start;
        for x in left..50 {
            if is_pulled(codes, x, y) {
                let ub = upper_bound(codes, y, x, 49);
                p1 += ub - x + 1;
                // img.push(
                //     ".".repeat(x as usize)
                //         + &"#".repeat((ub - x + 1) as usize)
                //         + &".".repeat((50 - ub - 1) as usize),
                // );
                start = x;
                if y == 49 {
                    min_x = x;
                    max_x = ub;
                }
                break;
            }
        }
        // if img.len() != y as usize + 1 {
        //     img.push(".".repeat(50));
        // }
    }
    // for row in &img {
    //     println!("{}", row);
    // }

    fn find_start(codes: &[i64], mut x_guess: i64, y: i64) -> i64 {
        while is_pulled(codes, x_guess, y) {
            x_guess -= 1;
        }
        while !is_pulled(codes, x_guess, y) {
            x_guess += 1;
        }
        x_guess
    }

    fn find_end(codes: &[i64], mut x_guess: i64, y: i64) -> i64 {
        while is_pulled(codes, x_guess, y) {
            x_guess += 1;
        }
        while !is_pulled(codes, x_guess, y) {
            x_guess -= 1;
        }
        x_guess
    }

    fn is_fit(codes: &[i64], y: i64, max_x: i64, min_x: i64) -> (bool, i64) {
        let guess = (y as f64 / 50.0 * max_x as f64) as i64;
        let end = find_end(codes, guess, y);
        let guess = ((y + 99) as f64 / 50.0 * min_x as f64) as i64;
        let start = find_start(codes, guess, y + 99);
        (start <= end - 99, start)
    }

    let top_guess = ((2 * min_x + 100) as f64 / (max_x - min_x + 1) as f64 * 50.0) as i64;

    let mut top = top_guess - 1000;
    let mut bottom = top_guess + 1000;
    let mut left = 0;
    while top < bottom {
        let mid = (top + bottom) / 2;
        let r = is_fit(codes, mid, max_x, min_x);
        if r.0 {
            bottom = mid;
            left = r.1;
        } else {
            top = mid + 1;
        }
    }

    (p1, left * 10000 + top)
}

pub fn main() {
    let codes = std::fs::read_to_string("data/2019/day19")
        .unwrap()
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|t| t.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let (p1, p2) = solve(&codes);
    println!("day 19 part1: {}", p1);
    println!("day 19 part2: {}", p2);
}
