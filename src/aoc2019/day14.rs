use ahash::AHashMap as HashMap;

type Graph<'a> = HashMap<&'a str, (usize, Vec<(&'a str, usize)>)>;

fn part1(g: &Graph, r: usize) -> usize {
    fn dfs<'a>(
        g: &Graph<'a>,
        curr: &'a str,
        mut amount: usize,
        remaining: &mut HashMap<&'a str, usize>,
    ) -> usize {
        if curr == "ORE" {
            return amount;
        }

        if let Some(&n) = remaining.get(curr) {
            if n > amount {
                remaining.insert(curr, n - amount);
                return 0;
            } else {
                remaining.remove(&curr);
                amount -= n;
                if amount == 0 {
                    return 0;
                }
            }
        }

        if let Some((n, inputs)) = g.get(&curr) {
            let mut result = 0;
            let count = if amount % n == 0 {
                amount / n
            } else {
                *remaining.entry(curr).or_default() += n - amount % n;
                amount / n + 1
            };
            for (input, k) in inputs {
                result += dfs(g, *input, count * k, remaining);
            }
            result
        } else {
            panic!("missing reaction for {}", curr);
        }
    }
    let mut remaining = HashMap::new();
    dfs(g, "FUEL", r, &mut remaining)
}

fn create_graph(data: &str) -> Graph {
    let mut g: Graph = HashMap::new();
    data.lines().for_each(|s| {
        let parts = s.trim().split("=>").collect::<Vec<_>>();
        let output = parts[1].trim().split(' ').collect::<Vec<_>>();
        let n = output[0].parse::<usize>().unwrap();
        let output = output[1];
        let e = g.entry(output).or_insert((n, Vec::new()));
        parts[0].split(',').for_each(|p| {
            let input = p.trim().split(' ').collect::<Vec<_>>();
            let n = input[0].parse::<usize>().unwrap();
            let input = input[1];
            e.1.push((input, n));
        });
    });
    g
}

fn part2(g: &Graph) -> usize {
    const TOTAL: usize = 1000000000000;
    let mut left = 0;
    let mut right = TOTAL;
    while left < right {
        let mid = (left + right + 1) / 2;
        let req = part1(g, mid);
        if req > TOTAL {
            right = mid - 1;
        } else {
            left = mid;
        }
    }
    left
}

pub fn main() {
    let data = std::fs::read_to_string("data/2019/day14").unwrap();
    let g = create_graph(&data);

    println!("day14 part1: {}", part1(&g, 1));
    println!("day14 part2: {}", part2(&g));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "9 ORE => 2 A
        8 ORE => 3 B
        7 ORE => 5 C
        3 A, 4 B => 1 AB
        5 B, 7 C => 1 BC
        4 C, 1 A => 1 CA
        2 AB, 3 BC, 4 CA => 1 FUEL";
        let g = create_graph(&data);
        assert_eq!(165, part1(&g, 1));
    }

    #[test]
    fn case2() {
        let data = "10 ORE => 10 A
        1 ORE => 1 B
        7 A, 1 B => 1 C
        7 A, 1 C => 1 D
        7 A, 1 D => 1 E
        7 A, 1 E => 1 FUEL";
        let g = create_graph(&data);
        assert_eq!(31, part1(&g, 1));
    }

    #[test]
    fn case3() {
        let data = "171 ORE => 8 CNZTR
        7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
        114 ORE => 4 BHXH
        14 VRPVC => 6 BMBT
        6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
        6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
        15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
        13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
        5 BMBT => 4 WPTQ
        189 ORE => 9 KTJDG
        1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
        12 VRPVC, 27 CNZTR => 2 XDBXC
        15 KTJDG, 12 BHXH => 5 XCVML
        3 BHXH, 2 VRPVC => 7 MZWV
        121 ORE => 7 VRPVC
        7 XCVML => 6 RJRHP
        5 BHXH, 4 VRPVC => 5 LTCX";
        let g = create_graph(&data);
        assert_eq!(2210736, part1(&g, 1));
    }
}
