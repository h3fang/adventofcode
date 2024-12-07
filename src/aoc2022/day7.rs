use std::{cell::RefCell, rc::Rc};

use ahash::HashMap;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{self, line_ending},
    combinator::all_consuming,
    multi::separated_list0,
    sequence::{preceded, separated_pair, tuple},
    IResult,
};

enum Entry<'a> {
    Dir(&'a str),
    File(&'a str, usize),
}

enum Cmd<'a> {
    Cd(&'a str),
    Ls(Vec<Entry<'a>>),
}

enum FSEntry<'a> {
    Dir(HashMap<&'a str, Rc<RefCell<FSEntry<'a>>>>),
    File(usize),
}

fn parse_file(input: &str) -> IResult<&str, Entry> {
    let (r, (size, name)) = separated_pair(
        complete::u64,
        complete::char(' '),
        take_while(|c: char| c != '\r' && c != '\n'),
    )(input)?;
    Ok((r, Entry::File(name, size as usize)))
}

fn parse_dir(input: &str) -> IResult<&str, Entry> {
    let (r, name) = preceded(tag("dir "), take_while(|c: char| c != '\r' && c != '\n'))(input)?;
    Ok((r, Entry::Dir(name)))
}

fn parse_entry(input: &str) -> IResult<&str, Entry> {
    alt((parse_file, parse_dir))(input)
}

fn parse_ls_output(input: &str) -> IResult<&str, Vec<Entry>> {
    separated_list0(line_ending, parse_entry)(input)
}

fn parse_ls(input: &str) -> IResult<&str, Cmd> {
    let (r, entries) = preceded(tuple((tag("ls"), line_ending)), parse_ls_output)(input)?;
    Ok((r, Cmd::Ls(entries)))
}

fn parse_cd(input: &str) -> IResult<&str, Cmd> {
    let (r, arg) = preceded(tag("cd "), take_while(|c: char| c != '\r' && c != '\n'))(input)?;
    Ok((r, Cmd::Cd(arg)))
}

fn parse_cmd(input: &str) -> IResult<&str, Cmd> {
    preceded(tag("$ "), alt((parse_cd, parse_ls)))(input)
}

fn parse(data: &str) -> Rc<RefCell<FSEntry>> {
    let (_, cmds) = all_consuming(separated_list0(line_ending, parse_cmd))(data.trim()).unwrap();

    let root = Rc::new(RefCell::new(FSEntry::Dir(HashMap::default())));
    let mut stack = vec![root.clone()];
    for cmd in cmds {
        match cmd {
            Cmd::Cd(arg) => match arg {
                ".." => {
                    stack.pop();
                }
                "/" => {
                    stack.drain(1..);
                }
                x => {
                    let sub = {
                        let sub = stack.last().unwrap();
                        let mut sub = sub.borrow_mut();
                        match &mut *sub {
                            FSEntry::Dir(d) => d
                                .entry(x)
                                .or_insert_with(|| {
                                    Rc::new(RefCell::new(FSEntry::Dir(HashMap::default())))
                                })
                                .clone(),
                            _ => unreachable!(),
                        }
                    };
                    stack.push(sub);
                }
            },
            Cmd::Ls(output) => {
                let curr = stack.last().unwrap();
                let mut curr = curr.borrow_mut();
                match &mut *curr {
                    FSEntry::Dir(d) => {
                        for e in output {
                            match e {
                                Entry::Dir(name) => {
                                    d.insert(
                                        name,
                                        Rc::new(RefCell::new(FSEntry::Dir(HashMap::default()))),
                                    );
                                }
                                Entry::File(name, s) => {
                                    d.insert(name, Rc::new(RefCell::new(FSEntry::File(s))));
                                }
                            }
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
    root
}

fn part1(root: &Rc<RefCell<FSEntry>>) -> (usize, usize) {
    let mut sum = 0;
    fn dfs(entry: &Rc<RefCell<FSEntry>>, sum: &mut usize) -> usize {
        let entry = entry.borrow();
        match &*entry {
            FSEntry::Dir(dir) => {
                let mut result = 0;
                for e in dir.values() {
                    result += dfs(e, sum);
                }

                if result <= 10_0000 {
                    *sum += result;
                }
                result
            }
            FSEntry::File(size) => *size,
        }
    }
    let total = dfs(root, &mut sum);
    (sum, total)
}

fn part2(root: &Rc<RefCell<FSEntry>>, total: usize) -> usize {
    let threshold = 3000_0000 - (7000_0000 - total);
    fn dfs(entry: &Rc<RefCell<FSEntry>>, threshold: usize, min: &mut usize) -> usize {
        let entry = entry.borrow();
        match &*entry {
            FSEntry::Dir(dir) => {
                let mut result = 0;
                for e in dir.values() {
                    result += dfs(e, threshold, min);
                }
                if result >= threshold && result < *min {
                    *min = result;
                }
                result
            }
            FSEntry::File(size) => *size,
        }
    }
    let mut result = usize::MAX;
    dfs(root, threshold, &mut result);
    result
}

pub fn main() {
    let data = std::fs::read_to_string("data/2022/day7").unwrap();
    let root = parse(&data);
    let (p1, total) = part1(&root);
    println!("part1: {}", p1);
    println!("part2: {}", part2(&root, total));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
        let root = parse(data);
        let (p1, total) = part1(&root);
        assert_eq!(95437, p1);
        assert_eq!(24933642, part2(&root, total));
    }
}
