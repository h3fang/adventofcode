enum Instruction {
    North(i32),
    East(i32),
    South(i32),
    West(i32),
    Forward(i32),
    Left(i32),
    Right(i32),
}

#[derive(Debug)]
struct Postion {
    x: i32,
    y: i32,
    angle: i32,
}

impl Postion {
    fn facing(&self) -> i32 {
        let f = self.angle % 360;
        if f < 0 {
            f + 360
        } else {
            f
        }
    }
    fn step(&mut self, ins: &Instruction) {
        match ins {
            Instruction::East(v) => self.x += v,
            Instruction::West(v) => self.x -= v,
            Instruction::North(v) => self.y += v,
            Instruction::South(v) => self.y -= v,
            Instruction::Forward(v) => match self.facing() {
                90 => self.x += v,
                270 => self.x -= v,
                0 => self.y += v,
                180 => self.y -= v,
                x => panic!("angle is not multiple of 90: {x}"),
            },
            Instruction::Left(v) => self.angle = self.facing() - v,
            Instruction::Right(v) => self.angle = self.facing() + v,
        }
    }

    fn rotate(&mut self, degrees: i32) {
        let r: f64 = degrees as f64 / 180.0 * std::f64::consts::PI;
        let cos = r.cos() as i32;
        let sin = r.sin() as i32;
        let x = cos * self.x - sin * self.y;
        let y = sin * self.x + cos * self.y;
        self.x = x;
        self.y = y;
    }
}

fn parse(content: &str) -> Vec<Instruction> {
    content
        .lines()
        .map(|line| {
            let mut chars = line.chars();
            let c = chars
                .next()
                .unwrap_or_else(|| panic!("invalid line: {line}"));
            let value = chars.collect::<String>();
            let v = value
                .parse::<i32>()
                .unwrap_or_else(|_| panic!("invalid line: {line}"));
            match c {
                'N' => Instruction::North(v),
                'E' => Instruction::East(v),
                'S' => Instruction::South(v),
                'W' => Instruction::West(v),
                'F' => Instruction::Forward(v),
                'L' => Instruction::Left(v),
                'R' => Instruction::Right(v),
                _ => panic!("invalid line: {line}"),
            }
        })
        .collect::<Vec<_>>()
}

fn part1(instructions: &[Instruction]) -> i32 {
    let mut p = Postion {
        x: 0,
        y: 0,
        angle: 90,
    };
    instructions.iter().for_each(|ins| {
        p.step(ins);
    });
    p.x.abs() + p.y.abs()
}

fn part2(instructions: &[Instruction]) -> i32 {
    let mut wp = Postion {
        x: 10,
        y: 1,
        angle: 0,
    };
    let mut ship = Postion {
        x: 0,
        y: 0,
        angle: 0,
    };

    instructions.iter().for_each(|ins| match ins {
        Instruction::East(v) => wp.x += v,
        Instruction::West(v) => wp.x -= v,
        Instruction::North(v) => wp.y += v,
        Instruction::South(v) => wp.y -= v,
        Instruction::Forward(v) => {
            ship.x += v * wp.x;
            ship.y += v * wp.y;
        }
        Instruction::Left(v) => wp.rotate(*v),
        Instruction::Right(v) => wp.rotate(-v),
    });
    ship.x.abs() + ship.y.abs()
}

pub fn main() {
    let instructions = parse(&std::fs::read_to_string("data/2020/day12").unwrap());

    // part 1
    println!("day 12 part1: {}", part1(&instructions));

    // part 2
    println!("day 12 part2: {}", part2(&instructions));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_input() {
        let instructions = parse(&std::fs::read_to_string("data/2020/day12-1").unwrap());
        assert_eq!(25, part1(&instructions));
        assert_eq!(286, part2(&instructions));
    }
}
