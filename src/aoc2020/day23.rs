fn parse(content: &str) -> (Vec<u32>, Vec<u32>) {
    let nums = content
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect::<Vec<_>>();
    let mut r1 = vec![0; nums.len() + 1];
    let mut prev = 0;
    for &n in &nums {
        r1[prev] = n;
        prev = n as usize;
    }
    r1[prev] = r1[0];

    let mut r2 = r1.clone();
    let one_million = 1000000;
    r2[prev] = nums.len() as u32 + 1;
    r2.reserve(one_million + 1);
    for i in nums.len() + 1..one_million {
        r2.push(i as u32 + 1);
    }
    r2.push(r2[0]);

    (r1, r2)
}

// fn print_cups(c: &[u32]) {
//     let mut next = c[0];
//     loop {
//         print!("{}", next);
//         next = c[next];
//         if next == c[0] {
//             break;
//         }
//     }
// }

fn make_moves(cups: &mut [u32], moves: u32) {
    let n = cups.len();
    let mut current = cups[0] as usize;
    for _ in 0..moves {
        // print_cups(c);

        let c1 = cups[current] as usize;
        let c2 = cups[c1] as usize;
        let c3 = cups[c2] as usize;

        // print!(" ({},{},{})", c1, c2 ,c3);

        // find destination
        let mut dest = current - 1;
        loop {
            if dest == 0 {
                dest = n;
            } else if !(dest == c1 || dest == c2 || dest == c3) {
                break;
            }
            dest -= 1;
        }

        // println!(" {}", dest);

        let t = cups[dest];
        cups[dest] = c1 as u32;
        cups[current] = cups[c3];
        cups[c3] = t;

        current = cups[current] as usize;
    }
}

fn part1(cups: &mut [u32]) -> usize {
    make_moves(cups, 100);

    // result
    let mut c = 1;
    let mut r = 0;
    while cups[c] != 1 {
        c = cups[c] as usize;
        r = r * 10 + c;
    }
    r
}

fn part2(cups: &mut [u32]) -> usize {
    make_moves(cups, 10000000);

    // result
    cups[1] as usize * cups[cups[1] as usize] as usize
}

pub fn main() {
    let content = std::fs::read_to_string("data/2020/day23").unwrap();
    let mut cups = parse(&content);

    // part 1
    println!("day 23 part1: {}", part1(&mut cups.0));

    // part 2
    println!("day 23 part2: {}", part2(&mut cups.1));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input1() {
        let mut cups = parse("389125467");

        // part 1
        assert_eq!(67384529, part1(&mut cups.0));

        // part 2
        assert_eq!(149245887792, part2(&mut cups.1));
    }
}
