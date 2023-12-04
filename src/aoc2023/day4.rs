fn parse(data: &str) -> Vec<usize> {
    data.trim()
        .lines()
        .map(|line| {
            let lists = line.split_once(": ").unwrap().1;
            let (targets, numbers) = lists.split_once(" | ").unwrap();
            let mut winning = [false; 100];
            for n in targets.split(' ') {
                if let Ok(n) = n.parse::<u32>() {
                    winning[n as usize] = true;
                }
            }
            numbers
                .split(' ')
                .filter(|n| n.parse::<usize>().is_ok_and(|n| winning[n]))
                .count()
        })
        .collect()
}

fn part1(input: &[usize]) -> u32 {
    input
        .iter()
        .map(|&m| if m == 0 { 0 } else { 2u32.pow(m as u32 - 1) })
        .sum()
}

fn part2(input: &[usize]) -> usize {
    let mut cards = vec![1; input.len()];
    for (i, m) in input.iter().enumerate() {
        let curr = cards[i];
        cards[i + 1..=i + m].iter_mut().for_each(|e| *e += curr);
    }
    cards.into_iter().sum()
}

pub fn main() {
    let data = std::fs::read_to_string("data/2023/day4").unwrap();
    let input = parse(&data);
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let input = parse(data);
        assert_eq!(13, part1(&input));
        assert_eq!(30, part2(&input));
    }
}
