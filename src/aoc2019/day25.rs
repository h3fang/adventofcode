use crate::day5::Intcode;
use ahash::AHashMap as HashMap;
use ahash::AHashSet as HashSet;
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    East,
    West,
    South,
    North,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::East => Direction::West,
            Direction::West => Direction::East,
            Direction::South => Direction::North,
            Direction::North => Direction::South,
        }
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::East => "east",
                Direction::West => "west",
                Direction::South => "south",
                Direction::North => "north",
            }
        )
    }
}

fn parse_output(output: &str) -> (String, Vec<Direction>, Vec<String>) {
    let mut name = String::new();
    let mut dirs = vec![];
    let mut items = vec![];
    for parts in output.split("\n\n") {
        if parts.contains("== ") {
            name = parts
                .lines()
                .filter_map(|line| line.strip_prefix("== "))
                .next()
                .unwrap()
                .strip_suffix(" ==")
                .unwrap()
                .to_string();
        } else if parts.starts_with("Doors here lead:") {
            for line in parts.lines() {
                if let Some(d) = line.strip_prefix("- ") {
                    dirs.push(match d {
                        "east" => Direction::East,
                        "west" => Direction::West,
                        "north" => Direction::North,
                        "south" => Direction::South,
                        _ => unreachable!(),
                    });
                }
            }
        } else if parts.starts_with("Items here:") {
            for line in parts.lines() {
                if let Some(d) = line.strip_prefix("- ") {
                    items.push(d.to_string());
                }
            }
        }
    }
    (name, dirs, items)
}

fn command(prog: &mut Intcode, cmd: &str) {
    prog.inputs
        .extend(cmd.as_bytes().iter().map(|b| *b as i64).chain([10]));
}

fn run_command(prog: &mut Intcode, cmd: &str) {
    command(prog, cmd);
    prog.run_till_input();
    prog.outputs.clear();
}

fn command_output(prog: &mut Intcode, cmd: &str) -> String {
    command(prog, cmd);
    prog.run_till_input();
    ascii_output(prog)
}

fn ascii_output(prog: &mut Intcode) -> String {
    prog.outputs.drain(..).map(|b| (b as u8) as char).collect()
}

fn dfs_traversal(
    visited: &mut HashSet<String>,
    collected: &mut HashSet<String>,
    bad_items: &HashSet<String>,
    prog: &mut Intcode,
    from: Option<Direction>,
) -> String {
    prog.run_till_input();
    let s = ascii_output(prog);
    let (name, dirs, items) = parse_output(&s);

    if visited.contains(&name) {
        return s;
    }

    // take all good items
    for item in items {
        if bad_items.contains(&item) {
            continue;
        }
        run_command(prog, &format!("take {}", item));
        collected.insert(item);
    }

    visited.insert(name);

    for d in &dirs {
        // explore
        if let Some(from) = from {
            if d == &from {
                continue;
            }
        }
        command(prog, &d.to_string());
        dfs_traversal(visited, collected, bad_items, prog, Some(d.opposite()));

        // go back one step
        run_command(prog, &d.opposite().to_string());
    }
    s
}

fn go_to_checkpoint(
    output: String,
    visited: &mut HashSet<String>,
    prog: &mut Intcode,
    from: Option<Direction>,
) -> Option<Direction> {
    let (name, dirs, _) = parse_output(&output);

    if name == "Security Checkpoint" {
        if let Some(from) = from {
            for &d in &dirs {
                if d != from {
                    return Some(d);
                }
            }
        }
        unreachable!()
    }

    if visited.contains(&name) {
        return None;
    }
    visited.insert(name);

    for d in &dirs {
        // explore
        if let Some(from) = from {
            if *d == from {
                continue;
            }
        }
        let output = command_output(prog, &d.to_string());
        if let Some(d) = go_to_checkpoint(output, visited, prog, Some(*d)) {
            return Some(d);
        }

        // go back one step
        run_command(prog, &d.opposite().to_string());
    }

    None
}

fn check_output(output: &str) -> Ordering {
    if output.contains("lighter") {
        Ordering::Greater
    } else if output.contains("heavier") {
        Ordering::Less
    } else {
        Ordering::Equal
    }
}

fn guess_items(
    prog: &mut Intcode,
    collected: &mut HashSet<String>,
    direction: Direction,
) -> String {
    fn search(
        cache: &mut HashMap<u8, Ordering>,
        prog: &mut Intcode,
        items: &[String],
        taken: u8,
        curr: u8,
        direction: &str,
    ) -> Option<String> {
        let order = if cache
            .iter()
            .any(|(&k, &v)| k & taken == taken && v == Ordering::Less)
        {
            Ordering::Less
        } else if cache
            .iter()
            .any(|(&k, &v)| k & taken == k && v == Ordering::Greater)
        {
            Ordering::Greater
        } else {
            let output = command_output(prog, direction);
            let order = check_output(&output);
            if order == Ordering::Equal {
                return Some(output);
            }
            cache.insert(taken, order);
            order
        };

        if curr.count_ones() == items.len() as u32 {
            return None;
        }

        match order {
            Ordering::Less => {
                for i in 0..items.len() {
                    let bit = 1u8 << i;
                    if curr & bit == 0 && taken & bit == 0 {
                        run_command(prog, &format!("take {}", items[i]));
                        let taken = taken | bit;
                        if let Some(s) = search(cache, prog, items, taken, curr | bit, direction) {
                            return Some(s);
                        }
                        run_command(prog, &format!("drop {}", items[i]));
                    }
                }
                None
            }
            Ordering::Equal => unreachable!(),
            Ordering::Greater => {
                for i in 0..items.len() {
                    let bit = 1u8 << i;
                    if curr & bit == 0 && taken & bit > 0 {
                        run_command(prog, &format!("drop {}", items[i]));
                        let taken = taken & !bit;
                        if let Some(s) = search(cache, prog, items, taken, curr | bit, direction) {
                            return Some(s);
                        }
                        run_command(prog, &format!("take {}", items[i]));
                    }
                }
                None
            }
        }
    }

    let n = collected.len();
    assert!(n <= 8);

    let mut cache = HashMap::new();
    let items = collected.iter().cloned().collect::<Vec<_>>();
    let taken = ((1u16 << n) - (1u16 << (n / 2))) as u8;
    for item in items.iter().take(n / 2) {
        run_command(prog, &format!("drop {}", item));
    }

    let output = search(&mut cache, prog, &items, taken, 0, &direction.to_string()).unwrap();

    let pattern = "You should be able to get in by typing ";
    let i = output.find(pattern).unwrap() + pattern.len();
    output[i..].split_once(' ').unwrap().0.to_string()
}

pub fn main() {
    let codes = std::fs::read_to_string("data/2019/day25")
        .unwrap()
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|t| t.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut prog = Intcode::new(&codes);

    let bad_items = [
        "photons",
        "infinite loop",
        "escape pod",
        "molten lava",
        "giant electromagnet",
    ]
    .iter()
    .map(|item| item.to_string())
    .collect();

    // traverse the map
    let mut visited = HashSet::new();
    let mut collected = HashSet::new();
    let output = dfs_traversal(&mut visited, &mut collected, &bad_items, &mut prog, None);

    // go to checkpoint
    visited.clear();
    let direction = go_to_checkpoint(output, &mut visited, &mut prog, None);

    // guess the correct items combination
    let p1 = guess_items(&mut prog, &mut collected, direction.unwrap());
    println!("day 25 part1: {}", p1);
}
