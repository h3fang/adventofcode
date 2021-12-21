use ahash::AHashMap as HashMap;

fn parse(data: &str) -> (usize, usize) {
    let pos = data
        .lines()
        .map(|line| line.split(": ").last().unwrap().parse::<usize>().unwrap())
        .take(2)
        .collect::<Vec<_>>();
    (pos[0], pos[1])
}

fn part1(mut p1: usize, mut p2: usize) -> usize {
    let mut dice = (1usize..=100).cycle();
    let mut total = 0;
    let mut s1 = 0;
    let mut s2 = 0;

    let mut roll3 = || {
        total += 3;
        dice.next().unwrap() + dice.next().unwrap() + dice.next().unwrap()
    };

    let mut play = |mut p: usize| {
        p = (p + roll3()) % 10;
        if p == 0 {
            10
        } else {
            p
        }
    };

    loop {
        p1 = play(p1);
        s1 += p1;

        if s1 >= 1000 {
            return s2 * total;
        }

        p2 = play(p2);
        s2 += p2;

        if s2 >= 1000 {
            return s1 * total;
        }
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct State {
    p1: u8,
    p2: u8,
    s1: u8,
    s2: u8,
    p1_to_play: bool,
}

fn part2(p1: usize, p2: usize) -> usize {
    fn play(s: State, cache: &mut HashMap<State, (usize, usize)>) -> (usize, usize) {
        if s.s1 >= 21 {
            return (1, 0);
        }
        if s.s2 >= 21 {
            return (0, 1);
        }

        if let Some(result) = cache.get(&s) {
            return *result;
        }

        let mut result = (0, 0);
        for (roll, n) in [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)] {
            let mut p = if s.p1_to_play { s.p1 } else { s.p2 };
            p = (p + roll) % 10;
            p = if p == 0 { 10 } else { p };
            let r = if s.p1_to_play {
                let mut next = s;
                next.p1 = p;
                next.s1 += p;
                next.p1_to_play = !next.p1_to_play;
                play(next, cache)
            } else {
                let mut next = s;
                next.p2 = p;
                next.s2 += p;
                next.p1_to_play = !next.p1_to_play;
                play(next, cache)
            };
            result.0 += n * r.0;
            result.1 += n * r.1;
        }

        cache.insert(s, result);
        result
    }

    let mut cache = HashMap::new();
    let s = State {
        p1: p1 as u8,
        p2: p2 as u8,
        s1: 0,
        s2: 0,
        p1_to_play: true,
    };
    let r = play(s, &mut cache);
    r.0.max(r.1)
}

pub fn main() {
    let data = std::fs::read_to_string("data/2021/day21").unwrap();
    let (p1, p2) = parse(&data);
    println!("day21 part1: {}", part1(p1, p2));
    println!("day21 part2: {}", part2(p1, p2));
}
