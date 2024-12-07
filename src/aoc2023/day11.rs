fn parse(data: &str) -> Vec<&[u8]> {
    data.trim().lines().map(|line| line.as_bytes()).collect()
}

fn expand(image: &[&[u8]], empty_line_width: usize) -> usize {
    let mut result = 0;
    let mut stars = vec![];
    for (i, r) in image.iter().enumerate() {
        for (j, &c) in r.iter().enumerate() {
            if c != b'#' {
                continue;
            }
            result += stars
                .iter()
                .map(|&(i1, j1)| i.abs_diff(i1) + j.abs_diff(j1))
                .sum::<usize>();
            stars.push((i, j));
        }
    }

    let total = image
        .iter()
        .map(|row| row.iter().filter(|&&c| c == b'#').count())
        .sum::<usize>();

    let mut pre = 0;
    for r in image {
        let stars = r.iter().filter(|&&c| c == b'#').count();
        if stars == 0 {
            result += (empty_line_width - 1) * pre * (total - pre);
        } else {
            pre += stars;
        }
    }

    let mut pre = 0;
    for j in 0..image[0].len() {
        let stars = image.iter().filter(|r| r[j] == b'#').count();
        if stars == 0 {
            result += (empty_line_width - 1) * pre * (total - pre);
        } else {
            pre += stars;
        }
    }

    result
}

pub fn main() {
    let data = std::fs::read_to_string("data/2023/day11").unwrap();
    let image = parse(&data);
    println!("part1: {}", expand(&image, 2));
    println!("part2: {}", expand(&image, 100_0000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";
        let image = parse(data);
        assert_eq!(374, expand(&image, 2));
        assert_eq!(1030, expand(&image, 10));
        assert_eq!(8410, expand(&image, 100));
    }
}
