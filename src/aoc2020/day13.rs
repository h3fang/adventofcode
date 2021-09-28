fn parse(content: &str) -> (usize, Vec<Option<usize>>) {
    let mut lines = content.lines();
    let timestamp = lines.next().unwrap().parse::<usize>().unwrap();
    let buses = lines
        .next()
        .unwrap()
        .split(',')
        .map(|e| {
            if let Ok(n) = e.parse::<usize>() {
                Some(n)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    (timestamp, buses)
}

fn part1(timestamp: usize, buses: &[Option<usize>]) -> usize {
    let wait_time = buses
        .iter()
        .flatten()
        .map(|&b| {
            let remaining = timestamp % b;
            if remaining > 0 {
                (b, b - remaining)
            } else {
                (b, 0)
            }
        })
        .min_by_key(|e| e.1)
        .unwrap();
    wait_time.0 * wait_time.1
}

fn part2(_timestamp: usize, buses: &[Option<usize>]) -> usize {
    let buses = buses.iter().enumerate().collect::<Vec<_>>();
    let mut sorted = buses
        .iter()
        .filter_map(|&(d, b)| b.as_ref().map(|&b| (d, b)))
        .collect::<Vec<_>>();
    sorted.sort_unstable_by_key(|(_, b)| *b);
    let (d_max, b_max) = *sorted.last().unwrap();
    let mut t = b_max - d_max;
    let mut step = 1;
    sorted.iter().rev().for_each(|&(d, b)| {
        while (t + d) % b != 0 {
            t += step;
        }
        step *= b;
    });
    t
}

pub fn main() {
    let (timestamp, buses) = parse(&std::fs::read_to_string("data/2020/day13").unwrap());

    // part 1
    println!("day 13 part1: {}", part1(timestamp, &buses));

    // part 2
    println!("day 13 part2: {}", part2(timestamp, &buses));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let (timestamp, buses) = parse(&std::fs::read_to_string("data/2020/day13-1").unwrap());
        assert_eq!(1068781, part2(timestamp, &buses));

        let (timestamp, buses) = parse(&std::fs::read_to_string("data/2020/day13-2").unwrap());
        assert_eq!(1202161486, part2(timestamp, &buses));
    }
}
