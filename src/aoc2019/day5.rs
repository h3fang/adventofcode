fn parameter_modes(mut n: i64) -> Vec<u8> {
    let mut r = Vec::with_capacity(3);
    for _ in 0..3 {
        r.push((n % 10) as u8);
        n /= 10;
    }
    r
}

pub struct Intcode {
    codes: Vec<i64>,
    ip: usize,
    relative_base: i64,
    pub inputs: Vec<i64>,
    pub output: i64,
}

impl Intcode {
    pub fn new(codes: &[i64]) -> Self {
        let mut codes = codes.to_vec();
        codes.resize(100000, 0);
        Self {
            codes: codes.to_vec(),
            ip: 0,
            relative_base: 0,
            inputs: Vec::new(),
            output: 0,
        }
    }

    fn position(&self, modes: &[u8], i: usize) -> usize {
        match modes[i] {
            0 => self.codes[self.ip + i + 1] as usize,
            1 => self.ip + i + 1,
            2 => (self.relative_base + self.codes[self.ip + i + 1]) as usize,
            x => panic!("invalid parameter mode {}", x),
        }
    }

    fn param(&self, modes: &[u8], i: usize) -> i64 {
        self.codes[self.position(modes, i)]
    }

    pub fn run(&mut self) {
        loop {
            let code = self.codes[self.ip];
            let opcode = code % 100;
            if opcode == 99 {
                break;
            }
            let modes = parameter_modes(code / 100);
            match opcode {
                1 => {
                    let a = self.param(&modes, 0);
                    let b = self.param(&modes, 1);
                    let c = self.position(&modes, 2);
                    self.codes[c] = a + b;
                    self.ip += 4;
                }
                2 => {
                    let a = self.param(&modes, 0);
                    let b = self.param(&modes, 1);
                    let c = self.position(&modes, 2);
                    self.codes[c] = a * b;
                    self.ip += 4;
                }
                3 => {
                    let c = self.position(&modes, 0);
                    self.codes[c] = self.inputs.pop().expect("not enough input");
                    self.ip += 2;
                }
                4 => {
                    self.output = self.param(&modes, 0);
                    self.ip += 2;
                    break;
                }
                5 => {
                    let a = self.param(&modes, 0);
                    if a != 0 {
                        self.ip = self.param(&modes, 1) as usize;
                    } else {
                        self.ip += 3;
                    }
                }
                6 => {
                    let a = self.param(&modes, 0);
                    if a == 0 {
                        self.ip = self.param(&modes, 1) as usize;
                    } else {
                        self.ip += 3;
                    }
                }
                7 => {
                    let a = self.param(&modes, 0);
                    let b = self.param(&modes, 1);
                    let c = self.position(&modes, 2);
                    if a < b {
                        self.codes[c] = 1;
                    } else {
                        self.codes[c] = 0;
                    }
                    self.ip += 4;
                }
                8 => {
                    let a = self.param(&modes, 0);
                    let b = self.param(&modes, 1);
                    let c = self.position(&modes, 2);
                    if a == b {
                        self.codes[c] = 1;
                    } else {
                        self.codes[c] = 0;
                    }
                    self.ip += 4;
                }
                9 => {
                    self.relative_base += self.param(&modes, 0);
                    self.ip += 2;
                }
                x => panic!("invalid opcode {}", x),
            }
        }
    }

    pub fn is_halted(&self) -> bool {
        self.codes[self.ip] % 100 == 99
    }

    pub fn run_till_halt(&mut self) {
        while !self.is_halted() {
            self.run();
        }
    }
}

pub fn main() {
    let codes = std::fs::read_to_string("data/2019/day5")
        .unwrap()
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|t| t.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut code = Intcode::new(&codes);
    code.inputs = vec![1];
    code.run_till_halt();
    println!("day5 part1: {}", code.output);

    let mut code = Intcode::new(&codes);
    code.inputs = vec![5];
    code.run();
    println!("day5 part2: {}", code.output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let codes = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99"
            .split(',')
            .map(|t| t.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        let mut code = Intcode::new(&codes);
        code.inputs = vec![7];
        code.run();
        assert_eq!(999, code.output);

        let mut code = Intcode::new(&codes);
        code.inputs = vec![8];
        code.run();
        assert_eq!(1000, code.output);

        let mut code = Intcode::new(&codes);
        code.inputs = vec![10];
        code.run();
        assert_eq!(1001, code.output);
    }
}
