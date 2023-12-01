use ahash::AHashMap as HashMap;
use std::str::FromStr;

#[derive(Debug, Clone)]
enum Rule {
    Exact(char),
    Any(Vec<Vec<usize>>),
}

#[derive(Debug)]
struct ParseRuleError;

impl FromStr for Rule {
    type Err = ParseRuleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.as_bytes()[0] == b'"' && s.as_bytes()[2] == b'"' {
            Ok(Rule::Exact(s.as_bytes()[1] as char))
        } else {
            let rules = s
                .split(" | ")
                .map(|part| {
                    part.split(' ')
                        .map(|n| n.parse().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            Ok(Rule::Any(rules))
        }
    }
}

fn parse(content: &str) -> (Vec<Rule>, Vec<&str>) {
    let mut lines = content.lines();
    let mut map: HashMap<usize, Rule> = HashMap::new();
    for line in &mut lines {
        if line.is_empty() {
            break;
        }

        let mut parts = line.split(": ");
        let num = parts.next().unwrap().parse().unwrap();
        let rule = parts.next().unwrap().parse().unwrap();
        map.insert(num, rule);
    }

    let messages = lines.collect::<Vec<_>>();
    let max_rules = map.keys().max().unwrap();
    let mut rules = Vec::with_capacity(max_rules + 1);
    for i in 0..=*max_rules {
        if let Some(r) = map.get(&i) {
            rules.push(r.clone());
        } else {
            rules.push(Rule::Exact('❓'));
        }
    }
    (rules, messages)
}

fn check_rule(rules: &[Rule], rule: usize, msg: &str, stack: &mut Vec<usize>) -> bool {
    match &rules[rule] {
        Rule::Exact(c) => {
            if let Some(m) = msg.chars().next() {
                if *c != m {
                    return false;
                }
            } else {
                return false;
            }

            if let Some(r) = stack.pop() {
                check_rule(rules, r, &msg[1..], stack)
            } else {
                msg.len() == 1
            }
        }
        Rule::Any(groups) => groups.iter().any(|group| {
            let mut stack = stack.clone();
            for &r in group.iter().rev() {
                stack.push(r);
            }
            if let Some(r) = stack.pop() {
                check_rule(rules, r, msg, &mut stack)
            } else {
                panic!("empty stack in Rule::Any");
            }
        }),
    }
}

fn part1(rules: &[Rule], messages: &[&str]) -> usize {
    let mut stack = Vec::new();
    messages
        .iter()
        .filter(|&&msg| check_rule(rules, 0, msg, &mut stack))
        .count()
}

pub fn main() {
    let content = std::fs::read_to_string("data/2020/day19").unwrap();
    let (mut rules, messages) = parse(&content);

    // part 1
    println!("day 19 part1: {}", part1(&rules, &messages));

    // part 2
    rules[8] = Rule::Any(vec![vec![42], vec![42, 8]]);
    rules[11] = Rule::Any(vec![vec![42, 31], vec![42, 11, 31]]);
    println!("day 19 part2: {}", part1(&rules, &messages));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let content = std::fs::read_to_string("data/2020/day19-1").unwrap();
        let (rules, messages) = parse(&content);

        // part 1
        assert_eq!(2, part1(&rules, &messages));
    }
}
