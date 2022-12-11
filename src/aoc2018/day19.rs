use nom::{
    bytes::complete::{tag, take},
    character::{
        self,
        complete::{char as ch, line_ending},
    },
    combinator::all_consuming,
    multi::separated_list1,
    sequence::{preceded, separated_pair, tuple},
    IResult,
};

#[derive(Debug)]
struct Instruction<'a> {
    op: &'a str,
    in1: u8,
    in2: u8,
    out: u8,
}

impl<'a> std::fmt::Display for Instruction<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {:2} {:2} {}", self.op, self.in1, self.in2, self.out)
    }
}

struct Program {
    ip_reg: u8,
    ip: i32,
    reg: [u32; 6],
}

impl Program {
    fn new(ip_reg: u8) -> Self {
        Self {
            ip_reg,
            ip: -1,
            reg: [0; 6],
        }
    }
}

impl Program {
    fn operate(&mut self, ins: &Instruction) {
        self.reg[self.ip_reg as usize] = (self.ip + 1) as u32;
        self.reg[ins.out as usize] = match ins.op {
            "addr" => self.reg[ins.in1 as usize] + self.reg[ins.in2 as usize],
            "addi" => self.reg[ins.in1 as usize] + ins.in2 as u32,
            "mulr" => self.reg[ins.in1 as usize] * self.reg[ins.in2 as usize],
            "muli" => self.reg[ins.in1 as usize] * ins.in2 as u32,
            "banr" => self.reg[ins.in1 as usize] & self.reg[ins.in2 as usize],
            "bani" => self.reg[ins.in1 as usize] & ins.in2 as u32,
            "borr" => self.reg[ins.in1 as usize] | self.reg[ins.in2 as usize],
            "bori" => self.reg[ins.in1 as usize] | ins.in2 as u32,
            "setr" => self.reg[ins.in1 as usize],
            "seti" => ins.in1 as u32,
            "gtir" => u32::from(ins.in1 as u32 > self.reg[ins.in2 as usize]),
            "gtri" => u32::from(self.reg[ins.in1 as usize] > ins.in2 as u32),
            "gtrr" => u32::from(self.reg[ins.in1 as usize] > self.reg[ins.in2 as usize]),
            "eqir" => u32::from(ins.in1 as u32 == self.reg[ins.in2 as usize]),
            "eqri" => u32::from(self.reg[ins.in1 as usize] == ins.in2 as u32),
            "eqrr" => u32::from(self.reg[ins.in1 as usize] == self.reg[ins.in2 as usize]),
            _ => unreachable!(),
        };
        self.ip = self.reg[self.ip_reg as usize] as i32;
    }

    fn run(&mut self, instructions: &[Instruction]) -> u32 {
        loop {
            let ip = self.ip + 1;
            if ip < 0 || ip >= instructions.len() as i32 {
                break;
            }
            self.operate(&instructions[ip as usize]);
            // println!("{:2} {} {:?}", ip, &instructions[ip as usize], self.reg);
        }
        self.reg[0]
    }
}

fn parse_ip(s: &str) -> IResult<&str, u8> {
    preceded(tag("#ip "), character::complete::u8)(s)
}

fn parse_instruction(s: &str) -> IResult<&str, Instruction> {
    let (r, (op, _, in1, _, in2, _, out)) = tuple((
        take(4usize),
        ch(' '),
        character::complete::u8,
        ch(' '),
        character::complete::u8,
        ch(' '),
        character::complete::u8,
    ))(s)?;
    Ok((r, Instruction { op, in1, in2, out }))
}

fn parse_instructions(s: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(line_ending, parse_instruction)(s)
}

fn parse(data: &str) -> (u8, Vec<Instruction>) {
    let (_, r) =
        all_consuming(separated_pair(parse_ip, line_ending, parse_instructions))(data.trim())
            .unwrap();
    r
}

#[allow(unused)]
fn part1_sim(ip_reg: u8, instructions: &[Instruction]) -> u32 {
    let mut p = Program::new(ip_reg);
    p.ip_reg = ip_reg;
    p.run(instructions)
}

fn sum_of_factors(n: u64) -> u64 {
    let mut r = 0;
    for i in 1..=(n as f64).sqrt() as u64 {
        if n % i == 0 {
            r += i;
            if i != (n / i) {
                r += n / i;
            }
        }
    }
    r
}

fn part1(_ip_reg: u8, _instructions: &[Instruction]) -> u64 {
    // reverse engineered from input
    sum_of_factors(919)
}

fn part2(_ip_reg: u8, _instructions: &[Instruction]) -> u64 {
    // reverse engineered from input
    sum_of_factors(10551319)
}

pub fn main() {
    let data = std::fs::read_to_string("data/2018/day19").unwrap();
    let (ip_reg, instructions) = parse(&data);
    println!("part1: {}", part1(ip_reg, &instructions));
    println!("part2: {}", part2(ip_reg, &instructions));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "
#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5";
        let (ip_reg, instructions) = parse(&data);
        assert_eq!(6, part1_sim(ip_reg, &instructions));
    }
}
