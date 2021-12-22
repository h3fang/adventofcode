use ahash::AHashMap as HashMap;
use ahash::AHashSet as HashSet;
// use lazy_static::lazy_static;
use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Vec3 {
    x: i32,
    y: i32,
    z: i32,
}

impl Vec3 {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    fn zero() -> Self {
        Self { x: 0, y: 0, z: 0 }
    }

    fn square_len(&self) -> i32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn abs(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl Add<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub<&Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Div<i32> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: i32) -> Vec3 {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Mul<i32> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: i32) -> Vec3 {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

// There is no need to do full matrix vector multiplication.
// Both ORIENTATIONS and ROTATIONS works. The computation time difference is negligible.

// #[derive(Debug, Clone, Copy)]
// struct Matrix3 {
//     arr: [i32; 9],
// }

// impl Matrix3 {
//     fn identity() -> Self {
//         Self {
//             arr: [1, 0, 0, 0, 1, 0, 0, 0, 1],
//         }
//     }

//     fn rotx_90() -> Self {
//         Self {
//             arr: [1, 0, 0, 0, 0, -1, 0, 1, 0],
//         }
//     }

//     fn roty_90() -> Self {
//         Self {
//             arr: [0, 0, 1, 0, 1, 0, -1, 0, 0],
//         }
//     }

//     fn rotz_90() -> Self {
//         Self {
//             arr: [0, -1, 0, 1, 0, 0, 0, 0, 1],
//         }
//     }
// }

// impl Mul for Matrix3 {
//     type Output = Self;

//     fn mul(self, rhs: Self) -> Self::Output {
//         Self {
//             arr: [
//                 self.arr[0] * rhs.arr[0] + self.arr[1] * rhs.arr[3] + self.arr[2] * rhs.arr[6],
//                 self.arr[0] * rhs.arr[1] + self.arr[1] * rhs.arr[4] + self.arr[2] * rhs.arr[7],
//                 self.arr[0] * rhs.arr[2] + self.arr[1] * rhs.arr[5] + self.arr[2] * rhs.arr[8],
//                 self.arr[3] * rhs.arr[0] + self.arr[4] * rhs.arr[3] + self.arr[5] * rhs.arr[6],
//                 self.arr[3] * rhs.arr[1] + self.arr[4] * rhs.arr[4] + self.arr[5] * rhs.arr[7],
//                 self.arr[3] * rhs.arr[2] + self.arr[4] * rhs.arr[5] + self.arr[5] * rhs.arr[8],
//                 self.arr[6] * rhs.arr[0] + self.arr[7] * rhs.arr[3] + self.arr[8] * rhs.arr[6],
//                 self.arr[6] * rhs.arr[1] + self.arr[7] * rhs.arr[4] + self.arr[8] * rhs.arr[7],
//                 self.arr[6] * rhs.arr[2] + self.arr[7] * rhs.arr[5] + self.arr[8] * rhs.arr[8],
//             ],
//         }
//     }
// }

// impl Mul<&Vec3> for &Matrix3 {
//     type Output = Vec3;

//     fn mul(self, rhs: &Vec3) -> Self::Output {
//         Self::Output {
//             x: self.arr[0] * rhs.x + self.arr[1] * rhs.y + self.arr[2] * rhs.z,
//             y: self.arr[3] * rhs.x + self.arr[4] * rhs.y + self.arr[5] * rhs.z,
//             z: self.arr[6] * rhs.x + self.arr[7] * rhs.y + self.arr[8] * rhs.z,
//         }
//     }
// }

// impl Display for Matrix3 {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(
//             f,
//             "{:^3} {:^3} {:^3}\n{:^3} {:^3} {:^3}\n{:^3} {:^3} {:^3}",
//             self.arr[0],
//             self.arr[1],
//             self.arr[2],
//             self.arr[3],
//             self.arr[4],
//             self.arr[5],
//             self.arr[6],
//             self.arr[7],
//             self.arr[8]
//         )
//     }
// }

// lazy_static! {
//     static ref ORIENTATIONS: Vec<Matrix3> = {
//         let mut r = Vec::new();

//         let rotate_x = Matrix3::rotx_90();
//         let rotate_y = Matrix3::roty_90();
//         let rotate_z = Matrix3::rotz_90();

//         // x
//         for mut curr in [Matrix3::identity(), rotate_y * rotate_y] {
//             for _ in 0..4 {
//                 r.push(curr);
//                 curr = rotate_x * curr;
//             }
//         }

//         // y
//         for mut curr in [rotate_z, rotate_x * rotate_x * rotate_z] {
//             for _ in 0..4 {
//                 r.push(curr);
//                 curr = rotate_y * curr;
//             }
//         }

//         // z
//         for mut curr in [rotate_y, rotate_x * rotate_x * rotate_y] {
//             for _ in 0..4 {
//                 r.push(curr);
//                 curr = rotate_z * curr;
//             }
//         }

//         r
//     };
// }

const ROTATIONS: [fn(&Vec3) -> Vec3; 24] = [
    |&Vec3 { x, y, z }| Vec3::new(x, y, z),
    |&Vec3 { x, y, z }| Vec3::new(x, -z, y),
    |&Vec3 { x, y, z }| Vec3::new(x, -y, -z),
    |&Vec3 { x, y, z }| Vec3::new(x, z, -y),
    |&Vec3 { x, y, z }| Vec3::new(-x, y, -z),
    |&Vec3 { x, y, z }| Vec3::new(-x, z, y),
    |&Vec3 { x, y, z }| Vec3::new(-x, -y, z),
    |&Vec3 { x, y, z }| Vec3::new(-x, -z, -y),
    |&Vec3 { x, y, z }| Vec3::new(-y, x, z),
    |&Vec3 { x, y, z }| Vec3::new(z, x, y),
    |&Vec3 { x, y, z }| Vec3::new(y, x, -z),
    |&Vec3 { x, y, z }| Vec3::new(-z, x, -y),
    |&Vec3 { x, y, z }| Vec3::new(-y, -x, -z),
    |&Vec3 { x, y, z }| Vec3::new(-z, -x, y),
    |&Vec3 { x, y, z }| Vec3::new(y, -x, z),
    |&Vec3 { x, y, z }| Vec3::new(z, -x, -y),
    |&Vec3 { x, y, z }| Vec3::new(z, y, -x),
    |&Vec3 { x, y, z }| Vec3::new(-y, z, -x),
    |&Vec3 { x, y, z }| Vec3::new(-z, -y, -x),
    |&Vec3 { x, y, z }| Vec3::new(y, -z, -x),
    |&Vec3 { x, y, z }| Vec3::new(z, -y, x),
    |&Vec3 { x, y, z }| Vec3::new(y, z, x),
    |&Vec3 { x, y, z }| Vec3::new(-z, y, x),
    |&Vec3 { x, y, z }| Vec3::new(-y, -z, x),
];

type FingerprintSouce = HashMap<i32, Vec<Vec3>>;

#[derive(Debug, Clone)]
struct Scanner {
    position: Vec3,
    fingerprints: HashSet<i32>,
    fingerprint_source: FingerprintSouce,
    rotation_index: usize,
    rotations: Vec<HashSet<Vec3>>,
}

impl Scanner {
    fn new(id: usize, beacons: Vec<Vec3>) -> Self {
        let mut fingerprints = HashSet::new();
        let mut fingerprint_source: FingerprintSouce = HashMap::new();

        for (i, a) in beacons.iter().enumerate() {
            for b in beacons.iter().skip(i + 1) {
                let fp = (a - b).square_len();
                fingerprints.insert(fp);
                fingerprint_source.entry(fp).or_default().push(a + b);
            }
        }

        let beacons = beacons.into_iter().collect::<HashSet<_>>();
        let mut rotations = vec![beacons];

        if id != 0 {
            for rotate in &ROTATIONS[1..] {
                let beacons = rotations[0].iter().map(rotate).collect();
                rotations.push(beacons);
            }
        }

        Self {
            position: Vec3::zero(),
            rotation_index: 0,
            rotations,
            fingerprints,
            fingerprint_source,
        }
    }

    fn beacons(&self) -> &HashSet<Vec3> {
        &self.rotations[self.rotation_index]
    }

    fn rotate(&mut self, rotation_index: usize) {
        self.rotation_index = rotation_index;
        self.fingerprint_source.values_mut().for_each(|p| {
            p.iter_mut().for_each(|p| {
                *p = ROTATIONS[rotation_index](p);
            })
        });
    }

    fn translate(&mut self, translation: Vec3) {
        self.rotations[self.rotation_index] =
            self.beacons().iter().map(|p| p + &translation).collect();
        let trans2 = &translation * 2;
        self.fingerprint_source.values_mut().for_each(|pts| {
            pts.iter_mut().for_each(|p| {
                *p = &*p + &trans2;
            })
        });
        self.position = translation;
    }

    fn align(&self, other: &mut Self) -> bool {
        let fps = self
            .fingerprints
            .intersection(&other.fingerprints)
            .cloned()
            .collect::<Vec<_>>();

        // Assume there are at least 6 different distances. These is not true for
        // general cases, but is fine with the puzzle input. And this can greatly reduce
        // the computation time.

        if fps.len() < 6 {
            return false;
        }

        for (rotation_index, rotation) in ROTATIONS.iter().enumerate() {
            for fp in &fps {
                for pa in self.fingerprint_source.get(fp).unwrap() {
                    for pb in other.fingerprint_source.get(fp).unwrap().clone() {
                        let pb = rotation(&pb);
                        let translation = &(pa - &pb) / 2;
                        if other.rotations[rotation_index]
                            .iter()
                            .filter(|p| self.beacons().contains(&(*p + &translation)))
                            .take(12)
                            .count()
                            == 12
                        {
                            other.rotate(rotation_index);
                            other.translate(translation);
                            return true;
                        }
                    }
                }
            }
        }
        false
    }
}

fn parse(data: &str) -> Vec<Scanner> {
    let lines = data.lines().collect::<Vec<_>>();
    lines
        .split(|line| line.is_empty())
        .map(|g| {
            let id = g[0]
                .trim()
                .split_ascii_whitespace()
                .nth(2)
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let beacons = g[1..]
                .iter()
                .map(|p| {
                    let v = p
                        .trim()
                        .split(',')
                        .map(|e| e.parse::<i32>().unwrap())
                        .collect::<Vec<_>>();
                    Vec3::new(v[0], v[1], v[2])
                })
                .collect();
            Scanner::new(id, beacons)
        })
        .collect()
}

fn solve(mut scanners: Vec<Scanner>) -> (usize, i32) {
    let mut done = vec![scanners.remove(0)];
    while !scanners.is_empty() {
        let mut aligned = false;
        'outer: for i in 0..scanners.len() {
            for a in &done {
                if a.align(&mut scanners[i]) {
                    aligned = true;
                    done.push(scanners.remove(i));
                    break 'outer;
                }
            }
        }
        if !aligned {
            break;
        }
    }
    let p1 = done
        .iter()
        .map(|s| s.beacons().iter())
        .flatten()
        .collect::<HashSet<_>>()
        .len();
    let positions = done.iter().map(|s| &s.position).collect::<Vec<_>>();
    let mut p2 = 0;
    for (i, a) in positions.iter().enumerate() {
        for b in positions.iter().skip(i + 1) {
            p2 = p2.max((*a - *b).abs());
        }
    }
    (p1, p2)
}

pub fn main() {
    let data = std::fs::read_to_string("data/2021/day19").unwrap();
    let scanners = parse(&data);
    let (p1, p2) = solve(scanners);
    println!("day19 part1: {}", p1);
    println!("day19 part2: {}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "--- scanner 0 ---
        404,-588,-901
        528,-643,409
        -838,591,734
        390,-675,-793
        -537,-823,-458
        -485,-357,347
        -345,-311,381
        -661,-816,-575
        -876,649,763
        -618,-824,-621
        553,345,-567
        474,580,667
        -447,-329,318
        -584,868,-557
        544,-627,-890
        564,392,-477
        455,729,728
        -892,524,684
        -689,845,-530
        423,-701,434
        7,-33,-71
        630,319,-379
        443,580,662
        -789,900,-551
        459,-707,401

        --- scanner 1 ---
        686,422,578
        605,423,415
        515,917,-361
        -336,658,858
        95,138,22
        -476,619,847
        -340,-569,-846
        567,-361,727
        -460,603,-452
        669,-402,600
        729,430,532
        -500,-761,534
        -322,571,750
        -466,-666,-811
        -429,-592,574
        -355,545,-477
        703,-491,-529
        -328,-685,520
        413,935,-424
        -391,539,-444
        586,-435,557
        -364,-763,-893
        807,-499,-711
        755,-354,-619
        553,889,-390

        --- scanner 2 ---
        649,640,665
        682,-795,504
        -784,533,-524
        -644,584,-595
        -588,-843,648
        -30,6,44
        -674,560,763
        500,723,-460
        609,671,-379
        -555,-800,653
        -675,-892,-343
        697,-426,-610
        578,704,681
        493,664,-388
        -671,-858,530
        -667,343,800
        571,-461,-707
        -138,-166,112
        -889,563,-600
        646,-828,498
        640,759,510
        -630,509,768
        -681,-892,-333
        673,-379,-804
        -742,-814,-386
        577,-820,562

        --- scanner 3 ---
        -589,542,597
        605,-692,669
        -500,565,-823
        -660,373,557
        -458,-679,-417
        -488,449,543
        -626,468,-788
        338,-750,-386
        528,-832,-391
        562,-778,733
        -938,-730,414
        543,643,-506
        -524,371,-870
        407,773,750
        -104,29,83
        378,-903,-323
        -778,-728,485
        426,699,580
        -438,-605,-362
        -469,-447,-387
        509,732,623
        647,635,-688
        -868,-804,481
        614,-800,639
        595,780,-596

        --- scanner 4 ---
        727,592,562
        -293,-554,779
        441,611,-461
        -714,465,-776
        -743,427,-804
        -660,-479,-426
        832,-632,460
        927,-485,-438
        408,393,-506
        466,436,-512
        110,16,151
        -258,-428,682
        -393,719,612
        -211,-452,876
        808,-476,-593
        -575,615,604
        -485,667,467
        -680,325,-822
        -627,-443,-432
        872,-547,-609
        833,512,582
        807,604,487
        839,-516,451
        891,-625,532
        -652,-548,-490
        30,-46,-14";

        let scanners = parse(&data);
        assert_eq!((79, 3621), solve(scanners));
    }
}
