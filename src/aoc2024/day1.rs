use ahash::HashMap;

fn parse(data: &str) -> (Vec<i32>, Vec<i32>) {
    let (mut l1, mut l2) = (vec![], vec![]);
    data.lines()
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            let (a, b) = line.split_once("   ").unwrap();
            l1.push(a.parse().unwrap());
            l2.push(b.parse().unwrap());
        });
    (l1, l2)
}

fn part1(l1: &mut [i32], l2: &mut [i32]) -> i32 {
    l1.sort_unstable();
    l2.sort_unstable();
    l1.iter().zip(&*l2).map(|(x, y)| (x - y).abs()).sum()
}

fn part2(l1: &[i32], l2: &[i32]) -> i32 {
    let mut f = HashMap::default();
    for &x in l2 {
        *f.entry(x).or_insert(0) += 1;
    }
    l1.iter().map(|x| x * f.get(x).cloned().unwrap_or(0)).sum()
}

pub fn main() {
    let data = std::fs::read_to_string("data/2024/day1").unwrap();
    let (mut l1, mut l2) = parse(&data);
    println!("part1: {}", part1(&mut l1, &mut l2));
    println!("part2: {}", part2(&l1, &l2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "
3   4
4   3
2   5
1   3
3   9
3   3";
        let (mut l1, mut l2) = parse(&data);
        assert_eq!(11, part1(&mut l1, &mut l2));
        assert_eq!(31, part2(&l1, &l2));
    }
}
