fn part1(mut codes: Vec<usize>) -> usize {
    let mut i = 0;
    while i < codes.len() {
        match codes[i] {
            1 => {
                let a = codes[i + 1];
                let b = codes[i + 2];
                let c = codes[i + 3];
                codes[c] = codes[a] + codes[b];
                i += 4;
            }
            2 => {
                let a = codes[i + 1];
                let b = codes[i + 2];
                let c = codes[i + 3];
                codes[c] = codes[a] * codes[b];
                i += 4;
            }
            _ => break,
        }
    }
    codes[0]
}

fn part2(codes: &[usize]) -> usize {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut codes = codes.to_vec();
            codes[1] = noun;
            codes[2] = verb;
            if part1(codes) == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    0
}

pub fn main() {
    let codes = std::fs::read_to_string("data/2019/day2")
        .unwrap()
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|t| t.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut p1 = codes.clone();
    p1[1] = 12;
    p1[2] = 2;
    println!("day2 part1: {}", part1(p1));

    println!("day2 part2: {}", part2(&codes));
}
