use std::collections::VecDeque;

fn parse(data: &str) -> (usize, usize) {
    let parts = data.split_ascii_whitespace().collect::<Vec<_>>();
    let players = parts[0].parse().unwrap();
    let worth = parts[6].parse().unwrap();
    (players, worth)
}

fn part1(players: usize, worth: usize) -> usize {
    let mut scores = vec![0; players];
    let mut q = VecDeque::new();
    q.push_back(0);
    let mut i = 0;
    let mut min = 1;
    loop {
        if min % 23 == 0 {
            scores[i] += min;
            q.rotate_left(7 % q.len());
            scores[i] += q.pop_back().unwrap();
        } else {
            q.rotate_right(2 % q.len());
            q.push_back(min);
        }
        if min == worth {
            break;
        }
        min += 1;
        i = (i + 1) % players;
    }
    scores.into_iter().max().unwrap()
}

pub fn main() {
    let data = std::fs::read_to_string("data/2018/day9").unwrap();
    let (players, worth) = parse(&data);
    println!("part1: {}", part1(players, worth));
    println!("part2: {}", part1(players, worth * 100));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "9 players; last marble is worth 25 points".to_string();
        let (players, worth) = parse(&data);
        assert_eq!(32, part1(players, worth));
    }

    #[test]
    fn case2() {
        let data = "10 players; last marble is worth 1618 points".to_string();
        let (players, worth) = parse(&data);
        assert_eq!(8317, part1(players, worth));
    }

    #[test]
    fn case3() {
        let data = "13 players; last marble is worth 7999 points".to_string();
        let (players, worth) = parse(&data);
        assert_eq!(146373, part1(players, worth));
    }

    #[test]
    fn case4() {
        let data = "17 players; last marble is worth 1104 points".to_string();
        let (players, worth) = parse(&data);
        assert_eq!(2764, part1(players, worth));
    }

    #[test]
    fn case5() {
        let data = "21 players; last marble is worth 6111 points".to_string();
        let (players, worth) = parse(&data);
        assert_eq!(54718, part1(players, worth));
    }

    #[test]
    fn case6() {
        let data = "30 players; last marble is worth 5807 points".to_string();
        let (players, worth) = parse(&data);
        assert_eq!(37305, part1(players, worth));
    }
}
