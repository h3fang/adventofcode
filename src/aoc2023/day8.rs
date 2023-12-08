use ahash::HashMap;

fn parse(data: &str) -> (&[u8], HashMap<&str, [&str; 2]>) {
    let (instruction, network) = data.trim().split_once('\n').unwrap();
    let network = network
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (parent, children) = line.split_once(" = (").unwrap();
            let (left, right) = children
                .strip_suffix(')')
                .unwrap()
                .split_once(", ")
                .unwrap();
            (parent, [left, right])
        })
        .collect();
    (instruction.trim().as_bytes(), network)
}

fn part1(instruction: &[u8], network: &HashMap<&str, [&str; 2]>) -> usize {
    let mut curr = "AAA";
    for (i, &c) in instruction.iter().cycle().enumerate() {
        if curr == "ZZZ" {
            return i;
        }
        let ins = &network[curr];
        match c {
            b'L' => curr = ins[0],
            b'R' => curr = ins[1],
            _ => panic!(),
        }
    }
    0
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        (a, b) = (b, a % b)
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn part2(instruction: &[u8], network: &HashMap<&str, [&str; 2]>) -> usize {
    network
        .keys()
        .filter(|k| *k.as_bytes().last().unwrap() == b'A')
        .map(|curr| {
            let mut curr = *curr;
            for (i, &c) in instruction.iter().cycle().enumerate() {
                if curr.ends_with('Z') {
                    return i;
                }
                let ins = &network[curr];
                match c {
                    b'L' => curr = ins[0],
                    b'R' => curr = ins[1],
                    _ => panic!(),
                }
            }
            0
        })
        .fold(1, lcm)
}

pub fn main() {
    let data = std::fs::read_to_string("data/2023/day8").unwrap();
    let (instruction, network) = parse(&data);
    println!("part1: {}", part1(instruction, &network));
    println!("part2: {}", part2(instruction, &network));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        let (instruction, network) = parse(&data);
        assert_eq!(2, part1(instruction, &network));
    }

    #[test]
    fn case2() {
        let data = "
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";
        let (instruction, network) = parse(&data);
        assert_eq!(6, part1(instruction, &network));
    }

    #[test]
    fn case3() {
        let data = "
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";
        let (instruction, network) = parse(&data);
        assert_eq!(6, part2(instruction, &network));
    }
}
