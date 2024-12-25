fn parse(input: &str) -> (Vec<[u8; 5]>, Vec<[u8; 5]>) {
    let (mut locks, mut keys) = (vec![], vec![]);
    for part in input.trim().split("\n\n") {
        let rows = part.lines().collect::<Vec<_>>();
        if rows[0] == "#####" {
            let mut lock = [0; 5];
            for (j, e) in lock.iter_mut().enumerate() {
                let h = (1..=6).find(|&h| rows[h].as_bytes()[j] == b'.').unwrap();
                *e = (h - 1) as u8;
            }
            locks.push(lock);
        } else {
            let mut key = [0; 5];
            for (j, e) in key.iter_mut().enumerate() {
                let h = (1..=6).find(|&h| rows[h].as_bytes()[j] == b'#').unwrap();
                *e = (6 - h) as u8;
            }
            keys.push(key);
        }
    }
    (locks, keys)
}

fn part1(locks: Vec<[u8; 5]>, keys: Vec<[u8; 5]>) -> usize {
    locks
        .iter()
        .map(|l| {
            keys.iter()
                .filter(|&k| l.iter().zip(k).all(|(a, b)| a + b <= 5))
                .count()
        })
        .sum()
}

pub fn main() {
    let input: String = std::fs::read_to_string("data/2024/day25").unwrap();
    let (locks, keys) = parse(&input);
    println!("part1: {}", part1(locks, keys));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = "
#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";
        let (locks, keys) = parse(input);
        assert_eq!(3, part1(locks, keys));
    }
}
