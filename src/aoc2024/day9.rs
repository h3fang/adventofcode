use std::{
    cmp::Reverse,
    collections::{BinaryHeap, VecDeque},
};

fn parse(input: &str) -> &[u8] {
    input.trim().as_bytes()
}

fn part1(mut map: &[u8]) -> usize {
    if map.len() % 2 == 0 {
        map = &map[..map.len() - 1];
    }

    let mut i = 0;
    let mut s = VecDeque::with_capacity(map.len() / 2);
    let mut pos = Vec::with_capacity(map.len());
    for c in map.chunks(2) {
        let a = (c[0] - b'0') as usize;
        pos.push(i);
        i += a;

        if c.len() == 2 {
            pos.push(i);
            let b = (c[1] - b'0') as usize;
            s.push_back((i, b));
            i += b;
        }
    }
    let mut result = 0;
    for i in (0..map.len()).rev().step_by(2) {
        let mut remaining = (map[i] - b'0') as usize;
        let id = i / 2;
        while remaining > 0 {
            if let Some((j, len)) = s.pop_front() {
                if j > pos[i] {
                    s.clear();
                    continue;
                }
                let moved = len.min(remaining);
                result += id * (j + j + moved - 1) * moved / 2;
                if len > moved {
                    s.push_front((j + moved, len - moved));
                }
                remaining -= moved;
            } else {
                result += id * (pos[i] + pos[i] + remaining - 1) * remaining / 2;
                break;
            }
        }
    }
    result
}

fn part2(mut map: &[u8]) -> usize {
    if map.len() % 2 == 0 {
        map = &map[..map.len() - 1];
    }

    let mut empty = vec![BinaryHeap::default(); 10];
    let mut pos = Vec::with_capacity(map.len());
    let mut p = 0;
    for c in map.chunks(2) {
        let a = (c[0] - b'0') as u32;
        pos.push(p);
        p += a;

        if c.len() == 2 {
            pos.push(p);
            let b = (c[1] - b'0') as u32;
            empty[b as usize].push(Reverse(p));
            p += b;
        }
    }
    let mut result = 0;
    for i in (0..map.len()).rev().step_by(2) {
        let size = (map[i] - b'0') as u32;
        let min = (size..10)
            .filter_map(|len| {
                if let Some(&Reverse(p)) = empty[len as usize].peek() {
                    if p < pos[i] {
                        return Some((p, len));
                    }
                }
                None
            })
            .min();
        let p = if let Some((j, len)) = min {
            if len > size {
                empty[(len - size) as usize].push(Reverse(j + size));
            }
            empty[len as usize].pop();
            j
        } else {
            pos[i]
        };
        result += i / 2 * (p + p + size - 1) as usize * size as usize / 2;
    }
    result
}

pub fn main() {
    let input: String = std::fs::read_to_string("data/2024/day9").unwrap();
    let map = parse(&input);
    println!("part1: {}", part1(map));
    println!("part2: {}", part2(map));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let input = "2333133121414131402";
        let map = parse(input);
        assert_eq!(1928, part1(map));
        assert_eq!(2858, part2(map));
    }
}
