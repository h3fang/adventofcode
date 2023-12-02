fn parse(data: &str) -> Vec<&str> {
    data.lines().filter(|line| !line.is_empty()).collect()
}

fn part1(document: &[&str]) -> i32 {
    document
        .iter()
        .map(|line| {
            let (mut a, mut b) = (-1, -1);
            line.as_bytes().iter().for_each(|c| {
                if c.is_ascii_digit() {
                    if a == -1 {
                        a = (c - b'0') as i32;
                    }
                    b = (c - b'0') as i32;
                }
            });
            a * 10 + b
        })
        .sum()
}

fn part2(document: &[&str]) -> i32 {
    document
        .iter()
        .map(|line| {
            let (mut a, mut b) = (-1, -1);
            for (i, c) in line.as_bytes().iter().enumerate() {
                if c.is_ascii_digit() {
                    if a == -1 {
                        a = (c - b'0') as i32;
                    }
                    b = (c - b'0') as i32;
                } else {
                    for (j, p) in [
                        "\n", "one", "two", "three", "four", "five", "six", "seven", "eight",
                        "nine",
                    ]
                    .into_iter()
                    .enumerate()
                    {
                        if line[i..].starts_with(p) {
                            if a == -1 {
                                a = j as i32;
                            }
                            b = j as i32;
                            break;
                        }
                    }
                }
            }
            a * 10 + b
        })
        .sum()
}

pub fn main() {
    let data = std::fs::read_to_string("data/2023/day1").unwrap();
    let document = parse(&data);
    println!("part1: {}", part1(&document));
    println!("part2: {}", part2(&document));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let document = parse(data);
        assert_eq!(142, part1(&document));
    }

    #[test]
    fn case2() {
        let data = "
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let document = parse(data);
        assert_eq!(281, part2(&document));
    }
}
