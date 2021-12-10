fn solve(lines: &[&str]) -> (usize, usize) {
    let mut p1 = 0;
    let mut p2 = vec![];
    for line in lines {
        let line = line.as_bytes();
        let mut s = vec![];
        let mut corrupted = false;
        for &b in line {
            match b {
                b')' => {
                    let s = s.pop().unwrap_or(b'X');
                    if s != b'(' {
                        p1 += 3;
                        corrupted = true;
                        break;
                    }
                }
                b']' => {
                    let s = s.pop().unwrap_or(b'X');
                    if s != b'[' {
                        p1 += 57;
                        corrupted = true;
                        break;
                    }
                }
                b'}' => {
                    let s = s.pop().unwrap_or(b'X');
                    if s != b'{' {
                        p1 += 1197;
                        corrupted = true;
                        break;
                    }
                }
                b'>' => {
                    let s = s.pop().unwrap_or(b'X');
                    if s != b'<' {
                        p1 += 25137;
                        corrupted = true;
                        break;
                    }
                }
                b => s.push(b),
            }
        }
        if corrupted {
            continue;
        }
        let mut total = 0;
        while let Some(b) = s.pop() {
            match b {
                b'(' => total = total * 5 + 1,
                b'[' => total = total * 5 + 2,
                b'{' => total = total * 5 + 3,
                b'<' => total = total * 5 + 4,
                _ => {
                    s.pop();
                }
            }
        }
        if total > 0 {
            p2.push(total);
        }
    }
    p2.sort_unstable();
    (p1, p2[p2.len() / 2])
}

pub fn main() {
    let data = std::fs::read_to_string("data/2021/day10").unwrap();
    let lines = data.lines().collect::<Vec<_>>();
    let (p1, p2) = solve(&lines);
    println!("day10 part1: {}", p1);
    println!("day10 part2: {}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "[({(<(())[]>[[{[]{<()<>>
            [(()[<>])]({[<{<<[]>>(
            {([(<{}[<>[]}>{[]{[(<()>
            (((({<>}<{<{<>}{[]{[]{}
            [[<[([]))<([[{}[[()]]]
            [{[{({}]{}}([{[{{{}}([]
            {<[[]]>}<{[{[{[]{()[[[]
            [<(<(<(<{}))><([]([]()
            <{([([[(<>()){}]>(<<{{
            <{([{{}}[<[[[<>{}]]]>[]]";
        let lines = data.lines().map(|s| s.trim()).collect::<Vec<_>>();
        assert_eq!((26397, 288957), solve(&lines));
    }
}
