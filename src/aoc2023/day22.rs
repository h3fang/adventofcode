use ahash::HashSet;
use arrayvec::ArrayVec;

fn parse(data: &str) -> Vec<[ArrayVec<u16, 3>; 2]> {
    let mut r: Vec<[ArrayVec<u16, 3>; 2]> = data
        .trim()
        .lines()
        .map(|l| {
            let (a, b) = l.split_once('~').unwrap();
            let a: ArrayVec<u16, 3> = a.split(',').map(|t| t.parse().unwrap()).collect();
            let b: ArrayVec<u16, 3> = b.split(',').map(|t| t.parse().unwrap()).collect();
            assert!(a[0] <= b[0]);
            assert!(a[1] <= b[1]);
            assert!(a[2] <= b[2]);
            [a, b]
        })
        .collect();
    r.sort_unstable_by_key(|e| (e[0][2], e[1][2]));
    r
}

fn fall(mut bricks: Vec<[ArrayVec<u16, 3>; 2]>) -> (Vec<HashSet<i32>>, Vec<HashSet<i32>>) {
    let n = bricks.len();
    let mut height = [[(0, -1); 10]; 10];
    let mut support: Vec<HashSet<i32>> = vec![HashSet::default(); n];
    let mut supported_by: Vec<HashSet<i32>> = vec![HashSet::default(); n];
    for (i, a) in bricks.iter_mut().enumerate() {
        let mut max = 0;
        let bottom_bricks = &mut supported_by[i];
        for x in a[0][0]..=a[1][0] {
            for y in a[0][1]..=a[1][1] {
                let (h, j) = height[x as usize][y as usize];
                match max.cmp(&h) {
                    std::cmp::Ordering::Less => {
                        max = h;
                        bottom_bricks.clear();
                        if j >= 0 {
                            bottom_bricks.insert(j);
                        }
                    }
                    std::cmp::Ordering::Equal => {
                        if j >= 0 {
                            bottom_bricks.insert(j);
                        }
                    }
                    std::cmp::Ordering::Greater => {}
                }
            }
        }
        let h = a[1][2] - a[0][2] + max + 1;
        a[0][2] = max + 1;
        a[1][2] = h;
        for x in a[0][0]..=a[1][0] {
            for y in a[0][1]..=a[1][1] {
                height[x as usize][y as usize] = (h, i as i32);
            }
        }
        for &t in bottom_bricks.iter() {
            support[t as usize].insert(i as i32);
        }
    }
    (support, supported_by)
}

fn part1(support: &[HashSet<i32>], supported_by: &[HashSet<i32>]) -> usize {
    support
        .iter()
        .filter(|s| s.iter().all(|&i| supported_by[i as usize].len() > 1))
        .count()
}

fn part2(support: &[HashSet<i32>], supported_by: &[HashSet<i32>]) -> usize {
    let n = support.len();
    let mut result = 0;
    let mut q = Vec::with_capacity(n);
    for i in 0..n {
        let mut moved = vec![false; n];
        moved[i] = true;
        for &a in &support[i] {
            q.push(a);
        }
        while let Some(i) = q.pop() {
            if supported_by[i as usize].iter().all(|&b| moved[b as usize]) {
                moved[i as usize] = true;
                result += 1;
                q.extend(&support[i as usize]);
            }
        }
    }
    result
}

pub fn main() {
    let data = std::fs::read_to_string("data/2023/day22").unwrap();
    let bricks = parse(&data);
    let (support, supported_by) = fall(bricks);
    println!("part1: {}", part1(&support, &supported_by));
    println!("part2: {}", part2(&support, &supported_by));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = r"
1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
        let bricks = parse(data);
        let (support, supported_by) = fall(bricks);
        assert_eq!(5, part1(&support, &supported_by));
        assert_eq!(7, part2(&support, &supported_by));
    }
}
