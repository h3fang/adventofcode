use ahash::AHashSet as HashSet;

type Points = HashSet<(usize, usize)>;

fn fold(points: Points, (fold_dir, p): (u8, usize)) -> Points {
    match fold_dir {
        b'x' => points
            .into_iter()
            .map(|(x, y)| if x > p { (2 * p - x, y) } else { (x, y) })
            .collect(),
        b'y' => points
            .into_iter()
            .map(|(x, y)| if y > p { (x, 2 * p - y) } else { (x, y) })
            .collect(),
        _ => panic!(),
    }
}

fn print_grid(points: &Points) {
    let (width, height) = points
        .iter()
        .fold((0, 0), |(w, h), p| (w.max(p.0), h.max(p.1)));
    let mut paper = vec![vec![false; width + 1]; height + 1];
    for &(x, y) in points {
        paper[y][x] = true;
    }
    for row in &paper {
        for &cell in row {
            print!("{}", if cell { '█' } else { ' ' });
        }
        println!();
    }
}

fn parse(data: &str) -> (Points, Vec<(u8, usize)>) {
    let mut lines = data.lines();

    let mut points = HashSet::new();
    let mut max_x = 0;
    let mut max_y = 0;
    let mut fold_lines = vec![];
    loop {
        let line = lines.next().unwrap().trim();
        if line.is_empty() {
            break;
        }
        let mut xy = line.split(',');
        let x = xy.next().unwrap().parse::<usize>().unwrap();
        max_x = max_x.max(x);
        let y = xy.next().unwrap().parse::<usize>().unwrap();
        max_y = max_y.max(y);
        points.insert((x, y));
    }
    for line in lines {
        let mut parts = line.trim().split('=');
        let xy = *parts.next().unwrap().as_bytes().last().unwrap();
        let pos = parts.next().unwrap().parse::<usize>().unwrap();
        fold_lines.push((xy, pos));
    }

    (points, fold_lines)
}

pub fn main() {
    let data = std::fs::read_to_string("data/2021/day13").unwrap();
    let (mut points, fold_lines) = parse(&data);

    points = fold(points, fold_lines[0]);
    println!("day13 part1: {}", points.len());
    for &line in &fold_lines[1..] {
        points = fold(points, line);
    }
    println!("day13 part2:");
    print_grid(&points);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "6,10
        0,14
        9,10
        0,3
        10,4
        4,11
        6,0
        6,12
        4,1
        0,13
        10,12
        3,4
        3,0
        8,4
        1,10
        2,14
        8,10
        9,0

        fold along y=7
        fold along x=5";
        let (mut points, fold_lines) = parse(data);
        print_grid(&points);
        points = fold(points, fold_lines[0]);
        assert_eq!(17, points.len());
        print_grid(&points);
        points = fold(points, fold_lines[1]);
        assert_eq!(16, points.len());
        print_grid(&points);
    }
}
