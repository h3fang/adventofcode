use ahash::AHashSet as HashSet;

struct Image {
    img: Vec<bool>,
    width: usize,
    bounds: (usize, usize, usize, usize),
}

impl Image {
    fn update_bounds<T: Ord + Copy>(bounds: &mut (T, T, T, T), x: T, y: T) {
        bounds.0 = bounds.0.min(x);
        bounds.1 = bounds.1.min(y);
        bounds.2 = bounds.2.max(x);
        bounds.3 = bounds.3.max(y);
    }

    fn from_lines(s: &str) -> Self {
        let mut pixels = HashSet::new();
        let mut bounds = (usize::MAX, usize::MAX, usize::MIN, usize::MIN);
        s.lines().enumerate().for_each(|(y, line)| {
            line.as_bytes().iter().enumerate().for_each(|(x, &pixel)| {
                if pixel == b'#' {
                    pixels.insert((x, y));
                    Self::update_bounds(&mut bounds, x, y);
                }
            })
        });
        let height = (bounds.2 - bounds.0 + 4 * 50) as usize;
        let width = (bounds.3 - bounds.1 + 4 * 50) as usize;
        let mut img = vec![false; width * height];
        for (x, y) in pixels {
            img[(y + 2 * 50) * width + x + 2 * 50] = true;
        }
        let bounds = (
            (bounds.0 + 2 * 50) as usize,
            (bounds.1 + 2 * 50) as usize,
            (bounds.2 + 2 * 50) as usize,
            (bounds.3 + 2 * 50) as usize,
        );
        Self { img, width, bounds }
    }

    fn combine(img: &[bool], width: usize, (x, y): (usize, usize)) -> usize {
        let mut result = 0;
        for yn in [y - 1, y, y + 1] {
            for xn in [x - 1, x, x + 1] {
                let pixel = img[yn * width + xn];
                result = (result << 1) + if pixel { 1 } else { 0 };
            }
        }
        result
    }

    fn enhance_twice(&mut self, algo: &[bool]) {
        let bounds = self.bounds;
        let mut new = vec![false; self.img.len()];

        // first
        for x in bounds.0 - 4..=bounds.2 + 4 {
            for y in bounds.1 - 4..=bounds.3 + 4 {
                let i = Self::combine(&self.img, self.width, (x, y));
                new[y * self.width + x] = algo[i];
            }
        }

        // second
        for x in bounds.0 - 2..=bounds.2 + 2 {
            for y in bounds.1 - 2..=bounds.3 + 2 {
                let i = Self::combine(&new, self.width, (x, y));
                self.img[y * self.width + x] = algo[i];
                if algo[i] {
                    Self::update_bounds(&mut self.bounds, x, y);
                }
            }
        }
    }

    fn lit_pixels(&self) -> usize {
        self.img.iter().filter(|p| **p).count()
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
