fn digits(mut p: i32) -> Vec<i32> {
    let mut s = Vec::with_capacity(8);
    while p > 0 {
        s.push(p % 10);
        p /= 10;
    }
    s
}

fn part1(nums: &[Vec<i32>]) -> usize {
    nums.iter()
        .filter(|s| s.windows(2).all(|w| w[0] >= w[1]) && s.windows(2).any(|w| w[0] == w[1]))
        .count()
}

fn part2(nums: &[Vec<i32>]) -> usize {
    nums.iter()
        .filter(|s| {
            s.windows(2).all(|w| w[0] >= w[1])
                && s.windows(2).enumerate().any(|(i, w)| {
                    w[0] == w[1]
                        && (i == 0 || s[i - 1] != w[0])
                        && (i == s.len() - 2 || s[i + 2] != w[0])
                })
        })
        .count()
}

pub fn main() {
    let lb = 156218;
    let ub = 652527;

    let nums = (lb..=ub).map(digits).collect::<Vec<_>>();

    println!("day4 part1: {}", part1(&nums));

    println!("day4 part2: {}", part2(&nums));
}
