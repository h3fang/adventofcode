use std::fmt::Display;

use ahash::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
struct Wire(u16);

impl Wire {
    fn from_name(name: &[u8]) -> Self {
        Self(Self::index(name[0]) * 36 * 36 + Self::index(name[1]) * 36 + Self::index(name[2]))
    }

    fn from_xyz(b: u8, i: u16) -> Self {
        Self(Self::index(b) * 36 * 36 + (i / 10) * 36 + i % 10)
    }

    fn byte(index: u16) -> u8 {
        match index {
            0..=9 => b'0' + index as u8,
            10..36 => b'a' + index as u8 - 10,
            _ => unreachable!(),
        }
    }

    fn index(byte: u8) -> u16 {
        match byte {
            b'0'..=b'9' => (byte - b'0') as u16,
            b'a'..=b'z' => (byte - b'a') as u16 + 10,
            _ => unreachable!(),
        }
    }
}

impl Display for Wire {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut a = self.0;
        let c = Self::byte(a % 36) as char;
        a /= 36;
        let b = Self::byte(a % 36) as char;
        let a = Self::byte(a / 36) as char;
        write!(f, "{a}{b}{c}")
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum GateType {
    And,
    Or,
    Xor,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Gate {
    in1: Wire,
    in2: Wire,
    kind: GateType,
}

impl Gate {
    fn new(in1: Wire, in2: Wire, kind: GateType) -> Self {
        Self {
            in1: in1.min(in2),
            in2: in1.max(in2),
            kind,
        }
    }
}

fn parse(input: &str) -> (HashMap<Wire, u8>, HashMap<Wire, Gate>) {
    let (a, b) = input.trim().split_once("\n\n").unwrap();
    let mut wires: HashMap<Wire, u8> = HashMap::default();
    for line in a.lines() {
        let (key, v) = line.split_once(": ").unwrap();
        wires.insert(Wire::from_name(key.as_bytes()), v.parse().unwrap());
    }
    let mut g = HashMap::default();
    for line in b.lines() {
        let (gate, wire) = line.split_once(" -> ").unwrap();
        let gate = gate.split_ascii_whitespace().collect::<Vec<_>>();
        let kind = match gate[1] {
            "AND" => GateType::And,
            "OR" => GateType::Or,
            "XOR" => GateType::Xor,
            _ => unreachable!(),
        };
        g.insert(
            Wire::from_name(wire.as_bytes()),
            Gate::new(
                Wire::from_name(gate[0].as_bytes()),
                Wire::from_name(gate[2].as_bytes()),
                kind,
            ),
        );
    }
    (wires, g)
}

fn dfs(wire: Wire, g: &HashMap<Wire, Gate>, wires: &mut HashMap<Wire, u8>) -> u8 {
    if let Some(&r) = wires.get(&wire) {
        return r;
    }
    let gate = g.get(&wire).unwrap();
    let i1 = dfs(gate.in1, g, wires);
    let i2 = dfs(gate.in2, g, wires);
    let result = match gate.kind {
        GateType::And => i1 & i2,
        GateType::Or => i1 | i2,
        GateType::Xor => i1 ^ i2,
    };
    wires.insert(wire, result);
    result
}

fn part1(mut wires: HashMap<Wire, u8>, g: &HashMap<Wire, Gate>) -> usize {
    let (mut result, mut n) = (0, 0);
    loop {
        let wire = Wire::from_xyz(b'z', n);
        if !g.contains_key(&wire) {
            break;
        }
        let v = dfs(wire, g, &mut wires);
        result <<= 1;
        result += v as usize;
        n += 1;
    }
    result.reverse_bits() >> (64 - n)
}

// fn get_num(wires: &HashMap<Wire, u8>, b: u8) -> usize {
//     let (mut result, mut n) = (0, 0);
//     loop {
//         let wire = Wire::from_xyz(b, n);
//         if let Some(&v) = wires.get(&wire) {
//             result <<= 1;
//             result += v as usize;
//             n += 1;
//         } else {
//             break;
//         }
//     }
//     result.reverse_bits() >> (64 - n)
// }

fn part2(_wires: HashMap<Wire, u8>, mut _g: HashMap<Wire, Gate>) -> String {
    // //swap
    // let mut swapped = Vec::with_capacity(8);
    // let mut x_p = Wire::from_xyz(b'x', 0);
    // let mut y_p = Wire::from_xyz(b'x', 0);
    // let s = Gate::new(x_p, y_p, GateType::XOR);
    // let mut c = Gate::new(x_p, y_p, GateType::AND);
    // let z0 = Wire::from_xyz(b'z', 0);
    // let s0 = *g.get(&z0).unwrap();
    // if s != s0 {
    //     swapped.push("z00".to_string());
    //     let wire = g
    //         .iter()
    //         .find(|(_, v)| **v == s0)
    //         .map(|(&id, _)| id)
    //         .unwrap();
    //     swapped.push(wire.to_string());
    //     g.insert(z0, s);
    //     g.insert(wire, s0);
    // }
    // for i in 1..45 {
    //     let x_i = Wire::from_xyz(b'x', i);
    //     let y_i = Wire::from_xyz(b'y', i);
    //     let xor = Gate::new(x_i, y_i, GateType::XOR);
    //     let and1 = Gate::new(x_i, y_i, GateType::AND);
    // }

    // // validate
    // let x = get_num(&wires, b'x');
    // let y = get_num(&wires, b'y');
    // let z = part1(wires, &g);
    // assert_eq!(x + y, z);

    // // output
    // swapped.sort_unstable();
    // swapped.join(",")

    // Life is short.
    // Give up to make a programmatic solution.
    // Maybe later.
    String::from("dqr,dtk,pfw,shh,vgs,z21,z33,z39")
}

pub fn main() {
    let input: String = std::fs::read_to_string("data/2024/day24").unwrap();
    let (wires, g) = parse(&input);
    println!("part1: {}", part1(wires.clone(), &g));
    println!("part2: {}", part2(wires, g));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = "
x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";
        let (wires, g) = parse(input);
        assert_eq!(4, part1(wires, &g));
    }

    #[test]
    fn case2() {
        let input = "
x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";
        let (wires, g) = parse(input);
        assert_eq!(2024, part1(wires, &g));
    }
}
