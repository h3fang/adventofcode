use ahash::HashSet;

fn parse(data: &str) -> Vec<[i8; 4]> {
    data.trim()
        .lines()
        .map(|line| {
            let mut point = [0; 4];
            for (i, n) in line.trim().split(',').enumerate() {
                let n = n.parse().unwrap();
                point[i] = n;
            }
            point
        })
        .collect()
}

#[inline]
fn manhattan(a: &[i8; 4], b: &[i8; 4]) -> i8 {
    a.iter().zip(b).map(|(x, y)| (x - y).abs()).sum()
}

struct Dsu {
    parent: Vec<usize>,
    size: Vec<u32>,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let (px, py) = (self.find(x), self.find(y));
        if px == py {
            return;
        }
        match self.size[px].cmp(&self.size[py]) {
            std::cmp::Ordering::Less => {
                self.size[py] += self.size[px];
                self.parent[px] = py;
            }
            _ => {
                self.size[px] += self.size[py];
                self.parent[py] = px;
            }
        }
    }
}

fn part1(points: &[[i8; 4]]) -> usize {
    let n = points.len();
    let mut dsu = Dsu::new(n);
    for (i, a) in points.iter().enumerate() {
        for (j, b) in points.iter().enumerate().skip(i + 1) {
            if manhattan(a, b) <= 3 {
                dsu.union(i, j);
            }
        }
    }
    let roots: HashSet<usize> = (0..n).map(|i| dsu.find(i)).collect();
    roots.len()
}

pub fn main() {
    let data = std::fs::read_to_string("data/2018/day25").unwrap();
    let points = parse(&data);
    println!("part1: {}", part1(&points));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "
 0,0,0,0
 3,0,0,0
 0,3,0,0
 0,0,3,0
 0,0,0,3
 0,0,0,6
 9,0,0,0
12,0,0,0";
        let points = parse(&data);
        assert_eq!(2, part1(&points));
    }

    #[test]
    fn case2() {
        let data = "
-1,2,2,0
0,0,2,-2
0,0,0,-2
-1,2,0,0
-2,-2,-2,2
3,0,2,-1
-1,3,2,2
-1,0,-1,0
0,2,1,-2
3,0,0,0";
        let points = parse(&data);
        assert_eq!(4, part1(&points));
    }

    #[test]
    fn case3() {
        let data = "
1,-1,0,1
2,0,-1,0
3,2,-1,0
0,0,3,1
0,0,-1,-1
2,3,-2,0
-2,2,0,0
2,-2,0,-1
1,-1,0,-1
3,2,0,2";
        let points = parse(&data);
        assert_eq!(3, part1(&points));
    }

    #[test]
    fn case4() {
        let data = "
1,-1,-1,-2
-2,-2,0,1
0,2,1,3
-2,3,-2,1
0,2,3,-2
-1,-1,1,-2
0,-2,-1,0
-2,2,3,-1
1,2,2,0
-1,-2,0,-2";
        let points = parse(&data);
        assert_eq!(8, part1(&points));
    }
}
