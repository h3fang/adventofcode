use ahash::HashMap;
use rayon::prelude::*;

fn parse(data: &str) -> Vec<(&str, Vec<usize>)> {
    data.trim()
        .lines()
        .map(|line| {
            let (springs, sizes) = line.split_once(' ').unwrap();
            let sizes: Vec<usize> = sizes.split(',').map(|s| s.parse().unwrap()).collect();
            (springs, sizes)
        })
        .collect()
}

fn split(springs: &[u8], sizes: &[usize]) -> usize {
    fn dfs<'a>(
        springs: &'a [u8],
        sizes: &'a [usize],
        curr: usize,
        cache: &mut HashMap<(&'a [u8], &'a [usize], usize), usize>,
    ) -> usize {
        if let Some(&r) = cache.get(&(springs, sizes, curr)) {
            return r;
        }
        let r = match springs.first() {
            Some(&c) => {
                let arrangements = match c {
                    b'.' | b'?' => {
                        if curr > 0 {
                            if let Some(&s) = sizes.first() {
                                if curr == s {
                                    dfs(&springs[1..], &sizes[1..], 0, cache)
                                } else {
                                    0
                                }
                            } else {
                                0
                            }
                        } else {
                            dfs(&springs[1..], sizes, 0, cache)
                        }
                    }
                    b'#' => dfs(&springs[1..], sizes, curr + 1, cache),
                    _ => unreachable!(),
                };
                if c == b'?' {
                    arrangements + dfs(&springs[1..], sizes, curr + 1, cache)
                } else {
                    arrangements
                }
            }
            None => {
                if curr == 0 {
                    usize::from(sizes.is_empty())
                } else {
                    usize::from(sizes == [curr])
                }
            }
        };
        cache.insert((springs, sizes, curr), r);
        r
    }
    dfs(springs, sizes, 0, &mut HashMap::default())
}

fn part1(records: &[(&str, Vec<usize>)]) -> usize {
    records
        .iter()
        .map(|(springs, sizes)| split(springs.as_bytes(), sizes))
        .sum()
}

fn part2(records: &[(&str, Vec<usize>)]) -> usize {
    records
        .par_iter()
        .map(|(springs, sizes)| {
            let springs: String = [springs, "?"].into_iter().cycle().take(9).collect();
            let sizes = sizes.repeat(5);
            split(springs.as_bytes(), &sizes)
        })
        .sum()
}

pub fn main() {
    let data = std::fs::read_to_string("data/2023/day12").unwrap();
    let records = parse(&data);
    println!("part1: {}", part1(&records));
    println!("part2: {}", part2(&records));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";
        let records = parse(data);
        assert_eq!(21, part1(&records));
        assert_eq!(525152, part2(&records));
    }
}
