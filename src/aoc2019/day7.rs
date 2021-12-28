use crate::day5::Intcode;

fn part1(codes: &[i64]) -> i64 {
    fn amp(codes: &[i64], signal: i64, phases: &mut [i64], result: &mut i64) {
        if phases.is_empty() {
            *result = (*result).max(signal);
        } else {
            for i in 0..phases.len() {
                phases.swap(i, 0);
                let mut code = Intcode::new(codes);
                code.inputs.extend([phases[0], signal]);
                code.run();
                let output = code.outputs.pop_front().unwrap();
                amp(codes, output, &mut phases[1..], result);
                phases.swap(i, 0);
            }
        }
    }
    let mut phases = (0..5).collect::<Vec<_>>();
    let mut result = 0;
    amp(codes, 0, &mut phases, &mut result);

    result
}

fn run(codes: &[i64], phases: &[i64]) -> i64 {
    let mut codes = (0..5).map(|_| Intcode::new(codes)).collect::<Vec<_>>();
    let mut outputs = [0; 5];
    for i in 0..5 {
        if i == 0 {
            codes[i].inputs.extend([phases[0], 0]);
        } else {
            codes[i].inputs.extend([phases[i], outputs[i - 1]]);
        }
        codes[i].run();
        outputs[i] = codes[i].outputs.pop_front().unwrap();
    }

    while !codes[4].is_halted() {
        for i in 0..5 {
            if i == 0 {
                codes[i].inputs.push_back(outputs[4]);
            } else {
                codes[i].inputs.push_back(outputs[i - 1]);
            }
            codes[i].run();
            if let Some(o) = codes[i].outputs.pop_front() {
                outputs[i] = o;
            }
        }
    }
    outputs[4]
}

fn permutation(phases: &mut [i64], i: usize, result: &mut i64, codes: &[i64]) {
    if i == phases.len() {
        *result = (*result).max(run(codes, phases));
    } else {
        for j in i..phases.len() {
            phases.swap(j, i);
            permutation(phases, i + 1, result, codes);
            phases.swap(j, i);
        }
    }
}

fn part2(codes: &[i64]) -> i64 {
    let mut phases = (5..10).collect::<Vec<_>>();
    let mut result = 0;
    permutation(&mut phases, 0, &mut result, codes);
    result
}

pub fn main() {
    let codes = std::fs::read_to_string("data/2019/day7")
        .unwrap()
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|t| t.trim().parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    println!("day7 part1: {}", part1(&codes));
    println!("day7 part2: {}", part2(&codes));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let codes = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"
            .split(',')
            .map(|t| t.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        assert_eq!(43210, part1(&codes));
    }

    #[test]
    fn case2() {
        let codes = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0"
            .split(',')
            .map(|t| t.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        assert_eq!(54321, part1(&codes));
    }

    #[test]
    fn case3() {
        let codes =
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"
                .split(',')
                .map(|t| t.parse::<i64>().unwrap())
                .collect::<Vec<_>>();

        assert_eq!(139629729, part2(&codes));
    }

    #[test]
    fn case4() {
        let codes = "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,
            -5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,
            53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"
            .split(',')
            .map(|t| t.trim().parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        assert_eq!(18216, part2(&codes));
    }
}
