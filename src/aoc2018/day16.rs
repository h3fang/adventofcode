use nom::{
    bytes::complete::tag,
    character::{
        complete::{char as ch, digit1, line_ending},
        streaming::space1,
    },
    combinator::{map_res, recognize},
    multi::{count, separated_list1},
    sequence::tuple,
    IResult,
};

type Vec4 = [i32; 4];

#[derive(Debug)]
struct Sample {
    before: Vec4,
    instruction: Vec4,
    after: Vec4,
}

fn operate(reg: &mut [i32], ins: &Vec4) {
    reg[ins[3] as usize] = match ins[0] {
        0 => reg[ins[1] as usize] + reg[ins[2] as usize],
        1 => reg[ins[1] as usize] + ins[2],
        2 => reg[ins[1] as usize] * reg[ins[2] as usize],
        3 => reg[ins[1] as usize] * ins[2],
        4 => reg[ins[1] as usize] & reg[ins[2] as usize],
        5 => reg[ins[1] as usize] & ins[2],
        6 => reg[ins[1] as usize] | reg[ins[2] as usize],
        7 => reg[ins[1] as usize] | ins[2],
        8 => reg[ins[1] as usize],
        9 => ins[1],
        10 => i32::from(ins[1] > reg[ins[2] as usize]),
        11 => i32::from(reg[ins[1] as usize] > ins[2]),
        12 => i32::from(reg[ins[1] as usize] > reg[ins[2] as usize]),
        13 => i32::from(ins[1] == reg[ins[2] as usize]),
        14 => i32::from(reg[ins[1] as usize] == ins[2]),
        15 => i32::from(reg[ins[1] as usize] == reg[ins[2] as usize]),
        _ => unreachable!(),
    };
}

fn parse_num(input: &str) -> IResult<&str, i32> {
    map_res(recognize(digit1), str::parse)(input)
}

fn parse_registers(input: &str) -> IResult<&str, Vec<i32>> {
    let (r, (_, nums, _)) =
        tuple((ch('['), separated_list1(tag(", "), parse_num), ch(']')))(input)?;
    Ok((r, nums))
}

fn parse_registers_before(input: &str) -> IResult<&str, Vec4> {
    let (r, (_, _, nums)) = tuple((tag("Before:"), space1, parse_registers))(input)?;
    let nums = [nums[0], nums[1], nums[2], nums[3]];
    Ok((r, nums))
}

fn parse_registers_after(input: &str) -> IResult<&str, Vec4> {
    let (r, (_, _, nums)) = tuple((tag("After:"), space1, parse_registers))(input)?;
    let nums = [nums[0], nums[1], nums[2], nums[3]];
    Ok((r, nums))
}

fn parse_instruction(input: &str) -> IResult<&str, Vec4> {
    let (r, nums) = (separated_list1(ch(' '), parse_num))(input)?;
    let nums = [nums[0], nums[1], nums[2], nums[3]];
    Ok((r, nums))
}

fn parse_sample(input: &str) -> IResult<&str, Sample> {
    let (r, (before, _, instruction, _, after)) = tuple((
        parse_registers_before,
        line_ending,
        parse_instruction,
        line_ending,
        parse_registers_after,
    ))(input)?;
    Ok((
        r,
        Sample {
            before,
            instruction,
            after,
        },
    ))
}

fn parse_samples(input: &str) -> IResult<&str, Vec<Sample>> {
    separated_list1(count(line_ending, 2), parse_sample)(input)
}

fn parse_program(input: &str) -> IResult<&str, Vec<Vec4>> {
    separated_list1(line_ending, parse_instruction)(input)
}

fn parse(data: &str) -> (Vec<Sample>, Vec<Vec4>) {
    let (_, (samples, _, program)) =
        tuple((parse_samples, count(line_ending, 4), parse_program))(data).unwrap();
    (samples, program)
}

fn try_opcode(op: i32, sample: &Sample) -> bool {
    let mut ins = sample.instruction;
    ins[0] = op;
    let mut reg = sample.before;
    operate(&mut reg, &ins);
    reg == sample.after
}

fn num_of_candidates(sample: &Sample) -> usize {
    (0..16)
        .into_iter()
        .filter(|&op| try_opcode(op, sample))
        .count()
}

fn part1(samples: &[Sample]) -> usize {
    samples.iter().filter(|s| num_of_candidates(s) >= 3).count()
}

fn backtrack(
    samples: &[Sample],
    possible: &[u16],
    used: u16,
    i: usize,
    mapping: &mut [i32],
) -> bool {
    if i == possible.len() {
        return samples.iter().all(|s| {
            let op = mapping[s.instruction[0] as usize];
            try_opcode(op, s)
        });
    }
    let p = possible[i];
    for j in 0..16 {
        if (used & (1 << j) == 0) && (p & (1 << j) > 0) {
            mapping[i] = j;
            if backtrack(samples, possible, used | (1 << j), i + 1, mapping) {
                return true;
            }
        }
    }
    false
}

fn part2(samples: &[Sample], program: Vec<Vec4>) -> i32 {
    let mut possible = [0; 16];
    for (i, p) in possible.iter_mut().enumerate() {
        for op in 0..16 {
            if samples
                .iter()
                .filter(|s| s.instruction[0] == i as i32)
                .all(|s| try_opcode(op, s))
            {
                *p |= 1 << op;
            }
        }
    }

    let mut mapping = [0; 16];
    backtrack(samples, &possible, 0, 0, &mut mapping);

    // for (k, v) in possible.iter().enumerate() {
    //     println!(
    //         "{:2} {:016b} {:2} {:016b}",
    //         k,
    //         v,
    //         mapping[k as usize],
    //         1 << mapping[k as usize]
    //     );
    // }

    let mut reg = [0; 4];
    for mut ins in program {
        let op = mapping[ins[0] as usize];
        ins[0] = op;
        operate(&mut reg, &ins);
    }
    reg[0]
}

pub fn main() {
    let data = std::fs::read_to_string("data/2018/day16").unwrap();
    let (samples, program) = parse(&data);
    println!("part1: {}", part1(&samples));
    println!("part2: {}", part2(&samples, program));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "
Before: [3, 2, 1, 1]
9 2 1 2
After:  [3, 2, 2, 1]"
            .to_string();
        let sample = parse_sample(data.trim_start()).unwrap().1;
        assert_eq!(3, num_of_candidates(&sample));
    }
}
