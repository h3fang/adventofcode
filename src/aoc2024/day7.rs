use rayon::prelude::*;

fn parse(input: &str) -> Vec<Vec<i64>> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(": ").unwrap();
            let mut result = vec![a.parse().unwrap()];
            for x in b.split(' ') {
                result.push(x.parse().unwrap());
            }
            result
        })
        .collect()
}

fn evaluate(test: &[i64], i: usize, curr: i64) -> bool {
    if i == test.len() {
        return test[0] == curr;
    }
    let x = test[i];

    if curr > test[0] || x > test[0] {
        return false;
    }
    evaluate(test, i + 1, curr + x) || evaluate(test, i + 1, curr * x)
}

fn part1(tests: &[Vec<i64>]) -> i64 {
    tests
        .par_iter()
        .filter(|t| evaluate(t, 2, t[1]))
        .map(|t| t[0])
        .sum()
}

fn evaluate2(test: &[i64], i: usize, curr: i64) -> bool {
    if i == test.len() {
        return test[0] == curr;
    }
    let x = test[i];

    if curr > test[0] || x > test[0] {
        return false;
    }
    if evaluate2(test, i + 1, curr + x) || evaluate2(test, i + 1, curr * x) {
        return true;
    }
    let y = curr * 10i64.pow(x.ilog10() + 1) + x;
    evaluate2(test, i + 1, y)
}

fn part2(tests: &[Vec<i64>]) -> i64 {
    tests
        .par_iter()
        .filter(|t| evaluate2(t, 2, t[1]))
        .map(|t| t[0])
        .sum()
}

pub fn main() {
    let input: String = std::fs::read_to_string("data/2024/day7").unwrap();
    let tests = parse(&input);
    println!("part1: {}", part1(&tests));
    println!("part2: {}", part2(&tests));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = "
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        let tests = parse(input);
        assert_eq!(3749, part1(&tests));
        assert_eq!(11387, part2(&tests));
    }
}
