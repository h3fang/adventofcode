use std::collections::{hash_map::Entry, VecDeque};

use ahash::HashMap;

#[derive(Debug, Clone, Copy, Default)]
enum ModuleType {
    #[default]
    Untyped,
    Broadcaster,
    FlipFlop,
    Conjuction,
}

#[derive(Debug, Default, Clone, Copy)]
struct Module {
    kind: ModuleType,
    // when module type is FlipFlop, state represents the on/off state,
    // when module type is Conjuction, state represents it's input pulses,
    // (since total number of modules is less than 64, for the examples and personal input)
    state: usize,
    // bit mask of the inputs
    inputs: usize,
    // bit mask of the outputs
    outputs: usize,
}

impl Module {
    fn new(kind: ModuleType) -> Self {
        Self {
            kind,
            ..Default::default()
        }
    }
}

fn get_index<'a>(map: &mut HashMap<&'a str, usize>, name: &'a str, idx: &mut usize) -> usize {
    match name {
        "broadcaster" => 0,
        "rx" => 1,
        name => match map.entry(name) {
            Entry::Occupied(e) => *e.get(),
            Entry::Vacant(e) => {
                *idx += 1;
                e.insert(*idx);
                *idx
            }
        },
    }
}

fn parse(data: &str) -> (HashMap<&str, usize>, Vec<Module>) {
    let lines = data.trim().lines().collect::<Vec<_>>();
    let mut map = HashMap::<&str, usize>::default();
    let mut idx = 1;
    let mut modules = vec![Module::new(ModuleType::Broadcaster), Module::default()];
    for line in lines {
        let (m, outpus) = line.split_once("->").unwrap();
        let m = m.trim();
        let i = match m.as_bytes()[0] {
            b'%' => {
                let i = get_index(&mut map, &m[1..], &mut idx);
                if i >= modules.len() {
                    modules.resize_with(i + 1, Module::default);
                }
                modules[i].kind = ModuleType::FlipFlop;
                i
            }
            b'&' => {
                let i = get_index(&mut map, &m[1..], &mut idx);
                if i >= modules.len() {
                    modules.resize_with(i + 1, Module::default);
                }
                modules[i].kind = ModuleType::Conjuction;
                i
            }
            b'b' => 0,
            _ => unreachable!(),
        };
        for t in outpus.split(',') {
            let t = t.trim();
            let j = get_index(&mut map, t, &mut idx);
            if j >= modules.len() {
                modules.resize_with(j + 1, Module::default);
            }
            modules[j].inputs |= 1 << i;
            modules[i].outputs |= 1 << j;
        }
    }
    (map, modules)
}

fn process(
    modules: &mut [Module],
    q: &mut VecDeque<(usize, usize, usize)>,
    p: usize,
    i: usize,
    j: usize,
) {
    match modules[j].kind {
        ModuleType::Untyped => {}
        ModuleType::Broadcaster => {
            for k in 0..usize::BITS {
                if (1 << k) & modules[j].outputs > 0 {
                    q.push_back((p, j, k as usize));
                }
            }
        }
        ModuleType::FlipFlop => {
            if p == 0 {
                modules[j].state = 1 - modules[j].state;
                let p = modules[j].state;
                for k in 0..usize::BITS {
                    if (1 << k) & modules[j].outputs > 0 {
                        q.push_back((p, j, k as usize));
                    }
                }
            }
        }
        ModuleType::Conjuction => {
            if p == 0 {
                modules[j].state &= !(1 << i);
            } else {
                modules[j].state |= 1 << i;
            }
            let p = usize::from(modules[j].state != modules[j].inputs);
            for k in 0..usize::BITS {
                if (1 << k) & modules[j].outputs > 0 {
                    q.push_back((p, j, k as usize));
                }
            }
        }
    }
}

fn part1(mut modules: Vec<Module>) -> usize {
    let mut pulses = [0; 2];
    for _ in 0..1000 {
        let mut q = VecDeque::with_capacity(256);
        q.push_back((0, 0, 0));
        while let Some((p, i, j)) = q.pop_front() {
            pulses[p] += 1;
            process(&mut modules, &mut q, p, i, j);
        }
    }
    pulses[0] * pulses[1]
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

// this is not a generalized solution
fn part2(mut modules: Vec<Module>) -> usize {
    let i = modules[1].inputs.trailing_zeros();
    let m = &modules[i as usize];
    let mut inputs = m.inputs;
    let mut cycles = vec![];
    for k in 1.. {
        let mut q = VecDeque::with_capacity(256);
        q.push_back((0, 0, 0));
        while let Some((p, i, j)) = q.pop_front() {
            process(&mut modules, &mut q, p, i, j);
            if (1 << j) & inputs > 0 && modules[j].state != modules[j].inputs {
                cycles.push(k);
                inputs &= !(1 << j);
                if inputs == 0 {
                    return cycles.into_iter().fold(1, lcm);
                }
            }
        }
    }
    unreachable!()
}

pub fn main() {
    let data = std::fs::read_to_string("data/2023/day20").unwrap();
    let (_, modules) = parse(&data);
    println!("part1: {}", part1(modules.clone()));
    println!("part2: {}", part2(modules));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = r"
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
        let (_, modules) = parse(data);
        assert_eq!(32000000, part1(modules));
    }

    #[test]
    fn case2() {
        let data = r"
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";
        let (_, modules) = parse(data);
        assert_eq!(11687500, part1(modules));
    }

    #[test]
    fn case3() {
        let data = std::fs::read_to_string("data/2023/day20").unwrap();
        let (_, modules) = parse(&data);
        assert_eq!(731517480, part1(modules.clone()));
        assert_eq!(244178746156661, part2(modules));
    }
}
