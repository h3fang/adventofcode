fn parse(data: &str) -> Vec<&str> {
    data.lines().filter(|line| !line.is_empty()).collect()
}

fn part1(lines: &[&str]) -> usize {
    let m = lines.len();
    let nums: Vec<usize> = lines[..m - 1]
        .iter()
        .flat_map(|line| line.split_ascii_whitespace().map(|t| t.parse().unwrap()))
        .collect();
    let ops: Vec<u8> = lines[m - 1]
        .split_ascii_whitespace()
        .map(|t| t.as_bytes()[0])
        .collect();
    let n = ops.len();
    let mut ans = 0;
    for (i, &op) in ops.iter().enumerate() {
        let nums = nums.iter().skip(i).step_by(n).cloned();
        match op {
            b'+' => ans += nums.sum::<usize>(),
            b'*' => ans += nums.product::<usize>(),
            _ => unreachable!(),
        }
    }
    ans
}

fn part2(lines: &[&str]) -> usize {
    let m = lines.len();
    let nums: Vec<&[u8]> = lines[..m - 1].iter().map(|line| line.as_bytes()).collect();
    let mut ans = 0;
    for (i, &op) in lines[m - 1].as_bytes().iter().enumerate() {
        if op == b' ' {
            continue;
        }
        let mut res = if op == b'+' { 0 } else { 1 };
        let mut j = i + 1;
        while j < nums[0].len() && nums.iter().any(|num| num[j] != b' ') {
            j += 1;
        }
        for k in i..j {
            let mut x = 0;
            for num in &nums {
                match num[k] {
                    b' ' => continue,
                    b => x = x * 10 + (b - b'0') as usize,
                }
            }

            match op {
                b'+' => res += x,
                b'*' => res *= x,
                _ => unreachable!(),
            }
        }
        ans += res;
    }
    ans
}

pub fn main() {
    let data = std::fs::read_to_string("data/2025/day6").unwrap();
    let lines = parse(&data);
    println!("part1: {}", part1(&lines));
    println!("part2: {}", part2(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
        let lines = parse(data);
        assert_eq!(4277556, part1(&lines));
        assert_eq!(3263827, part2(&lines));
    }
}
