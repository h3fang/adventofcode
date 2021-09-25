use anyhow::Result;
use std::{
    fs,
    io::{self, BufRead},
};

struct Map {
    tile: Vec<Vec<char>>,
    tile_width: usize,
}

impl Map {
    fn new(tile: Vec<Vec<char>>) -> Self {
        Self {
            tile_width: tile[0].len(),
            tile,
        }
    }

    fn height(&self) -> usize {
        self.tile.len()
    }

    fn get(&self, x: usize, y: usize) -> char {
        self.tile[x][y % self.tile_width]
    }
}

fn count_trees(map: &Map, dx: usize, dy: usize) -> usize {
    let mut x = 0;
    let mut y = 0;
    let mut c = 0;
    while x < map.height() {
        if map.get(x, y) == '#' {
            c += 1;
        }
        x += dx;
        y += dy;
    }
    c
}

pub fn main(file_path: &str) -> Result<()> {
    let data_file = fs::File::open(file_path)?;
    let tile: Vec<_> = io::BufReader::new(data_file)
        .lines()
        .filter_map(|line| line.ok())
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect();
    let map = Map::new(tile);
    let slops = vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];
    let n_trees = slops
        .iter()
        .map(|&(dx, dy)| count_trees(&map, dx, dy))
        .collect::<Vec<_>>();
    println!("day3 part1: {}", n_trees[1]);
    println!("day3 part2: {}", n_trees.iter().product::<usize>());
    Ok(())
}