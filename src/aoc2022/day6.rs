fn parse(data: &str) -> &[u8] {
    data.trim().as_bytes()
}

fn marker(signal: &[u8], len: usize) -> usize {
    let len = len as i32;
    let mut prev = [-1; 26];
    let mut curr = 0;
    for (i, b) in signal.iter().enumerate() {
        let k = (b - b'a') as usize;
        let j = prev[k];
        if j >= 0 && (i as i32 - j) <= curr {
            curr = i as i32 - j;
        } else {
            curr += 1;
        }
        if curr == len {
            return i + 1;
        }
        prev[k] = i as i32;
    }
    usize::MAX
}

pub fn main() {
    let data = std::fs::read_to_string("data/2022/day6").unwrap();
    let signal = parse(&data);
    println!("part1: {}", marker(signal, 4));
    println!("part2: {}", marker(signal, 14));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let signal = parse(data);
        assert_eq!(7, marker(signal, 4));
        assert_eq!(19, marker(signal, 14));
    }

    #[test]
    fn case2() {
        let data = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let signal = parse(data);
        assert_eq!(5, marker(signal, 4));
        assert_eq!(23, marker(signal, 14));
    }

    #[test]
    fn case3() {
        let data = "nppdvjthqldpwncqszvftbrmjlhg";
        let signal = parse(data);
        assert_eq!(6, marker(signal, 4));
        assert_eq!(23, marker(signal, 14));
    }

    #[test]
    fn case4() {
        let data = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let signal = parse(data);
        assert_eq!(10, marker(signal, 4));
        assert_eq!(29, marker(signal, 14));
    }

    #[test]
    fn case5() {
        let data = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let signal = parse(data);
        assert_eq!(11, marker(signal, 4));
        assert_eq!(26, marker(signal, 14));
    }
}
