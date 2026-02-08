struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn square_distance(&self, other: &Self) -> i64 {
        (self.x - other.x) * (self.x - other.x)
            + (self.y - other.y) * (self.y - other.y)
            + (self.z - other.z) * (self.z - other.z)
    }
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

fn parse(data: &str) -> Vec<Point> {
    data.trim()
        .lines()
        .map(|l| {
            let coords: Vec<i64> = l.split(',').map(|s| s.parse().unwrap()).collect();
            Point {
                x: coords[0],
                y: coords[1],
                z: coords[2],
            }
        })
        .collect()
}

fn solve(points: &[Point], mut max_connections: usize) -> (u32, i64) {
    let n = points.len();
    let mut dists = Vec::with_capacity(n * n / 2);
    for (i, x) in points.iter().enumerate() {
        for (j, y) in points.iter().enumerate().skip(i + 1) {
            dists.push((x.square_distance(y), i, j));
        }
    }
    dists.sort_unstable_by_key(|e| e.0);

    let (mut p1, mut p2) = (0, 0);

    let mut dsu = Dsu::new(n);
    for (_d, i, j) in dists {
        dsu.union(i, j);

        if max_connections > 0 {
            max_connections -= 1;
            if max_connections == 0 {
                let mut roots = vec![false; n];
                for i in 0..n {
                    roots[dsu.find(i)] = true;
                }

                let mut sizes: Vec<u32> = roots
                    .into_iter()
                    .enumerate()
                    .filter(|e| e.1)
                    .map(|e| dsu.size[e.0])
                    .collect();
                sizes.sort_unstable();

                p1 = sizes.iter().rev().take(3).product();
            }
        }

        let r = dsu.find(i);
        if dsu.size[r] == n as u32 {
            p2 = points[i].x * points[j].x;
            break;
        }
    }

    (p1, p2)
}

pub fn main() {
    let data = std::fs::read_to_string("data/2025/day8").unwrap();
    let points = parse(&data);
    let (p1, p2) = solve(&points, 1000);
    println!("part1: {p1}");
    println!("part2: {p2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
        let points = parse(data);
        let (p1, p2) = solve(&points, 10);
        assert_eq!(40, p1);
        assert_eq!(25272, p2);
    }
}
