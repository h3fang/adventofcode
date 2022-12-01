fn parse(data: &str) -> usize {
    data.trim().parse().unwrap()
}

fn part1(num: usize) -> String {
    let mut q: Vec<u8> = Vec::with_capacity(num + 10);
    q.push(3);
    q.push(7);
    let mut i = 0;
    let mut j = 1;
    while q.len() < num + 10 {
        let a = q[i];
        let b = q[j];
        let s = a + b;
        if s >= 10 {
            q.push(s / 10);
            q.push(s % 10);
        } else {
            q.push(s);
        }
        i = (i + 1 + a as usize) % q.len();
        j = (j + 1 + b as usize) % q.len();
    }
    q[num..num + 10]
        .iter()
        .map(|i| (i + b'0') as char)
        .collect()
}

fn part2(recipes: &str) -> usize {
    let n = recipes.len();
    let num = recipes.parse::<usize>().unwrap();
    let ten_to_n = 10usize.pow(n as u32 - 1);

    let mut q: Vec<u8> = vec![3, 7];
    let mut i = 0;
    let mut j = 1;
    let mut pre = 0;
    let mut len = 0;

    let mut validate = |d: u8| {
        if len == n {
            pre %= ten_to_n;
        } else {
            len += 1;
        }
        pre = pre * 10 + d as usize;
        len == n && pre == num
    };
    loop {
        let a = q[i];
        let b = q[j];
        let s = a + b;
        if s >= 10 {
            q.push(s / 10);
            if validate(*q.last().unwrap()) {
                return q.len() - n;
            }
            q.push(s % 10);
        } else {
            q.push(s);
        }
        if validate(*q.last().unwrap()) {
            return q.len() - n;
        }

        i = (i + 1 + a as usize) % q.len();
        j = (j + 1 + b as usize) % q.len();
    }
}

pub fn main() {
    let data = std::fs::read_to_string("data/2018/day14").unwrap();
    let num = parse(&data);
    println!("part1: {}", part1(num));
    println!("part2: {}", part2(data.trim()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!("5158916779", part1(9));
        assert_eq!("0124515891", part1(5));
        assert_eq!("9251071085", part1(18));
        assert_eq!("5941429882", part1(2018));
    }

    #[test]
    fn test_part2() {
        assert_eq!(9, part2("51589"));
        assert_eq!(5, part2("01245"));
        assert_eq!(18, part2("92510"));
        assert_eq!(2018, part2("59414"));
    }
}
