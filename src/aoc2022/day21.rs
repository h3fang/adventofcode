use ahash::HashMap;

enum Job<'a> {
    Num(i64),
    Op((&'a str, u8, &'a str)),
}

fn parse(data: &str) -> HashMap<&str, Job> {
    data.trim()
        .lines()
        .map(|line| {
            let (id, job) = line.split_once(": ").unwrap();
            let job = if let Ok(n) = job.parse::<i64>() {
                Job::Num(n)
            } else {
                let mut job = job.split_ascii_whitespace();
                let a = job.next().unwrap();
                let b = job.next().unwrap().as_bytes()[0];
                let c = job.next().unwrap();
                Job::Op((a, b, c))
            };
            (id, job)
        })
        .collect()
}

fn dfs<'a>(jobs: &'a HashMap<&str, Job>, m: &mut HashMap<&'a str, i64>, id: &'a str) -> i64 {
    if let Some(&r) = m.get(id) {
        return r;
    }
    let r = match *jobs.get(id).unwrap() {
        Job::Num(n) => n,
        Job::Op((a, b, c)) => {
            let a = dfs(jobs, m, a);
            let c = dfs(jobs, m, c);
            match b {
                b'+' => a + c,
                b'-' => a - c,
                b'*' => a * c,
                b'/' => a / c,
                _ => unreachable!(),
            }
        }
    };
    m.insert(id, r);
    r
}

fn part1(jobs: &HashMap<&str, Job>) -> i64 {
    let mut m = HashMap::default();
    dfs(jobs, &mut m, "root")
}

#[derive(Clone, Copy)]
enum Arg<'a> {
    Id(&'a str),
    Num(i64),
}

impl std::fmt::Display for Arg<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Arg::Id(id) => write!(f, "{id}"),
            Arg::Num(n) => write!(f, "{n}"),
        }
    }
}

#[derive(Clone, Copy)]
enum Exp<'a> {
    Num(i64),
    Op((Arg<'a>, u8, Arg<'a>)),
}

impl std::fmt::Display for Exp<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Exp::Num(n) => write!(f, "{n}"),
            Exp::Op((a, b, c)) => write!(f, "{} {} {}", a, *b as char, c),
        }
    }
}

fn find_exp<'a>(
    jobs: &HashMap<&str, Job<'a>>,
    m: &mut HashMap<&'a str, Exp<'a>>,
    id: &'a str,
) -> Exp<'a> {
    if let Some(&r) = m.get(id) {
        return r;
    }
    let r = if id == "humn" {
        Exp::Op((Arg::Id("humn"), b'?', Arg::Id("humn")))
    } else {
        match *jobs.get(id).unwrap() {
            Job::Num(n) => Exp::Num(n),
            Job::Op((a, b, c)) => {
                let j1 = find_exp(jobs, m, a);
                let j2 = find_exp(jobs, m, c);
                match (j1, j2) {
                    (Exp::Num(a), Exp::Num(c)) => match b {
                        b'+' => Exp::Num(a + c),
                        b'-' => Exp::Num(a - c),
                        b'*' => Exp::Num(a * c),
                        b'/' => Exp::Num(a / c),
                        _ => unreachable!(),
                    },
                    (Exp::Num(a), Exp::Op(_)) => Exp::Op((Arg::Num(a), b, Arg::Id(c))),
                    (Exp::Op(_), Exp::Num(c)) => Exp::Op((Arg::Id(a), b, Arg::Num(c))),
                    (Exp::Op(_), Exp::Op(_)) => unreachable!(),
                }
            }
        }
    };
    m.insert(id, r);
    r
}

fn find_value(m: &HashMap<&str, Exp>, exp: Exp, value: i64) -> i64 {
    let Exp::Op((a, b, c)) = exp else {
        unreachable!();
    };
    match (a, c) {
        (Arg::Id("humn"), Arg::Id("humn")) => value,
        (Arg::Id(id), Arg::Num(n)) => {
            let v = match b {
                b'+' => value - n,
                b'-' => value + n,
                b'*' => value / n,
                b'/' => value * n,
                _ => unreachable!(),
            };
            find_value(m, *m.get(id).unwrap(), v)
        }
        (Arg::Num(n), Arg::Id(id)) => {
            let v = match b {
                b'+' => value - n,
                b'-' => n - value,
                b'*' => value / n,
                b'/' => value * n,
                _ => unreachable!(),
            };
            find_value(m, *m.get(id).unwrap(), v)
        }
        _ => unreachable!(),
    }
}

/// Only works for input that every monkey is dependent to human at most once,
/// otherwise we have to solve the algebraic equation numerically.
fn part2(jobs: &HashMap<&str, Job>) -> i64 {
    let mut m = HashMap::default();
    let Job::Op((a, _b, c)) = *jobs.get("root").unwrap() else {
        unreachable!();
    };
    let a = find_exp(jobs, &mut m, a);
    let c = find_exp(jobs, &mut m, c);

    match (a, c) {
        (Exp::Num(v), Exp::Op(b)) => find_value(&m, Exp::Op(b), v),
        (Exp::Op(b), Exp::Num(v)) => find_value(&m, Exp::Op(b), v),
        _ => unreachable!(),
    }
}

pub fn main() {
    let data = std::fs::read_to_string("data/2022/day21").unwrap();
    let jobs = parse(&data);
    println!("part1: {}", part1(&jobs));
    println!("part2: {}", part2(&jobs));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "
root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";
        let jobs = parse(data);
        assert_eq!(152, part1(&jobs));
        assert_eq!(301, part2(&jobs));
    }
}
