use ahash::{HashSet, HashSetExt};

#[derive(Clone, Copy)]
struct Computer {
    a: u64,
    b: u64,
    c: u64,
    ip: i32,
}

impl Computer {
    fn run(&mut self, program: &[u8]) -> Vec<u8> {
        let mut result = Vec::with_capacity(program.len() + 1);
        while self.ip + 1 < program.len() as i32 {
            let (opcode, operand) = (program[self.ip as usize], program[self.ip as usize + 1]);
            match opcode {
                0 => self.a >>= self.combo_operand(operand),
                1 => self.b ^= operand as u64,
                2 => self.b = self.combo_operand(operand) & 7,
                3 => {
                    if self.a != 0 {
                        self.ip = operand as i32 - 2;
                    }
                }
                4 => self.b ^= self.c,
                5 => result.push((self.combo_operand(operand) & 7) as u8),
                6 => self.b = self.a >> self.combo_operand(operand),
                7 => self.c = self.a >> self.combo_operand(operand),
                _ => unreachable!(),
            }
            self.ip += 2;
        }
        result
    }

    fn combo_operand(&self, op: u8) -> u64 {
        match op {
            0..=3 => op as u64,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => unreachable!(),
        }
    }
}

fn parse(input: &str) -> (Computer, Vec<u8>) {
    let (regs, prog) = input.trim().split_once("\n\n").unwrap();
    let regs: Vec<u64> = regs
        .lines()
        .map(|r| r.split_once(": ").unwrap().1.parse().unwrap())
        .collect();
    let program = prog
        .trim_start_matches("Program: ")
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    let computer = Computer {
        a: regs[0],
        b: regs[1],
        c: regs[2],
        ip: 0,
    };
    (computer, program)
}

fn part1(mut computer: Computer, program: &[u8]) -> String {
    let output = computer.run(program);
    let mut r = String::with_capacity(output.len() * 2 - 1);
    for x in output {
        if !r.is_empty() {
            r.push(',');
        }
        r.push((x + b'0') as char);
    }
    r
}

fn part2(mut computer: Computer, program: &[u8]) -> u64 {
    let mut candidates = HashSet::default();
    candidates.insert(0);
    for &p in program.iter().rev() {
        let mut next = HashSet::with_capacity(candidates.len());
        for prev in candidates {
            for da in 0u64..8 {
                let a = (prev << 3) + da;
                let b = ((a & 7) ^ (a >> ((a & 7) ^ 7))) & 7;
                if b as u8 == p {
                    next.insert(a);
                }
            }
        }
        candidates = next;
    }
    let result = candidates.into_iter().min().unwrap();
    computer.a = result;
    let output = computer.run(program);
    assert_eq!(output, program);
    result
}

pub fn main() {
    let input: String = std::fs::read_to_string("data/2024/day17").unwrap();
    let (computer, program) = parse(&input);
    println!("part1: {}", part1(computer, &program));
    println!("part2: {}", part2(computer, &program));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = "
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
        let (computer, program) = parse(input);
        assert_eq!("4,6,3,5,6,3,5,2,1,0", part1(computer, &program));
    }

    #[test]
    fn case2() {
        let input: String = std::fs::read_to_string("data/2024/day17").unwrap();
        let (c, program) = parse(&input);
        assert_eq!(265652340990875, part2(c, &program));
    }
}
