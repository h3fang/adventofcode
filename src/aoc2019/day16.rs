fn phase(curr: &[u8], next: &mut [u8], offset: usize) {
    let n = curr.len();
    let mut prefix = vec![0i32; n + 1];
    for i in 1..=n {
        prefix[i] = prefix[i - 1] + curr[i - 1] as i32;
    }
    (offset + 1..=n).for_each(|i| {
        let mut sum = 0i32;
        let mut j = i - 1;
        let mut k = 1;
        while j < n {
            match k % 4 {
                1 => sum += prefix[(j + i).min(n)] - prefix[j],
                3 => sum -= prefix[(j + i).min(n)] - prefix[j],
                _ => {}
            }
            j += i;
            k += 1;
        }
        next[i - 1] = (sum.abs() % 10) as u8;
    });
}

fn fft(signal: Vec<u8>, phases: usize, offset: usize) -> Vec<u8> {
    let mut curr = signal.to_vec();
    let mut next = vec![0; curr.len()];
    for _ in 0..phases {
        phase(&curr, &mut next, offset);
        std::mem::swap(&mut curr, &mut next);
    }
    curr
}

fn part1(signal: &[u8]) -> String {
    let s = fft(signal.to_vec(), 100, 0);
    let s = s.iter().take(8).map(|b| b + b'0').collect::<Vec<_>>();
    unsafe { String::from_utf8_unchecked(s) }
}

fn part2(signal: &[u8]) -> String {
    let offset = signal[..7]
        .iter()
        .fold(0usize, |acc, i| acc * 10 + *i as usize);
    let mut s = signal.repeat(10000);

    let n = s.len();
    let mut p = vec![0; n];
    for _ in 0..100 {
        // Since the offset is larger than half of the repeated signal, the output
        // after each phase is just a suffix sum.
        p[n - 1] = s[n - 1];
        for i in (offset..n - 1).rev() {
            p[i] = (s[i] + p[i + 1]) % 10;
        }
        std::mem::swap(&mut s, &mut p);
    }

    let s = s
        .iter()
        .skip(offset)
        .take(8)
        .map(|b| b + b'0')
        .collect::<Vec<_>>();
    unsafe { String::from_utf8_unchecked(s) }
}

pub fn main() {
    let data = std::fs::read_to_string("data/2019/day16").unwrap();
    let signal = data
        .lines()
        .next()
        .unwrap()
        .as_bytes()
        .iter()
        .map(|b| b - b'0')
        .collect::<Vec<_>>();

    println!("day16 part1: {}", part1(&signal));
    println!("day16 part2: {}", part2(&signal));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let signal = "80871224585914546619083218645595"
            .as_bytes()
            .iter()
            .map(|b| b - b'0')
            .collect::<Vec<_>>();
        assert_eq!("24176176".to_string(), part1(&signal));
    }

    #[test]
    fn case2() {
        let signal = "19617804207202209144916044189917"
            .as_bytes()
            .iter()
            .map(|b| b - b'0')
            .collect::<Vec<_>>();
        assert_eq!("73745418".to_string(), part1(&signal));
    }

    #[test]
    fn case3() {
        let signal = "69317163492948606335995924319873"
            .as_bytes()
            .iter()
            .map(|b| b - b'0')
            .collect::<Vec<_>>();
        assert_eq!("52432133".to_string(), part1(&signal));
    }

    #[test]
    fn case4() {
        let signal = "12345678"
            .as_bytes()
            .iter()
            .map(|b| b - b'0')
            .collect::<Vec<_>>();
        let s = fft(signal.to_vec(), 1, 0);
        let s = s.iter().take(8).map(|b| b + b'0').collect::<Vec<_>>();
        let r = unsafe { String::from_utf8_unchecked(s) };
        assert_eq!("48226158".to_string(), r);
    }

    #[test]
    fn case5() {
        let signal = "03036732577212944063491565474664"
            .as_bytes()
            .iter()
            .map(|b| b - b'0')
            .collect::<Vec<_>>();
        assert_eq!("84462026".to_string(), part2(&signal));
    }
}
