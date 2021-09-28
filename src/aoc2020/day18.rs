#[derive(PartialEq, Eq, Clone, Debug)]
enum Operator {
    Add,
    Multiply,
}

#[derive(PartialEq, Eq, Clone, Debug)]
enum Node {
    None,
    Value(usize),
    Operator(Operator),
    LeftBracket,
    RightBracket,
}

impl Node {
    fn value(&self) -> usize {
        if let Node::Value(v) = self {
            *v
        } else {
            panic!("node is not Node::Value: {:?}", self)
        }
    }
}

impl From<char> for Node {
    fn from(c: char) -> Self {
        match c {
            ' ' => Node::None,
            '+' => Node::Operator(Operator::Add),
            '*' => Node::Operator(Operator::Multiply),
            '(' => Node::LeftBracket,
            ')' => Node::RightBracket,
            n => {
                if let Ok(op) = n.to_string().parse::<usize>() {
                    Node::Value(op)
                } else {
                    panic!("invalid char in expression: {}", n)
                }
            }
        }
    }
}

fn parse(content: &str) -> Vec<&str> {
    content.lines().collect()
}

fn reduce_part1(exp: &mut Vec<Node>) -> usize {
    let mut it = exp.iter_mut();
    let mut left = it.next().unwrap().value();
    while let Some(Node::Operator(op)) = it.next() {
        let right = it.next().unwrap().value();
        match op {
            Operator::Add => left += right,
            Operator::Multiply => left *= right,
        }
    }
    left
}

fn reduce_part2(exp: &mut Vec<Node>) -> usize {
    while let Some((i, _)) = exp
        .iter()
        .enumerate()
        .find(|&(_, n)| n == &Node::Operator(Operator::Add))
    {
        let mut left = exp[i - 1].value();
        left += exp[i + 1].value();
        exp.remove(i - 1);
        exp.remove(i - 1);
        exp.remove(i - 1);
        exp.insert(i - 1, Node::Value(left));
    }
    reduce_part1(exp)
}

fn evaluate(exp: &str, reduce: fn(&mut Vec<Node>) -> usize) -> usize {
    let exp = exp
        .chars()
        .map(Node::from)
        .filter(|c| c != &Node::None)
        .collect::<Vec<_>>();
    let mut stack = Vec::new();
    stack.reserve(exp.len());

    for c in exp {
        match c {
            Node::RightBracket => {
                let mut sub_exp = Vec::new();
                while stack.last().unwrap() != &Node::LeftBracket {
                    sub_exp.push(stack.pop().unwrap());
                }
                stack.pop();
                sub_exp.reverse();
                stack.push(Node::Value(reduce(&mut sub_exp)));
            }
            n => {
                stack.push(n);
            }
        }
    }
    reduce(&mut stack)
}

fn part1(expressions: &[&str]) -> usize {
    expressions
        .iter()
        .map(|exp| evaluate(exp, reduce_part1))
        .sum()
}

fn part2(expressions: &[&str]) -> usize {
    expressions
        .iter()
        .map(|exp| evaluate(exp, reduce_part2))
        .sum()
}

pub fn main() {
    let content = std::fs::read_to_string("data/2020/day18").unwrap();
    let expressions = parse(&content);

    // part 1
    println!("day 18 part1: {}", part1(&expressions));

    // part 2
    println!("day 18 part2: {}", part2(&expressions));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_part1() {
        vec![
            ("1 + (2 * 3) + (4 * (5 + 6))", 51),
            ("2 * 3 + (4 * 5)", 26),
            ("5 + (8 * 3 + 9 + 3 * 4 * 3)", 437),
            ("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 12240),
            ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 13632),
        ]
        .iter()
        .for_each(|&(exp, v)| {
            assert_eq!(evaluate(exp, reduce_part1), v);
        });
    }

    #[test]
    fn test_evaluate_part2() {
        vec![
            ("1 + (2 * 3) + (4 * (5 + 6))", 51),
            ("2 * 3 + (4 * 5)", 46),
            ("5 + (8 * 3 + 9 + 3 * 4 * 3)", 1445),
            ("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 669060),
            ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 23340),
        ]
        .iter()
        .for_each(|&(exp, v)| {
            assert_eq!(evaluate(exp, reduce_part2), v);
        });
    }
}
