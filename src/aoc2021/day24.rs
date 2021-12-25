use ahash::AHashSet as HashSet;

fn alu(
    invalid: &mut HashSet<(i64, usize)>,
    insturctions: &[(i64, i64, i64)],
    z: i64,
    i: usize,
    curr: i64,
    wcs: &mut Vec<i64>,
    part2: bool,
) -> i64 {
    if i == 14 {
        return if z == 0 { curr } else { -1 };
    }
    if invalid.contains(&(z, i)) {
        return -1;
    }
    let (a, b, c) = insturctions[i];
    let digits = if part2 {
        [1, 2, 3, 4, 5, 6, 7, 8, 9]
    } else {
        [9, 8, 7, 6, 5, 4, 3, 2, 1]
    };
    let wc = if a == 1 { -1 } else { wcs.pop().unwrap() };
    for w in digits {
        // the instructions in the input decompiles into the following
        // let x = if (z % 26 + b) != w { 1 } else { 0 };
        // let z = z / a * (25 * x + 1) + (w + c) * x;

        // By analyzing the input, we get:
        // a is either 1 or 26, there are 7 a == 1 and 7 a == 26,
        // And there is always a == 1 before a == 26.
        let z = if a == 1 {
            // When a == 1, b > 9, so (z % 16 + b) > w is always true,
            // z = 26 * z + w + c, this looks like "hashing" the digit to base 26,
            // The hash increases.
            wcs.push(w + c);
            z * 26 + w + c
        } else {
            // When a == 26, if the hash want to return to 0,
            // x = if (z % 26 + b) != w { 1 } else { 0 } must be 0,
            // so z = z / 26, makes the hash decrease.
            // To ensure x == 0, the "previous" (w + c) plus b must equal to w.
            if wc + b != w {
                continue;
            }
            z / 26
        };
        let r = alu(invalid, insturctions, z, i + 1, curr * 10 + w, wcs, part2);
        if a == 1 {
            wcs.pop();
        }
        if r != -1 {
            return r;
        }
    }
    if a == 26 {
        wcs.push(wc);
    }
    invalid.insert((z, i));
    -1
}

fn parse(data: &str) -> Vec<(i64, i64, i64)> {
    let mut result = vec![];
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    data.lines().enumerate().for_each(|(i, line)| {
        if i % 18 == 4 {
            a = line.trim().trim_start_matches("div z ").parse().unwrap();
        } else if i % 18 == 5 {
            b = line.trim().trim_start_matches("add x ").parse().unwrap();
        } else if i % 18 == 15 {
            c = line.trim().trim_start_matches("add y ").parse().unwrap();
            result.push((a, b, c));
        }
    });
    result
}

fn solve(insturctions: &[(i64, i64, i64)], part2: bool) -> i64 {
    let mut cache = HashSet::new();
    let mut ws = vec![];
    alu(&mut cache, insturctions, 0, 0, 0, &mut ws, part2)
}

pub fn main() {
    let data = std::fs::read_to_string("data/2021/day24").unwrap();
    let instructions = parse(&data);
    println!("day24 part1: {}", solve(&instructions, false));
    println!("day24 part2: {}", solve(&instructions, true));
}
