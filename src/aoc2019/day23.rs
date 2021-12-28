use crate::day5::Intcode;

struct Nat {
    nics: Vec<Intcode>,
    packet: [i64; 2],
}

impl Nat {
    fn new(codes: &[i64], n: usize) -> Self {
        let nics = (0..n as i64)
            .map(|id| {
                let mut prog = Intcode::new(codes);
                prog.inputs.push_back(id);
                prog.inputs.push_back(-1);
                prog
            })
            .collect();
        Self {
            nics,
            packet: [0; 2],
        }
    }

    fn read_packet(&mut self, i: usize) -> Option<(i64, i64, i64)> {
        let nic = &mut self.nics[i];
        if nic.outputs.len() >= 3 {
            let addr = nic.outputs.pop_front().unwrap();
            let x = nic.outputs.pop_front().unwrap();
            let y = nic.outputs.pop_front().unwrap();
            Some((addr, x, y))
        } else {
            None
        }
    }
}

fn solve(codes: &[i64]) -> (i64, i64) {
    let mut nat = Nat::new(codes, 50);
    let mut p1 = -1;
    let mut last_y = -1;
    loop {
        let mut packets = vec![];
        let mut idle = true;
        for i in 0..nat.nics.len() {
            if nat.nics[i].inputs.is_empty() {
                if !nat.nics[i].wait_for_input {
                    idle = false;
                }
                nat.nics[i].inputs.push_back(-1);
            } else {
                idle = false;
            }
            nat.nics[i].run();
            while let Some((addr, x, y)) = nat.read_packet(i) {
                if addr == 255 {
                    if p1 == -1 {
                        p1 = y;
                    }
                    nat.packet = [x, y];
                } else {
                    packets.push((addr, x, y));
                }
            }
        }

        if idle {
            if nat.packet[1] == last_y {
                return (p1, nat.packet[1]);
            }
            last_y = nat.packet[1];
            nat.nics[0].inputs.extend(nat.packet);
        }

        for (addr, x, y) in packets {
            nat.nics[addr as usize].inputs.extend([x, y]);
        }
    }
}

pub fn main() {
    let codes = std::fs::read_to_string("data/2019/day23")
        .unwrap()
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|t| t.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let (p1, p2) = solve(&codes);
    println!("day 23 part1: {}", p1);
    println!("day 23 part2: {}", p2);
}
