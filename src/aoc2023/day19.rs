use ahash::HashMap;
use arrayvec::ArrayVec;

enum Rule<'a> {
    Accept,
    Reject,
    Test(u8, u8, u32, &'a str),
    Workflow(&'a str),
}

fn parse_rule(s: &str) -> Rule {
    match s {
        "A" => Rule::Accept,
        "R" => Rule::Reject,
        x if x.contains(':') => {
            let t = s.as_bytes();
            let i = t.iter().position(|&c| c == b':').unwrap();
            let p = part_number(t[0]);
            let n = s[2..i].parse().unwrap();
            Rule::Test(p, t[1], n, &s[i + 1..])
        }
        x => Rule::Workflow(x),
    }
}

fn part_number(part: u8) -> u8 {
    match part {
        b'x' => 0,
        b'm' => 1,
        b'a' => 2,
        b's' => 3,
        _ => unreachable!(),
    }
}

fn parse(data: &str) -> (HashMap<&str, Vec<Rule>>, Vec<[u32; 4]>) {
    let lines = data.trim().lines().collect::<Vec<_>>();
    let i = lines.iter().position(|l| l.is_empty()).unwrap();
    let workflows = lines[..i]
        .iter()
        .map(|l| {
            let (k, v) = l.split_once('{').unwrap();
            let rules = v.trim_end_matches('}').split(',').map(parse_rule).collect();
            (k, rules)
        })
        .collect();
    let ratings = lines[i + 1..]
        .iter()
        .map(|l| {
            let l = l.trim_matches(|c| c == '{' || c == '}');
            let mut r = [0; 4];
            for p in l.split(',') {
                let (part, rating) = p.split_once('=').unwrap();
                let rating = rating.parse().unwrap();
                let i = part_number(part.as_bytes()[0]);
                r[i as usize] = rating;
            }
            r
        })
        .collect();
    (workflows, ratings)
}

fn part1(workflows: &HashMap<&str, Vec<Rule>>, ratings: &[[u32; 4]]) -> u32 {
    ratings
        .iter()
        .map(|rating| {
            let mut w = "in";
            loop {
                match w {
                    "A" => return rating.iter().sum(),
                    "R" => return 0,
                    _ => {}
                }
                for rule in &workflows[w] {
                    match rule {
                        Rule::Accept => return rating.iter().sum(),
                        Rule::Reject => return 0,
                        Rule::Test(p, c, n, next) => {
                            let test = match c {
                                b'>' => rating[*p as usize] > *n,
                                b'<' => rating[*p as usize] < *n,
                                _ => unreachable!(),
                            };
                            if test {
                                w = next;
                                break;
                            }
                        }
                        Rule::Workflow(next) => w = next,
                    }
                }
            }
        })
        .sum()
}

#[derive(Clone, Copy)]
struct Range {
    start: i16,
    end: i16,
}

impl Range {
    const fn new(start: i16, end: i16) -> Self {
        Self { start, end }
    }

    fn negate(&self) -> ArrayVec<Range, 2> {
        let mut r = ArrayVec::new();
        if self.start > 1 {
            r.push(Range::new(1, self.start));
        }
        if self.end < 4001 {
            r.push(Range::new(self.end, 4001));
        }
        r
    }

    fn intersect(&self, other: &Range) -> Option<Self> {
        let start = self.start.max(other.start);
        let end = self.end.min(other.end);
        if start < end {
            Some(Range::new(start, end))
        } else {
            None
        }
    }

    fn count(&self) -> usize {
        (self.end - self.start) as _
    }
}

const FULL: Range = Range::new(1, 4001);

fn combinations(parts: &[Vec<Range>]) -> usize {
    parts
        .iter()
        .map(|p| p.iter().map(|r| r.count()).sum::<usize>())
        .product()
}

fn dfs(workflows: &HashMap<&str, Vec<Rule>>, w: &str, mut parts: [Vec<Range>; 4]) -> usize {
    match w {
        "A" => return combinations(&parts),
        "R" => return 0,
        _ => {}
    }

    let mut result = 0;
    for rule in &workflows[w] {
        match rule {
            Rule::Accept => return result + combinations(&parts),
            Rule::Reject => return result,
            &Rule::Test(p, cmp, n, w) => {
                let range = if cmp == b'>' {
                    Range::new(n as i16 + 1, 4001)
                } else {
                    Range::new(1, n as i16)
                };
                let mut next = parts.clone();
                next[p as usize].iter_mut().for_each(|a| {
                    if let Some(r) = a.intersect(&range) {
                        *a = r;
                    }
                });
                result += dfs(workflows, w, next);

                // proceed to next rule
                for r in range.negate() {
                    parts[p as usize].iter_mut().for_each(|a| {
                        if let Some(b) = a.intersect(&r) {
                            *a = b;
                        }
                    });
                }
            }
            Rule::Workflow(w) => return result + dfs(workflows, w, parts.clone()),
        }
    }
    result
}

fn part2(workflows: &HashMap<&str, Vec<Rule>>) -> usize {
    dfs(
        workflows,
        "in",
        [vec![FULL], vec![FULL], vec![FULL], vec![FULL]],
    )
}

pub fn main() {
    let data = std::fs::read_to_string("data/2023/day19").unwrap();
    let (workflows, ratings) = parse(&data);
    println!("part1: {}", part1(&workflows, &ratings));
    println!("part2: {}", part2(&workflows));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = r"
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
        let (workflows, ratings) = parse(data);
        assert_eq!(19114, part1(&workflows, &ratings));
        assert_eq!(167409079868000, part2(&workflows));
    }
}
