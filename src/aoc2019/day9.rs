use crate::day5::Intcode;

fn part1(codes: &[i64]) -> i64 {
    let mut prog = Intcode::new(codes);
    prog.inputs = vec![1];
    prog.run();
    prog.output
}

fn part2(codes: &[i64]) -> i64 {
    let mut prog = Intcode::new(codes);
    prog.inputs = vec![2];
    prog.run();
    prog.output
}

pub fn main() {
    let codes = std::fs::read_to_string("data/2019/day9")
        .unwrap()
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|t| t.trim().parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    println!("day9 part1: {}", part1(&codes));
    println!("day9 part2: {}", part2(&codes));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let codes = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99"
            .split(',')
            .map(|t| t.trim().parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        let mut prog = Intcode::new(&codes);
        let mut outputs = vec![];
        loop {
            prog.run();
            if !prog.is_halted() {
                outputs.push(prog.output);
            } else {
                break;
            }
        }
        assert_eq!(codes, outputs);
    }

    #[test]
    fn case2() {
        let codes = "1102,34915192,34915192,7,4,7,99,0"
            .split(',')
            .map(|t| t.trim().parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        let mut prog = Intcode::new(&codes);
        prog.run();
        let n = prog.output.to_string().len();
        assert_eq!(16, n);
    }

    #[test]
    fn case3() {
        let codes = "104,1125899906842624,99"
            .split(',')
            .map(|t| t.trim().parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        let mut prog = Intcode::new(&codes);
        prog.run();
        assert_eq!(1125899906842624, prog.output);
    }
}
