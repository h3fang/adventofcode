use std::collections::HashSet;
use std::collections::VecDeque;

enum Player {
    P1,
    P2,
}

fn parse(content: &str) -> (VecDeque<usize>, VecDeque<usize>) {
    let mut lines = content.lines();

    assert_eq!("Player 1:", lines.next().unwrap());

    let deck1: VecDeque<usize> = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| line.parse().unwrap())
        .collect();

    assert_eq!("Player 2:", lines.next().unwrap());

    let deck2: VecDeque<usize> = lines.map(|line| line.parse().unwrap()).collect();

    (deck1, deck2)
}

fn evaluate(deck1: &VecDeque<usize>, deck2: &VecDeque<usize>) -> usize {
    let deck = if deck1.is_empty() { deck2 } else { deck1 };
    deck.iter()
        .rev()
        .enumerate()
        .map(|(i, c)| (i + 1) * c)
        .sum()
}

fn recursive_combat(deck1: &mut VecDeque<usize>, deck2: &mut VecDeque<usize>) -> Player {
    let mut rounds = HashSet::new();
    while !(deck1.is_empty() || deck2.is_empty()) {
        let round = (
            deck1.iter().cloned().collect::<Vec<_>>(),
            deck2.iter().cloned().collect::<Vec<_>>(),
        );
        if rounds.contains(&round) {
            return Player::P1;
        }

        rounds.insert(round);

        let c1 = deck1.pop_front().unwrap();
        let c2 = deck2.pop_front().unwrap();

        let winner = if deck1.len() >= c1 && deck2.len() >= c2 {
            let mut d1 = deck1.range(..c1).copied().collect::<VecDeque<_>>();
            let mut d2 = deck2.range(..c2).copied().collect::<VecDeque<_>>();
            recursive_combat(&mut d1, &mut d2)
        } else if c1 > c2 {
            Player::P1
        } else {
            Player::P2
        };

        match winner {
            Player::P1 => {
                deck1.push_back(c1);
                deck1.push_back(c2);
            }
            Player::P2 => {
                deck2.push_back(c2);
                deck2.push_back(c1);
            }
        }
    }

    if deck1.is_empty() {
        Player::P2
    } else {
        Player::P1
    }
}

fn part1(mut deck1: VecDeque<usize>, mut deck2: VecDeque<usize>) -> usize {
    while !(deck1.is_empty() || deck2.is_empty()) {
        let c1 = deck1.pop_front().unwrap();
        let c2 = deck2.pop_front().unwrap();

        if c1 > c2 {
            deck1.push_back(c1);
            deck1.push_back(c2);
        } else {
            deck2.push_back(c2);
            deck2.push_back(c1);
        }
    }
    evaluate(&deck1, &deck2)
}

fn part2(mut deck1: VecDeque<usize>, mut deck2: VecDeque<usize>) -> usize {
    recursive_combat(&mut deck1, &mut deck2);
    evaluate(&deck1, &deck2)
}

pub fn main() {
    let content = std::fs::read_to_string("data/2020/day22").unwrap();
    let (deck1, deck2) = parse(&content);
    // part 1
    println!(
        "day 22 part1: {}",
        part1(deck1.to_owned(), deck2.to_owned())
    );

    // part 2
    println!("day 22 part2: {}", part2(deck1, deck2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_input1() {
        let content = std::fs::read_to_string("data/2020/day22-1").unwrap();
        let (deck1, deck2) = parse(&content);
        assert_eq!(306, part1(deck1.to_owned(), deck2.to_owned()));
        let (deck1, deck2) = parse(&content);
        assert_eq!(291, part2(deck1, deck2));
    }
}
