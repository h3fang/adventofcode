use std::{cmp::Reverse, collections::BinaryHeap};

fn parse(data: &str) -> Vec<(u8, u8)> {
    data.lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let s = line.as_bytes();
            let a = s[5] - b'A';
            let b = s[36] - b'A';
            (a, b)
        })
        .collect()
}

fn part1(reqs: &[(u8, u8)]) -> String {
    let mut g = vec![vec![]; 26];
    let mut in_degs = vec![0; 26];
    for r in reqs {
        g[r.0 as usize].push(r.1);
        in_degs[r.1 as usize] += 1;
    }
    let mut q = BinaryHeap::new();
    for (i, &d) in in_degs.iter().enumerate() {
        if d == 0 && !g[i].is_empty() {
            q.push(Reverse(i));
        }
    }
    let mut result = String::with_capacity(26);
    while let Some(Reverse(i)) = q.pop() {
        result.push((i as u8 + b'A') as char);
        for &next in &g[i] {
            in_degs[next as usize] -= 1;
            if in_degs[next as usize] == 0 {
                q.push(Reverse(next as usize));
            }
        }
    }
    result
}

fn part2(reqs: &[(u8, u8)], n: usize, step: usize) -> usize {
    let mut graph = vec![vec![]; 26];
    let mut in_degs = vec![0; 26];
    for r in reqs {
        graph[r.0 as usize].push(r.1);
        in_degs[r.1 as usize] += 1;
    }
    let mut q = BinaryHeap::new();
    let mut tasks = BinaryHeap::new();
    let mut workers = BinaryHeap::new();
    workers.push(Reverse(0));
    for (i, &d) in in_degs.iter().enumerate() {
        if d == 0 && !graph[i].is_empty() {
            tasks.push((Reverse(0), i));
        }
    }
    let mut result = 0;
    while let Some(Reverse(t)) = workers.pop() {
        while !tasks.is_empty() && tasks.peek().unwrap().0 .0 <= t {
            let (_, i) = tasks.pop().unwrap();
            q.push(Reverse(i));
        }
        while workers.len() < n {
            if let Some(Reverse(i)) = q.pop() {
                let t1 = t + i + 1 + step;
                workers.push(Reverse(t1));
                result = result.max(t1);
                for &next in &graph[i] {
                    in_degs[next as usize] -= 1;
                    if in_degs[next as usize] == 0 {
                        tasks.push((Reverse(t1), next as usize));
                    }
                }
            } else {
                break;
            }
        }
    }
    result
}

pub fn main() {
    let data = std::fs::read_to_string("data/2018/day7").unwrap();
    let reqs = parse(&data);
    println!("part1: {}", part1(&reqs));
    println!("part2: {}", part2(&reqs, 5, 60));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "Step C must be finished before step A can begin.\nStep C must be finished before step F can begin.\nStep A must be finished before step B can begin.\nStep A must be finished before step D can begin.\nStep B must be finished before step E can begin.\nStep D must be finished before step E can begin.\nStep F must be finished before step E can begin.\n".to_string();
        let reqs = parse(&data);
        assert_eq!("CABDFE", part1(&reqs));
        assert_eq!(15, part2(&reqs, 2, 0));
    }
}
