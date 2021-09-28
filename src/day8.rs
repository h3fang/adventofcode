use regex::Regex;

#[derive(Clone)]
enum Instruction {
    Jmp(i32),
    Acc(i32),
    Nop(i32),
}

fn parse(line: &str, re: &Regex) -> Instruction {
    if let Some(cap) = re.captures(line) {
        let op = &cap[1];
        let arg = cap[2].parse::<i32>().expect("invalid argument");
        match op {
            "jmp" => Instruction::Jmp(arg),
            "acc" => Instruction::Acc(arg),
            "nop" => Instruction::Nop(arg),
            _ => panic!("invalid op code: {}", op),
        }
    } else {
        panic!("invalid line: {}", line);
    }
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
    let r1 = Regex::new(r"^(\w{3}) ([+|-]\d+)$").unwrap();

    let mut instructions = include_str!("../data/day8")
        .lines()
        .map(|line| parse(line, &r1))
        .collect::<Vec<_>>();

    // part 1
    let acc = run(&instructions, 0).0;
    println!("day8 part1: {}", acc);

    // part 2
    let acc = find_bug(&mut instructions);
    println!("day8 part2: {}", acc);
}
