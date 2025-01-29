use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self as cc, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::preceded,
    IResult,
    Parser,
};

#[derive(Debug)]
enum Instruction {
    Addx(i64),
    Noop,
}

fn parse_addx(s: &str) -> IResult<&str, Instruction> {
    let (r, arg) = preceded(tag("addx "), cc::i64).parse(s)?;
    Ok((r, Instruction::Addx(arg)))
}

fn parse_noop(s: &str) -> IResult<&str, Instruction> {
    let (r, _) = tag("noop")(s)?;
    Ok((r, Instruction::Noop))
}

fn parse_instructions(s: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(line_ending, alt((parse_addx, parse_noop))).parse(s)
}

fn parse(data: &str) -> Vec<Instruction> {
    all_consuming(parse_instructions).parse(data.trim()).unwrap().1
}

fn part1(instructions: &[Instruction]) -> i64 {
    let mut x = 1;
    let mut cycles = 1;
    let mut result = 0;
    let mut signal = |cycles: i64, x: i64| {
        if cycles <= 220 && (cycles - 20) % 40 == 0 {
            result += cycles * x;
        }
    };
    for ins in instructions {
        signal(cycles, x);
        match ins {
            Instruction::Addx(v) => {
                cycles += 1;
                signal(cycles, x);
                cycles += 1;
                x += v;
            }
            Instruction::Noop => {
                cycles += 1;
            }
        }
    }
    result
}

fn part2(instructions: &[Instruction]) -> String {
    let mut crt = [b'.'; 240];
    let mut x = 1;
    let mut cycles = 1;
    let mut draw = |x: i64, cycles: i64| {
        let c = (cycles - 1) % 40;
        if (x - 1..=x + 1).contains(&c) {
            crt[(cycles - 1) as usize] = b'#';
        }
    };
    for ins in instructions {
        draw(x, cycles);
        match ins {
            Instruction::Addx(v) => {
                cycles += 1;
                draw(x, cycles);
                cycles += 1;
                x += v;
            }
            Instruction::Noop => {
                cycles += 1;
            }
        }
    }
    (0..6)
        .map(|i| unsafe { std::str::from_utf8_unchecked(&crt[i * 40..(i + 1) * 40]) })
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn main() {
    let data = std::fs::read_to_string("data/2022/day10").unwrap();
    let program = parse(&data);
    println!("part1: {}", part1(&program));
    println!("part2:");
    println!("{}", part2(&program));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
        let program = parse(data);
        assert_eq!(13140, part1(&program));

        let p2 = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";
        assert_eq!(p2, part2(&program));
    }
}
