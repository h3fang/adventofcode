use nom::character::complete::i32 as n_i32;
use nom::{bytes::complete::tag, IResult, Parser};

fn mul(input: &str) -> IResult<&str, i32> {
    let (input, (_, a, _, b, _)) = (tag("mul("), n_i32, tag(","), n_i32, tag(")")).parse(input)?;
    Ok((input, a * b))
}

fn part1(mut input: &str) -> i32 {
    let mut result = 0;
    while !input.is_empty() {
        if let Ok((r, x)) = mul(input) {
            input = r;
            result += x;
        } else {
            input = &input[1..];
        }
    }
    result
}

fn part2(mut input: &str) -> i32 {
    let mut result = 0;
    while !input.is_empty() {
        if input.starts_with("don't()") {
            if let Some(i) = input.find("do()") {
                input = &input[i + "do()".len()..];
                continue;
            } else {
                break;
            }
        }
        if let Ok((r, x)) = mul(input) {
            input = r;
            result += x;
        } else {
            input = &input[1..];
        }
    }
    result
}

pub fn main() {
    let data: String = std::fs::read_to_string("data/2024/day3").unwrap();
    println!("part1: {}", part1(&data));
    println!("part2: {}", part2(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(161, part1(data));
    }

    #[test]
    fn case2() {
        let data = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(48, part2(data));
    }
}
