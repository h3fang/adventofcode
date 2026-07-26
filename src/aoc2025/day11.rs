use ahash::{HashMap, HashMapExt};

fn parse(data: &str) -> HashMap<&str, Vec<&str>> {
    data.trim()
        .lines()
        .map(|line| {
            let (dev, outputs) = line.split_once(": ").unwrap();

            let dev = dev.trim();
            let outputs = outputs.trim().split(' ').map(|o| o.trim()).collect();

            (dev, outputs)
        })
        .collect()
}

fn dfs<'a>(
    server: &HashMap<&'a str, Vec<&'a str>>,
    cache: &mut HashMap<&'a str, u64>,
    x: &'a str,
    target: &'a str,
) -> u64 {
    if x == target {
        return 1;
    }

    if let Some(&ways) = cache.get(x) {
        return ways;
    }

    let mut ans = 0;

    if let Some(children) = server.get(x) {
        for &y in children {
            ans += dfs(server, cache, y, target);
        }
    }

    cache.insert(x, ans);

    ans
}

fn part1(server: &HashMap<&str, Vec<&str>>, x: &str, target: &str) -> u64 {
    dfs(server, &mut HashMap::with_capacity(server.len()), x, target)
}

fn part2(server: &HashMap<&str, Vec<&str>>) -> u64 {
    let a = part1(server, "svr", "fft") * part1(server, "fft", "dac") * part1(server, "dac", "out");
    let b = part1(server, "svr", "dac") * part1(server, "dac", "fft") * part1(server, "fft", "out");
    a + b
}

pub fn main() {
    let data = std::fs::read_to_string("data/2025/day11").unwrap();
    let server = parse(&data);
    println!("part1: {}", part1(&server, "you", "out"));
    println!("part2: {}", part2(&server));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";
        let server = parse(data);
        assert_eq!(5, part1(&server, "you", "out"));
    }

    #[test]
    fn case2() {
        let data = "
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";
        let server = parse(data);
        assert_eq!(2, part2(&server));
    }
}
