#[derive(Debug, Clone, Copy)]
enum Arg {
    Old,
    Num(u64),
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add(Arg),
    Mul(Arg),
}

#[derive(Debug, Clone)]
struct Test {
    divisible: u64,
    if_true: usize,
    if_false: usize,
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
    op: Operation,
    test: Test,
    inspected: u64,
}

fn parse(data: &str) -> Vec<Monkey> {
    data.trim()
        .split("\n\n")
        .map(|m| {
            let mut lines = m.lines();
            lines.next();
            let items = lines
                .next()
                .unwrap()
                .trim_start_matches("  Starting items: ")
                .split(", ")
                .map(|i| i.parse().unwrap())
                .collect();
            let op = lines
                .next()
                .unwrap()
                .trim_start_matches("  Operation: new = old ");
            let arg = if &op[2..] == "old" {
                Arg::Old
            } else {
                Arg::Num(op[2..].parse().unwrap())
            };
            let op = match op.as_bytes()[0] {
                b'+' => Operation::Add(arg),
                b'*' => Operation::Mul(arg),
                _ => unreachable!(),
            };
            let divisible = lines
                .next()
                .unwrap()
                .trim_start_matches("  Test: divisible by ")
                .parse()
                .unwrap();
            let if_true = lines
                .next()
                .unwrap()
                .trim_start_matches("    If true: throw to monkey ")
                .parse()
                .unwrap();
            let if_false = lines
                .next()
                .unwrap()
                .trim_start_matches("    If false: throw to monkey ")
                .parse()
                .unwrap();
            Monkey {
                items,
                op,
                test: Test {
                    divisible,
                    if_true,
                    if_false,
                },
                inspected: 0,
            }
        })
        .collect()
}

fn apply_operation(worry: u64, op: &Operation) -> u64 {
    match op {
        Operation::Add(arg) => {
            worry
                + match arg {
                    Arg::Old => worry,
                    Arg::Num(n) => *n,
                }
        }
        Operation::Mul(arg) => {
            worry
                * match arg {
                    Arg::Old => worry,
                    Arg::Num(n) => *n,
                }
        }
    }
}

fn take_round1(monkeys: &mut [Monkey]) {
    for i in 0..monkeys.len() {
        monkeys[i].inspected += monkeys[i].items.len() as u64;
        std::mem::take(&mut monkeys[i].items)
            .into_iter()
            .for_each(|item| {
                let worry = apply_operation(item, &monkeys[i].op) / 3;
                if worry % monkeys[i].test.divisible == 0 {
                    monkeys[monkeys[i].test.if_true].items.push(worry);
                } else {
                    monkeys[monkeys[i].test.if_false].items.push(worry);
                }
            });
    }
}

fn take_round2(monkeys: &mut [Monkey], modulus: u64) {
    for i in 0..monkeys.len() {
        monkeys[i].inspected += monkeys[i].items.len() as u64;
        std::mem::take(&mut monkeys[i].items)
            .into_iter()
            .for_each(|item| {
                let worry = apply_operation(item, &monkeys[i].op) % modulus;
                if worry % monkeys[i].test.divisible == 0 {
                    monkeys[monkeys[i].test.if_true].items.push(worry);
                } else {
                    monkeys[monkeys[i].test.if_false].items.push(worry);
                }
            });
    }
}

fn monkey_business(monkeys: &[Monkey]) -> u64 {
    let mut max1 = 0;
    let mut max2 = 0;
    monkeys.iter().for_each(|m| {
        if m.inspected > max1 {
            max2 = max1;
            max1 = m.inspected;
        } else if m.inspected > max2 {
            max2 = m.inspected;
        }
    });
    max1 * max2
}

fn part1(mut monkeys: Vec<Monkey>) -> u64 {
    for _ in 0..20 {
        take_round1(&mut monkeys);
    }
    monkey_business(&monkeys)
}

fn part2(mut monkeys: Vec<Monkey>) -> u64 {
    let modulus = monkeys.iter().map(|m| m.test.divisible).product();
    for _ in 0..10000 {
        take_round2(&mut monkeys, modulus);
    }
    monkey_business(&monkeys)
}

pub fn main() {
    let data = std::fs::read_to_string("data/2022/day11").unwrap();
    let monkeys = parse(&data);
    println!("part1: {}", part1(monkeys.clone()));
    println!("part2: {}", part2(monkeys));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
        let monkeys = parse(&data);
        assert_eq!(10605, part1(monkeys.clone()));
        assert_eq!(2713310158, part2(monkeys));
    }
}
