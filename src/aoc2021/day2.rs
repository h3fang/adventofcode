use std::{fmt::Display, str::FromStr};

#[derive(Debug)]
enum Command {
    Up(i64),
    Down(i64),
    Forward(i64),
}

struct InvalidCommand(String);

impl Display for InvalidCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid command, source: {}", self.0)
    }
}

impl FromStr for Command {
    type Err = InvalidCommand;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.trim().split(' ');
        let dir = parts.next().ok_or_else(|| InvalidCommand(s.to_string()))?;
        let n = parts
            .next()
            .ok_or_else(|| InvalidCommand(s.to_string()))?
            .parse::<i64>()
            .map_err(|_| InvalidCommand(s.to_string()))?;
        match dir {
            "up" => Ok(Command::Up(n)),
            "down" => Ok(Command::Down(n)),
            "forward" => Ok(Command::Forward(n)),
            _ => Err(InvalidCommand(s.to_string())),
        }
    }
}

fn part1(cmds: &[Command]) -> i64 {
    let mut horizontal = 0;
    let mut depth = 0;
    for cmd in cmds {
        match cmd {
            Command::Up(n) => depth -= n,
            Command::Down(n) => depth += n,
            Command::Forward(n) => horizontal += n,
        }
    }
    horizontal * depth
}

fn part2(cmds: &[Command]) -> i64 {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    for cmd in cmds {
        match cmd {
            Command::Up(n) => aim -= *n,
            Command::Down(n) => aim += *n,
            Command::Forward(n) => {
                horizontal += n;
                depth += aim * n;
            }
        }
    }
    horizontal * depth
}

pub fn main() {
    let cmds: Result<Vec<Command>, _> = std::fs::read_to_string("data/2021/day2")
        .unwrap()
        .lines()
        .map(|s| s.parse())
        .collect();

    match cmds {
        Ok(cmds) => {
            println!("day2 part1: {}", part1(&cmds));
            println!("day2 part2: {}", part2(&cmds));
        }
        Err(e) => println!("failed to parse input, error: {e}"),
    }
}
