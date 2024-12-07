const MODULO: usize = 20201227;
const SUBJECT: usize = 7;

fn parse(content: &str) -> (usize, usize) {
    let nums = content
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse().unwrap())
        .collect::<Vec<_>>();
    (nums[0], nums[1])
}

#[inline]
fn round(value: usize, subject: usize) -> usize {
    (value * subject) % MODULO
}

fn loop_size(pubkey: usize) -> usize {
    let mut s = 1;
    let mut v = 1;
    loop {
        v = round(v, SUBJECT);
        if v == pubkey {
            return s;
        }
        s += 1;
    }
}

fn part1(card_pub: usize, door_pub: usize) -> usize {
    let card_loop = loop_size(card_pub);
    // let door_loop = loop_size(door_pub);
    let mut v = 1;
    for _ in 0..card_loop {
        v = round(v, door_pub);
    }
    v
}

pub fn main() {
    let content = std::fs::read_to_string("data/2020/day25").unwrap();
    let (card_pub, door_pub) = parse(&content);

    // part 1
    println!("day 25 part1: {}", part1(card_pub, door_pub));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input1() {
        let content = "5764801\n17807724";
        let (card_pub, door_pub) = parse(content);

        // part 1
        assert_eq!(14897079, part1(card_pub, door_pub));
    }
}
