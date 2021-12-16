use ahash::AHashSet as HashSet;

enum Player {
    P1,
    P2,
}

fn parse(content: &str) -> (Vec<u8>, Vec<u8>) {
    let mut lines = content.lines();

    assert_eq!("Player 1:", lines.next().unwrap());

    let deck1: Vec<u8> = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| line.parse().unwrap())
        .collect();

    assert_eq!("Player 2:", lines.next().unwrap());

    let deck2: Vec<u8> = lines.map(|line| line.parse().unwrap()).collect();

    (deck1, deck2)
}

fn score(deck: &[u8]) -> usize {
    deck.iter()
        .rev()
        .enumerate()
        .map(|(i, &c)| (i + 1) * c as usize)
        .sum()
}

fn evaluate(deck1: &[u8], deck2: &[u8]) -> usize {
    let deck = if deck1.is_empty() { deck2 } else { deck1 };
    score(deck)
}

fn hash(deck1: &[u8], deck2: &[u8]) -> (usize, usize) {
    (score(deck1), score(deck2))
}

fn recursive_combat(deck1: &mut Vec<u8>, deck2: &mut Vec<u8>) -> Player {
    let mut rounds = HashSet::new();
    while !(deck1.is_empty() || deck2.is_empty()) {
        let round = hash(deck1, deck2);
        if rounds.contains(&round) {
            return Player::P1;
        }

        rounds.insert(round);

        let c1 = deck1.remove(0) as usize;
        let c2 = deck2.remove(0) as usize;

        let winner = if deck1.len() >= c1 && deck2.len() >= c2 {
            let mut d1 = deck1[..c1].to_vec();
            let mut d2 = deck2[..c2].to_vec();
            recursive_combat(&mut d1, &mut d2)
        } else if c1 > c2 {
            Player::P1
        } else {
            Player::P2
        };

        match winner {
            Player::P1 => {
                deck1.push(c1 as u8);
                deck1.push(c2 as u8);
            }
            Player::P2 => {
                deck2.push(c2 as u8);
                deck2.push(c1 as u8);
            }
        }
    }

    if deck1.is_empty() {
        Player::P2
    } else {
        Player::P1
    }
}

fn part1(mut deck1: Vec<u8>, mut deck2: Vec<u8>) -> usize {
    while !(deck1.is_empty() || deck2.is_empty()) {
        let c1 = deck1.remove(0);
        let c2 = deck2.remove(0);

        if c1 > c2 {
            deck1.push(c1);
            deck1.push(c2);
        } else {
            deck2.push(c2);
            deck2.push(c1);
        }
    }
    evaluate(&deck1, &deck2)
}

fn part2(mut deck1: Vec<u8>, mut deck2: Vec<u8>) -> usize {
    recursive_combat(&mut deck1, &mut deck2);
    evaluate(&deck1, &deck2)
}

pub fn main() {
    let content = std::fs::read_to_string("data/2020/day22").unwrap();
    let (deck1, deck2) = parse(&content);
    // part 1
    println!("day 22 part1: {}", part1(deck1.clone(), deck2.clone()));

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
