use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, one_of},
    combinator::{eof, map_res, recognize},
    IResult, Parser,
};

#[derive(Clone)]
enum Instruction {
    Jmp(i32),
    Acc(i32),
    Nop(i32),
}

fn parse(line: &str) -> IResult<&str, Instruction> {
    let operator = alt((tag("jmp"), tag("acc"), tag("nop")));
    fn number(input: &str) -> IResult<&str, i32> {
        let num = (one_of("+-"), digit1);
        map_res(recognize(num), str::parse).parse(input)
    }
    let (_, (op, _, num, _)) = (operator, char(' '), number, eof).parse(line)?;
    let ins = match op {
        "jmp" => Instruction::Jmp(num),
        "acc" => Instruction::Acc(num),
        "nop" => Instruction::Nop(num),
        _ => panic!("invalid op code: {}", op),
    };
    Ok(("", ins))
}

fn run(instructions: &[Instruction], mut pos: usize) -> (i32, usize) {
    let mut acc = 0;
    let n = instructions.len();
    let mut visited = vec![false; n];
    while pos < n && !visited[pos] {
        let offset: i32 = match instructions[pos] {
            Instruction::Acc(value) => {
                acc += value;
                1
            }
            Instruction::Jmp(value) => value,
            Instruction::Nop(_) => 1,
        };
        visited[pos] = true;
        if pos == n - 1 {
            break;
        }
        pos = (pos as i32 + offset) as usize;
    }
    (acc, pos)
}

fn find_bug(instructions: &mut [Instruction]) -> i32 {
    let len = instructions.len() - 1;
    for i in 0..=len {
        match instructions[i] {
            Instruction::Jmp(v) => instructions[i] = Instruction::Nop(v),
            Instruction::Nop(v) => instructions[i] = Instruction::Jmp(v),
            Instruction::Acc(_) => continue,
        }

        let (acc, pos) = run(instructions, 0);
        if pos == len {
            return acc;
        }

        match instructions[i] {
            Instruction::Jmp(v) => instructions[i] = Instruction::Nop(v),
            Instruction::Nop(v) => instructions[i] = Instruction::Jmp(v),
            Instruction::Acc(_) => continue,
        }
    }
    panic!("no solutions found")
}

pub fn main() {
    let mut instructions = std::fs::read_to_string("data/2020/day8")
        .unwrap()
        .lines()
        .map(|line| parse(line).map(|r| r.1).unwrap())
        .collect::<Vec<_>>();

    // part 1
    let acc = run(&instructions, 0).0;
    println!("day8 part1: {}", acc);

    // part 2
    let acc = find_bug(&mut instructions);
    println!("day8 part2: {}", acc);
}
