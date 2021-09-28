fn parse(content: &str) -> Vec<usize> {
    content
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|token| token.parse().unwrap())
        .collect::<Vec<_>>()
}

fn part(starting_nums: &[usize], target_pos: usize) -> usize {
    let mut map = vec![usize::MAX; target_pos];
    let mut last = *starting_nums.first().unwrap();
    for (i, n) in starting_nums.iter().skip(1).enumerate() {
        map[last] = i;
        last = *n;
    }

    for i in starting_nums.len()..target_pos {
        let turn = map[last];
        let next = if turn == usize::MAX { 0 } else { i - 1 - turn };
        map[last] = i - 1;

        last = next;
    }
    last
}

pub fn main() {
    let starting_nums = parse(include_str!("../data/day15"));

    // part 1
    println!("day 15 part1: {}", part(&starting_nums, 2020));

    // part 2
    println!("day 15 part2: {}", part(&starting_nums, 30000000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let starting_nums = parse(include_str!("../data/day15-1"));
        assert_eq!(436, part(&starting_nums, 2020));

        let starting_nums = parse(include_str!("../data/day15-2"));
        assert_eq!(1836, part(&starting_nums, 2020));
    }
}
