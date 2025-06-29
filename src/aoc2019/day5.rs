use std::collections::VecDeque;

fn parameter_modes(mut n: i64) -> [u8; 3] {
    let mut r = [0; 3];
    for m in &mut r {
        *m = (n % 10) as u8;
        n /= 10;
    }
    r
}

#[derive(Clone)]
pub struct Intcode {
    codes: Vec<i64>,
    ip: usize,
    relative_base: i64,
    pub inputs: VecDeque<i64>,
    pub outputs: VecDeque<i64>,
    pub wait_for_input: bool,
}

impl Intcode {
    pub fn new(codes: &[i64]) -> Self {
        let mut codes = codes.to_vec();
        codes.resize(10000, 0);
        Self {
            codes: codes.to_vec(),
            ip: 0,
            relative_base: 0,
            inputs: Default::default(),
            outputs: Default::default(),
            wait_for_input: false,
        }
    }

    fn position(&self, modes: &[u8], i: usize) -> usize {
        match modes[i] {
            0 => self.codes[self.ip + i + 1] as usize,
            1 => self.ip + i + 1,
            2 => (self.relative_base + self.codes[self.ip + i + 1]) as usize,
            x => panic!("invalid parameter mode {x}"),
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
                    if let Some(num) = self.inputs.pop_front() {
                        self.codes[c] = num;
                        self.ip += 2;
                        self.wait_for_input = false;
                    } else {
                        self.wait_for_input = true;
                        break;
                    }
                }
                4 => {
                    self.outputs.push_back(self.param(&modes, 0));
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
                x => panic!("invalid opcode {x}"),
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

    pub fn run_till_input(&mut self) {
        loop {
            self.run();
            if self.is_halted() || self.wait_for_input {
                break;
            }
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
    code.inputs.push_back(1);
    code.run_till_halt();
    println!("day5 part1: {}", code.outputs.pop_back().unwrap());

    let mut code = Intcode::new(&codes);
    code.inputs.push_back(5);
    code.run();
    println!("day5 part2: {}", code.outputs.pop_back().unwrap());
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
        code.inputs.push_back(7);
        code.run();
        assert_eq!(999, code.outputs.pop_front().unwrap());

        let mut code = Intcode::new(&codes);
        code.inputs.push_back(8);
        code.run();
        assert_eq!(1000, code.outputs.pop_front().unwrap());

        let mut code = Intcode::new(&codes);
        code.inputs.push_back(10);
        code.run();
        assert_eq!(1001, code.outputs.pop_front().unwrap());
    }
}
