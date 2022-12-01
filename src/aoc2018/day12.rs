use std::collections::VecDeque;

fn parse(data: &str) -> (Vec<bool>, [bool; 32]) {
    let mut lines = data.lines();

    let first_line = lines.next().unwrap();
    let initial = first_line.trim().split_ascii_whitespace().last().unwrap();
    let initial = initial.as_bytes().iter().map(|&b| b == b'#').collect();

    lines.next();

    let mut rules = [false; 32];
    lines.for_each(|line| {
        let (pattern, outcome) = line.trim().split_once(" => ").unwrap();
        if outcome == "." {
            return;
        }
        let pattern = pattern
            .as_bytes()
            .iter()
            .enumerate()
            .fold(
                0,
                |acc, (i, b)| {
                    if *b == b'#' {
                        acc | (1 << i)
                    } else {
                        acc
                    }
                },
            );
        rules[pattern] = true;
    });
    (initial, rules)
}

fn step(pots: &[bool], rules: &[bool]) -> Vec<bool> {
    let mut next = pots.to_vec();
    for (i, n) in next.iter_mut().enumerate().take(pots.len() - 2).skip(2) {
        let mut pattern = 0;
        for (j, &pot) in pots[i - 2..=i + 2].iter().enumerate() {
            if pot {
                pattern |= 1 << j;
            }
        }
        *n = rules[pattern];
    }
    next
}

// fn print_pots(pots: &[bool]) {
//     let s = pots
//         .iter()
//         .map(|&p| if p { b'#' } else { b'.' })
//         .collect::<Vec<_>>();
//     println!("{}", unsafe { std::str::from_utf8_unchecked(&s) });
// }

fn sum_of_numbers(pots: &[bool], days: usize) -> i64 {
    pots.iter()
        .enumerate()
        .filter_map(|(i, &p)| {
            if p {
                Some(i as i64 - 2 * (days as i64))
            } else {
                None
            }
        })
        .sum()
}

fn part1(initial: &[bool], rules: &[bool], days: usize) -> i64 {
    let mut pots = Vec::with_capacity(initial.len() + 4 * days);
    pots.extend(vec![false; 2 * days]);
    pots.extend(initial);
    pots.extend(vec![false; 2 * days]);
    for _ in 0..days {
        pots = step(&pots, rules);
        // print_pots(&pots);
    }
    sum_of_numbers(&pots, days)
}

fn part2(initial: &[bool], rules: &[bool]) -> i64 {
    let days = 1000;
    let mut pots = Vec::with_capacity(initial.len() + 4 * days);
    pots.extend(vec![false; 2 * days]);
    pots.extend(initial);
    pots.extend(vec![false; 2 * days]);

    let mut q = VecDeque::new();
    q.push_front(sum_of_numbers(&pots, days));
    for i in 0..days {
        pots = step(&pots, rules);
        q.push_front(sum_of_numbers(&pots, days));
        if q.len() > 3 && q[0] - q[1] == q[1] - q[2] && q[0] - q[1] == q[2] - q[3] {
            return (500_0000_0000 - i as i64 - 1) * (q[0] - q[1]) + q[0];
        }
        if q.len() > 3 {
            q.pop_back();
        }
    }
    panic!("did not converge")
}

pub fn main() {
    let data = std::fs::read_to_string("data/2018/day12").unwrap();
    let (initial, rules) = parse(&data);
    println!("part1: {:?}", part1(&initial, &rules, 20));
    println!("part2: {:?}", part2(&initial, &rules));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #"
            .to_string();
        let (initial, rules) = parse(&data);
        assert_eq!(325, part1(&initial, &rules, 20));
    }
}
