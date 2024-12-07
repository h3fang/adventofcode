#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
enum Card1 {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

impl From<u8> for Card1 {
    fn from(c: u8) -> Self {
        match c {
            b'A' => Card1::A,
            b'K' => Card1::K,
            b'Q' => Card1::Q,
            b'J' => Card1::J,
            b'T' => Card1::T,
            b'9' => Card1::Nine,
            b'8' => Card1::Eight,
            b'7' => Card1::Seven,
            b'6' => Card1::Six,
            b'5' => Card1::Five,
            b'4' => Card1::Four,
            b'3' => Card1::Three,
            b'2' => Card1::Two,
            _ => panic!("invald card symbol: {c}"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand1 {
    kind: u8,
    cards: [Card1; 5],
}

impl Hand1 {
    fn new(cards: [Card1; 5]) -> Self {
        let mut freq = [0; 13];
        for c in cards {
            freq[c as usize] += 1;
        }
        let max = freq.into_iter().max().unwrap();
        let kind = match max {
            5 => 6,
            4 => 5,
            3 => {
                let k = freq.into_iter().filter(|&f| f > 0).count();
                if k == 2 {
                    4
                } else {
                    3
                }
            }
            2 => freq.into_iter().filter(|&f| f == 2).count() as u8,
            1 => 0,
            _ => panic!("impossible frequncy for a hand of 5 cards"),
        };
        Self { kind, cards }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
enum Card2 {
    J,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    Q,
    K,
    A,
}

impl From<u8> for Card2 {
    fn from(c: u8) -> Self {
        match c {
            b'A' => Card2::A,
            b'K' => Card2::K,
            b'Q' => Card2::Q,
            b'J' => Card2::J,
            b'T' => Card2::T,
            b'9' => Card2::Nine,
            b'8' => Card2::Eight,
            b'7' => Card2::Seven,
            b'6' => Card2::Six,
            b'5' => Card2::Five,
            b'4' => Card2::Four,
            b'3' => Card2::Three,
            b'2' => Card2::Two,
            _ => panic!("invald card symbol: {c}"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand2 {
    kind: u8,
    cards: [Card2; 5],
}

impl Hand2 {
    fn new(cards: [Card2; 5]) -> Self {
        let mut freq = [0; 13];
        for c in cards {
            freq[c as usize] += 1;
        }
        let max_freq = freq.into_iter().max().unwrap();
        let jokers = freq[0];
        let kind = match max_freq {
            5 => 6,
            4 => {
                if jokers == 4 || jokers == 1 {
                    6
                } else {
                    5
                }
            }
            3 => {
                let k = freq.into_iter().filter(|&f| f > 0).count();
                if k == 2 {
                    if jokers > 0 {
                        6
                    } else {
                        4
                    }
                } else if jokers > 0 {
                    5
                } else {
                    3
                }
            }
            2 => {
                let pairs = freq.into_iter().filter(|&f| f == 2).count();
                if pairs == 2 {
                    match jokers {
                        0 => 2,
                        1 => 4,
                        2 => 5,
                        _ => panic!("number of jokers should be <= 2"),
                    }
                } else {
                    match jokers {
                        0 => 1,
                        1 | 2 => 3,
                        _ => panic!("number of jokers should be <= 2"),
                    }
                }
            }
            1 => jokers as u8,
            _ => panic!("impossible frequncy for a hand of 5 cards"),
        };
        Self { kind, cards }
    }
}

fn parse(data: &str) -> Vec<(&str, u32)> {
    data.trim()
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').unwrap();
            let bid = bid.parse().unwrap();
            (hand, bid)
        })
        .collect()
}

fn part1(input: &[(&str, u32)]) -> u32 {
    let mut input = input
        .iter()
        .map(|(hand, bid)| {
            let h = hand.as_bytes();
            let hand = Hand1::new([
                h[0].into(),
                h[1].into(),
                h[2].into(),
                h[3].into(),
                h[4].into(),
            ]);
            (hand, bid)
        })
        .collect::<Vec<_>>();
    input.sort_unstable();
    input
        .into_iter()
        .enumerate()
        .map(|(i, e)| e.1 * (i as u32 + 1))
        .sum()
}

fn part2(input: &[(&str, u32)]) -> u32 {
    let mut input = input
        .iter()
        .map(|(hand, bid)| {
            let h = hand.as_bytes();
            let hand = Hand2::new([
                h[0].into(),
                h[1].into(),
                h[2].into(),
                h[3].into(),
                h[4].into(),
            ]);
            (hand, bid)
        })
        .collect::<Vec<_>>();
    input.sort_unstable();
    input
        .into_iter()
        .enumerate()
        .map(|(i, e)| e.1 * (i as u32 + 1))
        .sum()
}

pub fn main() {
    let data = std::fs::read_to_string("data/2023/day7").unwrap();
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
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let input = parse(data);
        assert_eq!(6440, part1(&input));
        assert_eq!(5905, part2(&input));
    }
}
