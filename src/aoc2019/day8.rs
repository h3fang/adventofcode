const WIDTH: usize = 25;
const HEIGHT: usize = 6;
const LAYER_SIZE: usize = WIDTH * HEIGHT;

fn part1(image: &str) -> usize {
    let img = image.as_bytes();
    let zeros = img
        .chunks_exact(LAYER_SIZE)
        .enumerate()
        .map(|(i, layer)| (i, layer.iter().filter(|p| **p == b'0').count()))
        .min_by_key(|e| e.1)
        .expect("smaller than one layer");
    let ones = img
        .chunks_exact(LAYER_SIZE)
        .nth(zeros.0)
        .unwrap()
        .iter()
        .filter(|b| **b == b'1')
        .count();
    ones * (LAYER_SIZE - ones - zeros.1)
}

fn part2(image: &str) {
    let img = image.as_bytes();
    let layers = img.chunks_exact(LAYER_SIZE).collect::<Vec<_>>();
    let mut rendered = [b' '; LAYER_SIZE];
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            for layer in &layers {
                let p = layer[i * WIDTH + j];
                if p != b'2' {
                    if p == b'1' {
                        rendered[i * WIDTH + j] = p;
                    }
                    break;
                }
            }
        }
        println!("{}", unsafe {
            std::str::from_utf8_unchecked(&rendered[i * WIDTH..i * WIDTH + WIDTH])
        });
    }
}

pub fn main() {
    let data = std::fs::read_to_string("data/2019/day8").unwrap();
    let image = data.lines().next().unwrap();

    println!("day8 part1: {}", part1(image));
    println!("day8 part2:");
    part2(image);
}
