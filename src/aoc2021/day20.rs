use ahash::AHashMap as HashMap;

struct Image {
    img: HashMap<(i32, i32), bool>,
    bounds: (i32, i32, i32, i32),
}

impl Default for Image {
    fn default() -> Self {
        Self {
            img: Default::default(),
            bounds: (i32::MAX, i32::MAX, i32::MIN, i32::MIN),
        }
    }
}

impl Image {
    fn update_bounds(&mut self, x: i32, y: i32) {
        self.bounds.0 = self.bounds.0.min(x);
        self.bounds.1 = self.bounds.1.min(y);
        self.bounds.2 = self.bounds.2.max(x);
        self.bounds.3 = self.bounds.3.max(y);
    }

    fn from_lines(s: &str) -> Self {
        let mut result = Self::default();
        s.lines().enumerate().for_each(|(y, line)| {
            line.as_bytes().iter().enumerate().for_each(|(x, &pixel)| {
                if pixel == b'#' {
                    result.img.insert((x as i32, y as i32), true);
                    result.update_bounds(x as i32, y as i32);
                }
            })
        });
        result
    }

    fn combine(img: &HashMap<(i32, i32), bool>, (x, y): (i32, i32)) -> usize {
        let mut result = 0;
        for yn in [y - 1, y, y + 1] {
            for xn in [x - 1, x, x + 1] {
                let pixel = *img.get(&(xn, yn)).unwrap_or(&false);
                result = (result << 1) + if pixel { 1 } else { 0 };
            }
        }
        result
    }

    fn enhance_twice(&mut self, algo: &[bool]) {
        let bounds = self.bounds;

        let mut new = HashMap::with_capacity(
            (bounds.2 - bounds.0 + 9) as usize * (bounds.3 - bounds.1 + 9) as usize,
        );

        // first
        for x in bounds.0 - 4..=bounds.2 + 4 {
            for y in bounds.1 - 4..=bounds.3 + 4 {
                let i = Self::combine(&self.img, (x, y));
                new.insert((x, y), algo[i]);
            }
        }

        // second
        for x in bounds.0 - 2..=bounds.2 + 2 {
            for y in bounds.1 - 2..=bounds.3 + 2 {
                let i = Self::combine(&new, (x, y));
                self.img.insert((x, y), algo[i]);
                if algo[i] {
                    self.update_bounds(x, y);
                }
            }
        }
    }

    fn lit_pixels(&self) -> usize {
        self.img.values().filter(|p| **p).count()
    }
}

fn parse(data: &str) -> (Vec<bool>, Image) {
    let mut parts = data.split("\n\n");
    let algo = parts
        .next()
        .unwrap()
        .as_bytes()
        .iter()
        .map(|p| *p == b'#')
        .collect::<Vec<_>>();
    assert_eq!(512, algo.len());
    let img = Image::from_lines(parts.next().unwrap());
    (algo, img)
}

fn enhace(algo: &[bool], img: &mut Image, times: usize) -> usize {
    for _ in 0..times / 2 {
        img.enhance_twice(algo);
    }
    img.lit_pixels()
}

pub fn main() {
    let data = std::fs::read_to_string("data/2021/day20").unwrap();
    let (algo, mut img) = parse(&data);
    println!("day20 part1: {}", enhace(&algo, &mut img, 2));
    println!("day20 part2: {}", enhace(&algo, &mut img, 48));
}
