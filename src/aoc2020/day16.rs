use ahash::AHashMap as HashMap;
use ahash::AHashSet as HashSet;

type Ranges = HashMap<String, Vec<(usize, usize)>>;

fn parse(content: &str) -> (Ranges, Vec<usize>, Vec<Vec<usize>>) {
    let mut map = Ranges::new();
    let mut lines = content.lines();
    for line in &mut lines {
        if line.is_empty() {
            break;
        }

        let mut parts = line.split(": ");
        let name = parts.next().unwrap();
        let ranges = parts.next().unwrap();
        let ranges = ranges
            .split(" or ")
            .map(|r| {
                let endpoints = r
                    .split('-')
                    .map(|e| e.parse::<usize>().unwrap())
                    .collect::<Vec<_>>();
                (endpoints[0], endpoints[1])
            })
            .collect::<Vec<_>>();
        map.insert(name.to_string(), ranges);
    }

    assert_eq!(lines.next().unwrap(), "your ticket:");
    let my_ticket = lines
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    assert!(lines.next().unwrap().is_empty());
    assert_eq!(lines.next().unwrap(), "nearby tickets:");
    let nearby_tickets = lines
        .map(|line| {
            line.split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    (map, my_ticket, nearby_tickets)
}

fn part1(ranges: &Ranges, nearby_tickets: &[Vec<usize>]) -> (usize, Vec<Vec<usize>>) {
    let numbers = ranges
        .values()
        .flatten()
        .map(|r| [r.0, r.1])
        .flatten()
        .collect::<Vec<_>>();
    let max = numbers.iter().max().unwrap();
    let mut valid = vec![false; max + 1];
    ranges
        .values()
        .flatten()
        .for_each(|(s, e)| (*s..=*e).for_each(|n| valid[n] = true));

    let mut sum = 0;
    let mut valid_tickets = Vec::new();

    for ticket in nearby_tickets {
        let mut good = true;
        for &n in ticket {
            if n > *max || !valid[n] {
                sum += n;
                good = false;
                break;
            }
        }
        if good {
            valid_tickets.push(ticket.clone());
        }
    }

    (sum, valid_tickets)
}

fn part2(ranges: &Ranges, valid_tickets: &[Vec<usize>]) -> usize {
    let ranges = ranges
        .iter()
        .map(|(k, v)| {
            let max = v.iter().map(|e| [e.0, e.1]).flatten().max().unwrap();
            let mut valid = vec![false; max + 1];
            for r in v {
                (r.0..=r.1).for_each(|n| {
                    valid[n] = true;
                });
            }
            (k.as_str(), valid)
        })
        .collect::<HashMap<_, _>>();

    let mut map = HashMap::<&str, usize>::new();

    let n = ranges.len();

    let mut candidates = Vec::with_capacity(n);

    for i in 0..n {
        let fields = valid_tickets.iter().map(|t| t[i]).collect::<Vec<_>>();
        let mut c = HashSet::new();
        for (k, r) in &ranges {
            if fields.iter().all(|&f| f < r.len() && r[f]) {
                c.insert(*k);
            }
        }
        candidates.push((i, c));
    }
    candidates.sort_by_key(|c| c.1.len());

    loop {
        if candidates.is_empty() {
            break;
        }

        let c = candidates.first().unwrap();
        if c.1.len() > 1 {
            panic!("mutiple choices");
        }
        let key = *c.1.iter().next().unwrap();
        map.insert(key, c.0);
        candidates.remove(0);
        for (_, c) in &mut candidates {
            c.remove(key);
        }
    }

    let my_ticket = valid_tickets.last().unwrap();

    let mut r = 1;
    for (k, i) in map {
        if k.starts_with("departure") {
            r *= my_ticket[i];
        }
    }

    r
}

pub fn main() {
    let (ranges, my_ticket, nearby_tickets) =
        parse(&std::fs::read_to_string("data/2020/day16").unwrap());

    // part 1
    let (sum, mut valid_tickets) = part1(&ranges, &nearby_tickets);
    println!("day 16 part1: {}", sum);

    // part 2
    valid_tickets.push(my_ticket);
    println!("day 16 part2: {}", part2(&ranges, &valid_tickets));
}
