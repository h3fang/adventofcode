use std::collections::VecDeque;

fn parse(data: &str) -> Vec<i64> {
    data.trim()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

fn mixing(nums: &mut [i64], key: i64, repeat: usize) -> i64 {
    if key != 1 {
        nums.iter_mut().for_each(|e| *e *= key);
    }
    let n = nums.len();
    let mut q = (0..n).collect::<VecDeque<_>>();
    for (i, &e) in nums.iter().enumerate().cycle().take(n * repeat) {
        let j = q.iter().position(|&k| k == i).unwrap();
        if e != 0 {
            q.rotate_left(j);
            q.pop_front();
            if e > 0 {
                q.rotate_left(e as usize % (n - 1));
            } else {
                q.rotate_right((-e) as usize % (n - 1));
            }
            q.push_front(i);
        }
    }
    let i = nums.iter().position(|&e| e == 0).unwrap();
    let i = q.iter().position(|&e| e == i).unwrap();
    [1000, 2000, 3000]
        .into_iter()
        .map(|d| {
            let j = q[(i + d) % n];
            nums[j]
        })
        .sum()
}

pub fn main() {
    let data = std::fs::read_to_string("data/2022/day20").unwrap();
    let mut nums = parse(&data);
    println!("part1: {}", mixing(&mut nums, 1, 1));
    println!("part2: {}", mixing(&mut nums, 811589153, 10));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "
1
2
-3
3
-2
0
4";
        let mut nums = parse(&data);
        assert_eq!(3, mixing(&mut nums, 1, 1));
        assert_eq!(1623178306, mixing(&mut nums, 811589153, 10));
    }
}
