use std::collections::VecDeque;

fn parse(data: &str) -> &str {
    data.trim()
}

fn hash(s: &str) -> usize {
    s.as_bytes()
        .iter()
        .fold(0, |curr, &b| ((curr + b as usize) * 17) % 256)
}

fn part1(sequence: &str) -> usize {
    sequence.split(',').map(hash).sum()
}

fn part2(sequence: &str) -> usize {
    let mut boxes: Vec<VecDeque<(&str, u8)>> = (0..256).map(|_| VecDeque::new()).collect();
    sequence.split(',').for_each(|s| {
        let j = s.chars().position(|c| c == '=' || c == '-').unwrap();
        let operation = s.as_bytes()[j];
        let label = &s[..j];
        let i = hash(label);
        if operation == b'-' {
            boxes[i] = boxes[i].drain(..).filter(|&(l, _)| l != label).collect();
        } else {
            let f: u8 = s[j+1..].parse().unwrap();
            if let Some(k) = boxes[i].iter().position(|&(l, _)| l == label) {
                boxes[i][k].1 = f;
            } else {
                boxes[i].push_back((label, f));
            }
        }
    });
    boxes
        .into_iter()
        .enumerate()
        .map(|(i, b)| {
            (i+1) * b
                .into_iter()
                .enumerate()
                .map(|(j, (_, f))| (j + 1) * (f as usize))
                .sum::<usize>()
        })
        .sum()
}

pub fn main() {
    let data = std::fs::read_to_string("data/2023/day15").unwrap();
    let sequence = parse(&data);
    println!("part1: {}", part1(sequence));
    println!("part2: {}", part2(sequence));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let sequence = parse(data);
        assert_eq!(1320, part1(sequence));
        assert_eq!(145, part2(sequence));
    }
}
