fn parse_stacks(data: &str) -> Vec<Vec<u8>> {
    let lines = data.lines().collect::<Vec<_>>();
    let m = lines.len() - 1;
    let n = lines.last().unwrap().split_ascii_whitespace().count();
    (0..n)
        .map(|j| {
            (0..m)
                .rev()
                .map_while(|i| {
                    let b = lines[i].as_bytes()[4 * j + 1];
                    if b == b' ' {
                        None
                    } else {
                        Some(b)
                    }
                })
                .collect()
        })
        .collect()
}

fn parse_procedure(data: &str) -> Vec<[u8; 3]> {
    data.trim()
        .lines()
        .map(|line| {
            let parts = line.split_ascii_whitespace().collect::<Vec<_>>();
            [
                parts[1].parse().unwrap(),
                parts[3].parse().unwrap(),
                parts[5].parse().unwrap(),
            ]
        })
        .collect()
}

fn parse(data: &str) -> (Vec<Vec<u8>>, Vec<[u8; 3]>) {
    let (stacks, procedure) = data.split_once("\n\n").unwrap();
    (parse_stacks(stacks), parse_procedure(procedure))
}

fn part1(mut stacks: Vec<Vec<u8>>, procedure: &[[u8; 3]]) -> String {
    procedure.iter().for_each(|p| {
        for _ in 0..p[0] {
            let c = stacks[p[1] as usize - 1].pop().unwrap();
            stacks[p[2] as usize - 1].push(c);
        }
    });
    stacks.iter().map(|s| *s.last().unwrap() as char).collect()
}

fn part2(mut stacks: Vec<Vec<u8>>, procedure: &[[u8; 3]]) -> String {
    procedure.iter().for_each(|p| {
        let n = stacks[p[1] as usize - 1].len() - p[0] as usize;
        let s2: *mut Vec<u8> = &mut stacks[p[2] as usize - 1];
        let s1 = &mut stacks[p[1] as usize - 1];
        unsafe { (*s2).extend(&s1[n..]) };
        s1.resize(n, b' ');
    });
    stacks.iter().map(|s| *s.last().unwrap() as char).collect()
}

pub fn main() {
    let data = std::fs::read_to_string("data/2022/day5").unwrap();
    let (stacks, procedure) = parse(&data);
    println!("part1: {}", part1(stacks.clone(), &procedure));
    println!("part2: {}", part2(stacks, &procedure));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        let (stacks, procedure) = parse(&data);
        assert_eq!("CMZ", part1(stacks.clone(), &procedure));
        assert_eq!("MCD", part2(stacks, &procedure));
    }
}
