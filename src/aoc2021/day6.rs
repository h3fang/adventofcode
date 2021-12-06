fn part1(count: &mut [usize], days: usize) -> usize {
    fn step(count: &mut [usize]) {
        let new = count[0];
        for i in 0..8 {
            count[i] = count[i + 1];
        }
        count[8] = new;
        count[6] += new;
    }

    for _ in 0..days {
        step(count);
    }
    count.iter().sum::<usize>()
}

pub fn main() {
    let data = std::fs::read_to_string("data/2021/day6").unwrap();
    let mut count = [0usize; 9];
    data.split(',').for_each(|s| {
        let f = s.trim().parse::<usize>().unwrap();
        count[f] += 1;
    });

    println!("day6 part1: {}", part1(&mut count.clone(), 80));
    println!("day6 part2: {}", part1(&mut count, 256));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "3,4,3,1,2";
        let mut count = [0usize; 9];
        data.split(',').for_each(|s| {
            let f = s.trim().parse::<usize>().unwrap();
            count[f] += 1;
        });

        assert_eq!(26, part1(&mut count.clone(), 18));
        assert_eq!(5934, part1(&mut count, 80));
    }
}
