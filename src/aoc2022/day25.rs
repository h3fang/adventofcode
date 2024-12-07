fn parse(data: &str) -> Vec<&str> {
    data.trim().lines().collect::<Vec<_>>()
}

fn snafu_digit_to_dec(d: u8) -> i64 {
    match d {
        b'2' => 2,
        b'1' => 1,
        b'0' => 0,
        b'-' => -1,
        b'=' => -2,
        _ => unreachable!(),
    }
}

fn dec_digit_to_snafu(d: i64) -> (u8, bool) {
    match d {
        2 => (b'2', false),
        1 => (b'1', false),
        0 => (b'0', false),
        4 => (b'-', true),
        3 => (b'=', true),
        _ => unreachable!(),
    }
}

fn snafu_to_dec(s: &str) -> i64 {
    let mut result = 0;
    let mut base = 1;
    for &d in s.as_bytes().iter().rev() {
        result += snafu_digit_to_dec(d) * base;
        base *= 5;
    }
    result
}

fn dec_to_snafu(mut n: i64) -> String {
    let mut result = vec![];
    let mut carry = false;
    while n > 0 {
        let d = i64::from(carry) + n % 5;
        let (sd, ca) = dec_digit_to_snafu(d);
        carry = ca;
        result.push(sd);
        n /= 5;
    }
    result.reverse();
    String::from_utf8(result).unwrap()
}

fn part1(nums: &[&str]) -> String {
    let sum = nums.iter().map(|n| snafu_to_dec(n)).sum();
    dec_to_snafu(sum)
}

pub fn main() {
    let data = std::fs::read_to_string("data/2022/day25").unwrap();
    let nums = parse(&data);
    println!("part1: {}", part1(&nums));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let s = "2=-01";
        let n = 976;
        assert_eq!(n, snafu_to_dec(s));
        assert_eq!(s, dec_to_snafu(n));
    }

    #[test]
    fn case2() {
        let data = "
1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";
        let nums = parse(data);
        assert_eq!("2=-1=0", part1(&nums));
    }
}
