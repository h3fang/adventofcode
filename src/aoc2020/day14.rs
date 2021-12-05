use std::collections::HashMap;

#[derive(Clone)]
enum Instruction {
    Mask(String),
    Mem { addr: usize, value: usize },
}

fn mask_part1(s: &str) -> (usize, usize) {
    let mut mask = 0;
    let mut value = 0;
    s.chars().rev().enumerate().for_each(|(i, m)| match m {
        'X' => mask |= 1 << i,
        '0' => {}
        '1' => {
            value |= 1 << i;
        }
        _ => panic!("invalid mask value: {}", s),
    });
    (mask, value)
}

fn gen_addr(addrs: &[usize], i: usize) -> Vec<usize> {
    let mut r = addrs.to_vec();
    r.iter_mut().for_each(|addr| {
        *addr |= 1 << i;
    });
    r.extend(addrs);
    r
}

fn mask_part2(s: &str) -> (usize, usize, Vec<usize>) {
    let mut mask = 0;
    let mut value = 0;
    let mut xs = Vec::new();
    s.chars().rev().enumerate().for_each(|(i, m)| match m {
        'X' => {
            xs.push(i);
        }
        '0' => mask |= 1 << i,
        '1' => {
            mask |= 1 << i;
            value |= 1 << i;
        }
        _ => panic!("invalid mask value: {}", s),
    });
    let mut addrs = vec![0];
    for i in xs {
        addrs = gen_addr(&addrs, i);
    }
    (mask, value, addrs)
}

fn parse(content: &str) -> Vec<Instruction> {
    content
        .lines()
        .map(|line| {
            if &line[0..4] == "mask" {
                Instruction::Mask(line[7..].to_string())
            } else if &line[0..4] == "mem[" {
                let mut tokens = line[4..].split("] = ");
                let addr = tokens.next().unwrap().parse().unwrap();
                let value = tokens.next().unwrap().parse().unwrap();
                Instruction::Mem { addr, value }
            } else {
                panic!("invalid line: {}", line);
            }
        })
        .collect::<Vec<_>>()
}

fn part1(instructions: &[Instruction]) -> usize {
    let mut map = HashMap::new();
    let mut m: Option<(usize, usize)> = None;
    instructions.iter().for_each(|ins| match ins {
        Instruction::Mask(mask) => {
            m = Some(mask_part1(mask));
        }
        Instruction::Mem { addr, value } => {
            let mut v = *value;
            if let Some((mask, value)) = m {
                v &= mask;
                v |= value;
            }
            map.insert(addr, v);
        }
    });

    map.values().sum()
}

fn part2(instructions: &[Instruction]) -> usize {
    let mut map = HashMap::new();
    let mut m: Option<(usize, usize, Vec<usize>)> = None;
    instructions.iter().for_each(|ins| match ins {
        Instruction::Mask(mask) => {
            m = Some(mask_part2(mask));
        }
        Instruction::Mem { addr, value } => {
            if let Some((mask, keep, addrs)) = &m {
                for f in addrs {
                    let a = addr & mask | keep | f;
                    map.insert(a, *value);
                }
            } else {
                panic!("no mask when write to memory");
            }
        }
    });

    map.values().sum()
}

pub fn main() {
    let instructions = parse(&std::fs::read_to_string("data/2020/day14").unwrap());

    // part 1
    println!("day 14 part1: {}", part1(&instructions));

    // part 2
    println!("day 14 part2: {}", part2(&instructions));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let instructions = parse(&std::fs::read_to_string("data/2020/day14-1").unwrap());
        assert_eq!(165, part1(&instructions));
    }

    #[test]
    fn test_mask2() {
        let s = "000000000000000000000000000000X1001X";
        let (mask, keep, mut addrs) = mask_part2(s);
        let addr = 42;
        for f in addrs.iter_mut() {
            *f |= addr & mask | keep;
        }
        addrs.sort_unstable();
        assert_eq!(addrs, vec![26, 27, 58, 59]);
    }

    #[test]
    fn test_part2() {
        let instructions = parse(&std::fs::read_to_string("data/2020/day14-2").unwrap());
        assert_eq!(208, part2(&instructions));
    }
}
