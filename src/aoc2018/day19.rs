use nom::{
    bytes::complete::{tag, take},
    character::{
        self,
        complete::{char as ch, line_ending},
    },
    combinator::all_consuming,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult, Parser,
};

#[derive(Debug)]
struct Instruction<'a> {
    op: &'a str,
    in1: u64,
    in2: u64,
    out: u64,
}

impl std::fmt::Display for Instruction<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {:2} {:2} {}", self.op, self.in1, self.in2, self.out)
    }
}

struct Program {
    ip_reg: usize,
    ip: usize,
    reg: [u64; 6],
}

impl Program {
    fn new(ip_reg: usize) -> Self {
        Self {
            ip_reg,
            ip: 0,
            reg: [0; 6],
        }
    }

    fn operate(&mut self, ins: &Instruction) {
        self.reg[self.ip_reg] = self.ip as u64;
        self.reg[ins.out as usize] = match ins.op {
            "addr" => self.reg[ins.in1 as usize] + self.reg[ins.in2 as usize],
            "addi" => self.reg[ins.in1 as usize] + ins.in2,
            "mulr" => self.reg[ins.in1 as usize] * self.reg[ins.in2 as usize],
            "muli" => self.reg[ins.in1 as usize] * ins.in2,
            "banr" => self.reg[ins.in1 as usize] & self.reg[ins.in2 as usize],
            "bani" => self.reg[ins.in1 as usize] & ins.in2,
            "borr" => self.reg[ins.in1 as usize] | self.reg[ins.in2 as usize],
            "bori" => self.reg[ins.in1 as usize] | ins.in2,
            "setr" => self.reg[ins.in1 as usize],
            "seti" => ins.in1,
            "gtir" => u64::from(ins.in1 > self.reg[ins.in2 as usize]),
            "gtri" => u64::from(self.reg[ins.in1 as usize] > ins.in2),
            "gtrr" => u64::from(self.reg[ins.in1 as usize] > self.reg[ins.in2 as usize]),
            "eqir" => u64::from(ins.in1 == self.reg[ins.in2 as usize]),
            "eqri" => u64::from(self.reg[ins.in1 as usize] == ins.in2),
            "eqrr" => u64::from(self.reg[ins.in1 as usize] == self.reg[ins.in2 as usize]),
            _ => unreachable!(),
        };
        self.ip = self.reg[self.ip_reg] as usize + 1;
    }
}

fn parse_ip(s: &str) -> IResult<&str, u8> {
    preceded(tag("#ip "), character::complete::u8).parse(s)
}

fn parse_instruction(s: &str) -> IResult<&str, Instruction> {
    let (r, (op, _, in1, _, in2, _, out)) = (
        take(4usize),
        ch(' '),
        character::complete::u64,
        ch(' '),
        character::complete::u64,
        ch(' '),
        character::complete::u64,
    )
        .parse(s)?;
    Ok((r, Instruction { op, in1, in2, out }))
}

fn parse_instructions(s: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(line_ending, parse_instruction).parse(s)
}

#[allow(unused)]
fn parse(data: &str) -> (u8, Vec<Instruction>) {
    let (_, r) = all_consuming(separated_pair(parse_ip, line_ending, parse_instructions))
        .parse(data.trim())
        .unwrap();
    r
}

#[allow(unused)]
fn part1_sim(ip_reg: usize, instructions: &[Instruction]) -> u64 {
    let mut p = Program::new(ip_reg);
    while p.ip < instructions.len() {
        p.operate(&instructions[p.ip]);
    }
    p.reg[0]
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

/// reverse engineered solution, dependent on input
pub fn main() {
    println!("part1: {}", sum_of_factors(919));
    println!("part2: {}", sum_of_factors(10551319));
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
        let (ip_reg, instructions) = parse(data);
        assert_eq!(6, part1_sim(ip_reg as usize, &instructions));
    }
}
