enum Action {
    Reverse,
    Cut(i64),
    Deal(i64),
}

#[derive(Debug)]
struct Shuffle {
    n: i64,
    a: i64,
    b: i64,
}

fn mod_pow(base: i64, mut exponent: i64, modulus: i64) -> i64 {
    let mut result = 1i128;
    let mut base = (base % modulus) as i128;
    let modulus = modulus as i128;
    while exponent > 0 {
        if exponent & 1 > 0 {
            result = (result * base) % modulus;
        }
        exponent >>= 1;
        base = (base * base) % modulus;
    }
    result as i64
}

fn mod_inverse(modulus: i64, k: i64) -> i64 {
    mod_pow(k, modulus - 2, modulus)
}

impl Shuffle {
    fn new(n: i64) -> Self {
        Self { n, a: 1, b: 0 }
    }

    fn reverse(&mut self) {
        self.a *= -1;
        self.b = self.n - 1 - self.b;
    }

    fn cut(&mut self, k: i64) {
        self.b = (self.b - k + self.n) % self.n;
    }

    fn deal(&mut self, k: i64) {
        // k is small (less than 100 for my input),
        // a is nowhere near i64::MAX or i64::MIN,
        // so a * k doesn't overflow.
        self.a = (self.a * k) % self.n;
        self.b *= k;
    }

    fn compose(&mut self, actions: &[Action]) {
        for action in actions {
            match action {
                Action::Reverse => self.reverse(),
                Action::Cut(k) => self.cut(*k),
                Action::Deal(k) => self.deal(*k),
            }
        }
    }

    fn repeat(&mut self, mut n: usize) {
        fn square(m: i128, (a, b): (i128, i128), (c, d): (i128, i128)) -> (i128, i128) {
            ((a * c) % m, (c * b + d) % m)
        }
        let modulus = self.n as i128;
        let mut base = (self.a as i128 % modulus, self.b as i128 % modulus);
        let mut result = (1, 0);
        while n > 0 {
            if n & 1 == 1 {
                result = square(modulus, base, result);
            }
            n >>= 1;
            base = square(modulus, base, base);
        }
        self.a = result.0 as i64;
        self.b = result.1 as i64;
    }

    fn inverse(&mut self) {
        let inv = mod_inverse(self.n, self.a) as i128;
        self.b = ((self.n as i128 - inv * self.b as i128) % (self.n as i128)) as i64;
        self.a = inv as i64;
    }

    fn to(&self, x: i64) -> i64 {
        ((self.a * x + self.b) % self.n + self.n) % self.n
    }
}

fn part1(actions: &[Action], deck_size: i64) -> i64 {
    let mut s = Shuffle::new(deck_size);
    s.compose(actions);
    s.to(2019)
}

fn part2(actions: &[Action], deck_size: i64, repeat: usize) -> i64 {
    let mut s = Shuffle::new(deck_size);
    s.compose(actions);
    s.inverse();
    s.repeat(repeat);
    s.to(2020)
}

fn parse(data: &str) -> Vec<Action> {
    data.lines()
        .map(|line| {
            let line = line.trim();
            if line.starts_with("deal i") {
                Action::Reverse
            } else if line.starts_with("deal w") {
                let n = line
                    .strip_prefix("deal with increment ")
                    .unwrap()
                    .parse::<i64>()
                    .unwrap();
                Action::Deal(n)
            } else {
                let n = line.strip_prefix("cut ").unwrap().parse::<i64>().unwrap();
                Action::Cut(n)
            }
        })
        .collect()
}

pub fn main() {
    let data = std::fs::read_to_string("data/2019/day22").unwrap();
    let actions = parse(&data);
    println!("day 22 part1: {}", part1(&actions, 10007));
    println!(
        "day 22 part2: {}",
        part2(&actions, 119315717514047, 101741582076661)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let deck_size = 7;
        let mut s = Shuffle::new(deck_size);
        s.reverse();
        let mut deck = vec![0; deck_size as usize];
        (0..deck_size).for_each(|i| deck[s.to(i) as usize] = i);
        assert_eq!(vec![6, 5, 4, 3, 2, 1, 0], deck);
    }

    #[test]
    fn case2() {
        let deck_size = 7;
        let mut s = Shuffle::new(deck_size);
        s.cut(3);
        let mut deck = vec![0; deck_size as usize];
        (0..deck_size).for_each(|i| deck[s.to(i) as usize] = i);
        assert_eq!(vec![3, 4, 5, 6, 0, 1, 2], deck);
    }

    #[test]
    fn case3() {
        let deck_size = 7;
        let mut s = Shuffle::new(deck_size);
        s.cut(-3);
        let mut deck = vec![0; deck_size as usize];
        (0..deck_size).for_each(|i| deck[s.to(i) as usize] = i);
        assert_eq!(vec![4, 5, 6, 0, 1, 2, 3], deck);
    }

    #[test]
    fn case4() {
        let deck_size = 7;
        let mut s = Shuffle::new(deck_size);
        s.deal(3);
        let mut deck = vec![0; deck_size as usize];
        (0..deck_size).for_each(|i| deck[s.to(i) as usize] = i);
        assert_eq!(vec![0, 5, 3, 1, 6, 4, 2], deck);
    }

    #[test]
    fn case5() {
        let data = "deal with increment 7
        deal into new stack
        deal into new stack";
        let actions = parse(&data);
        let deck_size = 10i64;
        let mut s = Shuffle::new(deck_size);
        s.compose(&actions);
        let mut deck = vec![0; deck_size as usize];
        (0..deck_size).for_each(|i| deck[s.to(i) as usize] = i);
        assert_eq!(vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7], deck);
    }

    #[test]
    fn case6() {
        let data = "cut 6
        deal with increment 7
        deal into new stack";
        let actions = parse(&data);
        let deck_size = 10i64;
        let mut s = Shuffle::new(deck_size);
        s.compose(&actions);
        let mut deck = vec![0; deck_size as usize];
        (0..deck_size).for_each(|i| deck[s.to(i) as usize] = i);
        assert_eq!(vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6], deck);
    }

    #[test]
    fn case7() {
        let data = "deal with increment 7
        deal with increment 9
        cut -2";
        let actions = parse(&data);
        let deck_size = 10i64;
        let mut s = Shuffle::new(deck_size);
        s.compose(&actions);
        let mut deck = vec![0; deck_size as usize];
        (0..deck_size).for_each(|i| deck[s.to(i) as usize] = i);
        assert_eq!(vec![6, 3, 0, 7, 4, 1, 8, 5, 2, 9], deck);
    }

    #[test]
    fn case8() {
        let data = "deal into new stack
        cut -2
        deal with increment 7
        cut 8
        cut -4
        deal with increment 7
        cut 3
        deal with increment 9
        deal with increment 3
        cut -1";
        let actions = parse(&data);
        let deck_size = 10i64;
        let mut s = Shuffle::new(deck_size);
        s.compose(&actions);
        let mut deck = vec![0; deck_size as usize];
        (0..deck_size).for_each(|i| deck[s.to(i) as usize] = i);
        assert_eq!(vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6], deck);
    }
}
