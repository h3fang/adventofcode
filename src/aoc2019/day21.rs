use crate::day5::Intcode;

fn execute(codes: &[i64], script: &str) -> usize {
    let mut p = Intcode::new(codes);
    p.inputs.extend(script.as_bytes().iter().map(|b| *b as i64));
    let mut rendered = vec![];
    while !p.is_halted() {
        p.run();
        let o = p.outputs.pop_front().unwrap();
        if o <= u8::MAX as i64 {
            rendered.push(o as u8);
        } else {
            return o as usize;
        }
    }
    println!("{}", unsafe { String::from_utf8_unchecked(rendered) });
    0
}

pub fn main() {
    let codes = std::fs::read_to_string("data/2019/day21")
        .unwrap()
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|t| t.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let p1 = "OR A T\nAND B T\nAND C T\nNOT T J\nAND D J\nWALK\n";
    let p2 = "OR A T\nAND B T\nAND C T\nNOT T T\nAND D T\nOR E J\nOR H J\nAND T J\nRUN\n";
    println!("day 21 part1: {}", execute(&codes, p1));
    println!("day 21 part2: {}", execute(&codes, p2));
}
