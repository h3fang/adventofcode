fn encode_digit(d: &str) -> u8 {
    let mut result = 0;
    for d in d.as_bytes() {
        result |= 1 << (d - b'a');
    }
    result
}

fn part1(notes: &[(Vec<u8>, Vec<u8>)]) -> usize {
    notes
        .iter()
        .map(|n| {
            n.1.iter()
                .filter(|d| {
                    let c = d.count_ones();
                    c == 2 || c == 4 || c == 3 || c == 7
                })
                .count()
        })
        .sum()
}

fn get_output(patterns: &[u8], outputs: &[u8]) -> usize {
    let mut one = (0, patterns[0]);
    let mut seven = (0, patterns[0]);
    let mut four = (0, patterns[0]);
    let mut eight = (0, patterns[0]);
    let mut fs = vec![];
    let mut ss = vec![];
    for (i, &p) in patterns.iter().enumerate() {
        match p.count_ones() {
            2 => one = (i, p),
            3 => seven = (i, p),
            4 => four = (i, p),
            5 => fs.push((i, p)),
            6 => ss.push((i, p)),
            7 => eight = (i, p),
            _ => {}
        }
    }

    let mut map = [0; 10];
    map[one.0] = 1;
    map[seven.0] = 7;
    map[four.0] = 4;
    map[eight.0] = 8;

    type Parts = (Vec<(usize, u8)>, Vec<(usize, u8)>);

    let (nine, six_zero): Parts = ss.into_iter().partition(|s| four.1 & s.1 == four.1);
    map[nine[0].0] = 9;
    let e = eight.1 ^ nine[0].1;

    let bd = four.1 ^ one.1;
    for s in &fs {
        if seven.1 & s.1 == seven.1 {
            map[s.0] = 3;
        } else if s.1 & e > 0 {
            map[s.0] = 2;
        } else if bd & s.1 == bd {
            map[s.0] = 5;
        }
    }

    for s in &six_zero {
        if s.1 & one.1 == one.1 {
            map[s.0] = 0;
        } else {
            map[s.0] = 6;
        }
    }

    let mut result = 0;
    for output in outputs {
        let i = patterns.iter().position(|p| p == output).unwrap();
        result = result * 10 + map[i];
    }
    result
}

fn part2(notes: &[(Vec<u8>, Vec<u8>)]) -> usize {
    notes.iter().map(|n| get_output(&n.0, &n.1)).sum()
}

pub fn main() {
    let data = std::fs::read_to_string("data/2021/day8").unwrap();
    let notes = data
        .lines()
        .map(|line| {
            let mut parts = line.split('|');
            let patterns = parts
                .next()
                .unwrap()
                .trim()
                .split(' ')
                .map(encode_digit)
                .collect::<Vec<_>>();
            let output = parts
                .next()
                .unwrap()
                .trim()
                .split(' ')
                .map(encode_digit)
                .collect::<Vec<_>>();
            (patterns, output)
        })
        .collect::<Vec<_>>();

    println!("day8 part1: {}", part1(&notes));
    println!("day8 part2: {}", part2(&notes));
}
