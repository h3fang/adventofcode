use nom::{
    bytes::complete::tag,
    character::complete::{char as ch, digit1, space0},
    combinator::{eof, map_res, opt, recognize},
    sequence::tuple,
    IResult,
};

type Vec2 = (i32, i32);

fn parse_i32(input: &str) -> IResult<&str, i32> {
    let num = tuple((space0, opt(ch('-')), digit1));
    map_res(recognize(num), |s: &str| s.trim_start().parse())(input)
}

fn parse_vec2(input: &str) -> IResult<&str, Vec2> {
    let (r, (_, x, _, y, _)) = tuple((ch('<'), parse_i32, tag(", "), parse_i32, ch('>')))(input)?;
    Ok((r, (x, y)))
}

fn parse_star(input: &str) -> IResult<&str, (Vec2, Vec2)> {
    let (r, (_, vp, _, _, ve, _)) = tuple((
        tag("position="),
        parse_vec2,
        ch(' '),
        tag("velocity="),
        parse_vec2,
        eof,
    ))(input)?;
    Ok((r, (vp, ve)))
}

fn parse(data: &str) -> Vec<(Vec2, Vec2)> {
    data.lines()
        .filter(|line| !line.is_empty())
        .map(|line| parse_star(line.trim()).map(|r| r.1).unwrap())
        .collect()
}

fn step_forward(stars: &mut [(Vec2, Vec2)]) {
    stars.iter_mut().for_each(|(p, v)| {
        p.0 += v.0;
        p.1 += v.1;
    });
}

fn step_backward(stars: &mut [(Vec2, Vec2)]) {
    stars.iter_mut().for_each(|(p, v)| {
        p.0 -= v.0;
        p.1 -= v.1;
    });
}

fn aabb(stars: &[(Vec2, Vec2)]) -> (Vec2, Vec2) {
    let mut min = (i32::MAX, i32::MAX);
    let mut max = (i32::MIN, i32::MIN);
    stars.iter().for_each(|(p, _)| {
        min.0 = min.0.min(p.0);
        min.1 = min.1.min(p.1);
        max.0 = max.0.max(p.0);
        max.1 = max.1.max(p.1);
    });
    (min, max)
}

fn area(stars: &[(Vec2, Vec2)]) -> i64 {
    let (min, max) = aabb(stars);
    (max.0 - min.0) as i64 * (max.1 - min.1) as i64
}

fn print_stars(stars: &[(Vec2, Vec2)]) {
    let (min, max) = aabb(stars);
    let w = (max.0 - min.0 + 1) as usize;
    let h = (max.1 - min.1 + 1) as usize;
    let mut grid = vec![vec![b' '; w]; h];
    stars
        .iter()
        .for_each(|(p, _)| grid[(p.1 - min.1) as usize][(p.0 - min.0) as usize] = b'#');
    grid.iter()
        .for_each(|r| println!("{}", unsafe { std::str::from_utf8_unchecked(r) }));
}

fn solve(stars: &mut [(Vec2, Vec2)]) -> i32 {
    let mut last_area = area(stars);
    for i in 0.. {
        step_forward(stars);
        let curr = area(stars);
        if curr > last_area {
            step_backward(stars);
            return i;
        }
        last_area = curr;
    }
    -1
}

pub fn main() {
    let data = std::fs::read_to_string("data/2018/day10").unwrap();
    let mut stars = parse(&data);
    let t = solve(&mut stars);
    println!("part1:");
    print_stars(&stars);
    println!("part2: {}", t);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "
        position=< 9,  1> velocity=< 0,  2>
        position=< 7,  0> velocity=<-1,  0>
        position=< 3, -2> velocity=<-1,  1>
        position=< 6, 10> velocity=<-2, -1>
        position=< 2, -4> velocity=< 2,  2>
        position=<-6, 10> velocity=< 2, -2>
        position=< 1,  8> velocity=< 1, -1>
        position=< 1,  7> velocity=< 1,  0>
        position=<-3, 11> velocity=< 1, -2>
        position=< 7,  6> velocity=<-1, -1>
        position=<-2,  3> velocity=< 1,  0>
        position=<-4,  3> velocity=< 2,  0>
        position=<10, -3> velocity=<-1,  1>
        position=< 5, 11> velocity=< 1, -2>
        position=< 4,  7> velocity=< 0, -1>
        position=< 8, -2> velocity=< 0,  1>
        position=<15,  0> velocity=<-2,  0>
        position=< 1,  6> velocity=< 1,  0>
        position=< 8,  9> velocity=< 0, -1>
        position=< 3,  3> velocity=<-1,  1>
        position=< 0,  5> velocity=< 0, -1>
        position=<-2,  2> velocity=< 2,  0>
        position=< 5, -2> velocity=< 1,  2>
        position=< 1,  4> velocity=< 2,  1>
        position=<-2,  7> velocity=< 2, -2>
        position=< 3,  6> velocity=<-1, -1>
        position=< 5,  0> velocity=< 1,  0>
        position=<-6,  0> velocity=< 2,  0>
        position=< 5,  9> velocity=< 1, -2>
        position=<14,  7> velocity=<-2,  0>
        position=<-3,  6> velocity=< 2, -1>"
            .to_string();
        let mut stars = parse(&data);
        let t = solve(&mut stars);
        print_stars(&stars);
        assert_eq!(3, t);
    }
}
