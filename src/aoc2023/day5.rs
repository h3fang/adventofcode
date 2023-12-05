use std::collections::VecDeque;

fn parse(data: &str) -> (Vec<u64>, Vec<Vec<Vec<u64>>>) {
    let lines = data.trim().lines().collect::<Vec<_>>();
    let seeds = lines[0]
        .strip_prefix("seeds: ")
        .unwrap()
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u64>>();
    let maps = lines[2..]
        .split(|s| s.is_empty())
        .map(|g| {
            let mut g = g
                .iter()
                .skip(1)
                .map(|s| s.split(' ').map(|n| n.parse().unwrap()).collect::<Vec<_>>())
                .collect::<Vec<_>>();
            g.sort_unstable_by_key(|e| e[1]);
            g
        })
        .collect();
    (seeds, maps)
}

fn part1(seeds: &[u64], maps: &[Vec<Vec<u64>>]) -> u64 {
    seeds
        .iter()
        .map(|s| {
            let mut seed = *s;
            for g in maps {
                for m in g {
                    if (m[1]..m[1] + m[2]).contains(&seed) {
                        seed = m[0] + (seed - m[1]);
                        break;
                    }
                }
            }
            seed
        })
        .min()
        .unwrap()
}

fn part2(seeds: &[u64], maps: &[Vec<Vec<u64>>]) -> u64 {
    let mut ranges = seeds
        .chunks(2)
        .map(|c| [c[0], c[0] + c[1] - 1])
        .collect::<VecDeque<_>>();
    for map in maps {
        let mut next = VecDeque::with_capacity(ranges.len());
        while let Some([a, b]) = ranges.pop_front() {
            let i = map.partition_point(|e| e[1] + e[2] - 1 < a);
            if i == map.len() {
                next.push_back([a, b]);
                continue;
            }
            for e in &map[i..] {
                let (c, d, e) = (e[1], e[1] + e[2] - 1, e[0]);
                if c > b {
                    next.push_back([a, b]);
                    break;
                } else if c <= a && d >= b {
                    next.push_back([e + a - c, e + b - c]);
                    break;
                } else if a <= c && b >= d {
                    if c > a {
                        next.push_back([a, c - 1]);
                    }
                    next.push_back([e, e + d - c]);
                    if b > d {
                        ranges.push_back([d + 1, b]);
                    }
                    break;
                } else if c >= a && d >= b {
                    if c > a {
                        next.push_back([a, c - 1]);
                    }
                    next.push_back([e, e + b - c]);
                    break;
                } else if c <= a && b >= d {
                    next.push_back([e + a - c, e + d - c]);
                    if b > d {
                        ranges.push_back([d + 1, b]);
                    }
                    break;
                }
            }
        }
        ranges = next;
    }
    ranges.into_iter().map(|e| e[0]).min().unwrap()
}

pub fn main() {
    let data = std::fs::read_to_string("data/2023/day5").unwrap();
    let (seeds, maps) = parse(&data);
    println!("part1: {}", part1(&seeds, &maps));
    println!("part2: {}", part2(&seeds, &maps));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let (seeds, maps) = parse(&data);
        assert_eq!(35, part1(&seeds, &maps));
        assert_eq!(46, part2(&seeds, &maps));
    }
}
