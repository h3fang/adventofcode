fn phase(curr: &[u8], next: &mut [u8], prefix: &mut [i32]) {
    let n = curr.len();
    for i in 1..=n {
        prefix[i] = prefix[i - 1] + curr[i - 1] as i32;
    }
    (1..=n).for_each(|i| {
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

fn fft(signal: Vec<u8>, phases: usize) -> Vec<u8> {
    let mut curr = signal.to_vec();
    let mut next = vec![0; curr.len()];
    let mut prefix = vec![0i32; curr.len() + 1];
    for _ in 0..phases {
        phase(&curr, &mut next, &mut prefix);
        std::mem::swap(&mut curr, &mut next);
    }
    curr
}

fn part1(signal: &[u8]) -> usize {
    let s = fft(signal.to_vec(), 100);
    s.into_iter()
        .take(8)
        .fold(0, |acc, n| acc * 10 + n as usize)
}

fn part2(signal: &[u8]) -> usize {
    let offset = signal[..7]
        .iter()
        .fold(0usize, |acc, i| acc * 10 + *i as usize);
    let n = 10000 * signal.len() - offset;
    let s = signal
        .iter()
        .cycle()
        .skip(offset)
        .take(n)
        .cloned()
        .collect::<Vec<_>>();

    // The diagnals of the Pascal Triangle mod 10 has a period of 16000.
    const PERIOD: usize = 16000;
    let mut diag = vec![1u8; PERIOD];
    for _ in 1..100 {
        for i in 1..PERIOD {
            diag[i] = (diag[i] + diag[i - 1]) % 10;
        }
    }

    (0..8)
        .map(|i| {
            s[i..]
                .iter()
                .zip(diag.iter().cycle())
                .map(|(a, b)| ((a * b) % 10) as usize)
                .sum::<usize>()
                % 10
        })
        .fold(0, |acc, n| acc * 10 + n)
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
        assert_eq!(24176176, part1(&signal));
    }

    #[test]
    fn case2() {
        let signal = "19617804207202209144916044189917"
            .as_bytes()
            .iter()
            .map(|b| b - b'0')
            .collect::<Vec<_>>();
        assert_eq!(73745418, part1(&signal));
    }

    #[test]
    fn case3() {
        let signal = "69317163492948606335995924319873"
            .as_bytes()
            .iter()
            .map(|b| b - b'0')
            .collect::<Vec<_>>();
        assert_eq!(52432133, part1(&signal));
    }

    #[test]
    fn case4() {
        let signal = "12345678"
            .as_bytes()
            .iter()
            .map(|b| b - b'0')
            .collect::<Vec<_>>();
        let s = fft(signal.to_vec(), 1);
        let r = s
            .into_iter()
            .take(8)
            .fold(0, |acc, n| acc * 10 + n as usize);
        assert_eq!(48226158, r);
    }

    #[test]
    fn case5() {
        let signal = "03036732577212944063491565474664"
            .as_bytes()
            .iter()
            .map(|b| b - b'0')
            .collect::<Vec<_>>();
        assert_eq!(84462026, part2(&signal));
    }
}
