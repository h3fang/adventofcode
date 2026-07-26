type Shape = Vec<Vec<u8>>;

struct Region {
    width: u32,
    height: u32,
    quantities: Vec<u32>,
}

fn parse(data: &str) -> (Vec<Shape>, Vec<Region>) {
    let parts = data.trim().split("\n\n").collect::<Vec<_>>();

    let shapes = parts
        .iter()
        .take(parts.len() - 1)
        .map(|p| {
            p.trim()
                .lines()
                .skip(1)
                .map(|row| row.as_bytes().to_vec())
                .collect()
        })
        .collect();

    let regions = parts
        .last()
        .unwrap()
        .lines()
        .map(|line| {
            let (size, quantities) = line.split_once(": ").unwrap();

            let (w, h) = size.split_once('x').unwrap();
            let (width, height) = (w.parse().unwrap(), h.parse().unwrap());

            let quantities = quantities.split(' ').map(|q| q.parse().unwrap()).collect();

            Region {
                width,
                height,
                quantities,
            }
        })
        .collect();

    (shapes, regions)
}

fn part1(shapes: &[Shape], regions: &[Region]) -> usize {
    let shape_areas = shapes
        .iter()
        .map(|s| s.iter().flatten().filter(|&&b| b == b'#').count())
        .collect::<Vec<_>>();
    regions
        .iter()
        .filter(|r| {
            let total = r.width * r.height;
            let required = r
                .quantities
                .iter()
                .zip(&shape_areas)
                .map(|(q, area)| q * (*area as u32))
                .sum::<u32>();
            if required > total {
                false
            } else if r.quantities.iter().sum::<u32>() * 9 <= total {
                true
            } else {
                unreachable!("np-hard")
            }
        })
        .count()
}

pub fn main() {
    let data = std::fs::read_to_string("data/2025/day12").unwrap();
    let (shapes, regions) = parse(&data);
    println!("part1: {}", part1(&shapes, &regions));
}
