fn parse(data: &str) -> Vec<i32> {
    data.split("\n\n")
        .map(|e| {
            e.trim()
                .lines()
                .map(|line| line.parse::<i32>().unwrap())
                .sum()
        })
        .collect()
}

fn part1(calories: &[i32]) -> i32 {
    *calories.last().unwrap()
}

fn part2(calories: &[i32]) -> i32 {
    calories.iter().rev().take(3).sum()
}

pub fn main() {
    let data = std::fs::read_to_string("data/2022/day1").unwrap();
    let mut calories = parse(&data);
    calories.sort_unstable();
    println!("part1: {}", part1(&calories));
    println!("part2: {}", part2(&calories));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        let mut calories = parse(data);
        calories.sort_unstable();
        assert_eq!(24000, part1(&calories));
        assert_eq!(45000, part2(&calories));
    }
}
