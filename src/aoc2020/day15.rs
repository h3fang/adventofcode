fn parse(content: &str) -> Vec<u32> {
    content
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|token| token.parse().unwrap())
        .collect::<Vec<_>>()
}

fn part(starting_nums: &[u32], target_pos: usize) -> u32 {
    let mut map = vec![u32::MAX; target_pos];
    let mut last = *starting_nums.first().unwrap();
    for (i, n) in starting_nums.iter().skip(1).enumerate() {
        map[last as usize] = i as u32;
        last = *n;
    }

    for i in starting_nums.len()..target_pos {
        let i = i as u32;
        let turn = map[last as usize];
        let next = if turn == u32::MAX { 0 } else { i - 1 - turn };
        map[last as usize] = i - 1;

        last = next;
    }
    last
}

pub fn main() {
    let starting_nums = parse(&std::fs::read_to_string("data/2020/day15").unwrap());

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
        let starting_nums = parse(&std::fs::read_to_string("data/2020/day15-1").unwrap());
        assert_eq!(436, part(&starting_nums, 2020));

        let starting_nums = parse(&std::fs::read_to_string("data/2020/day15-2").unwrap());
        assert_eq!(1836, part(&starting_nums, 2020));
    }
}
