use ahash::AHashMap as HashMap;
use ahash::AHashSet as HashSet;
use std::fmt;

const SEA_MONSTER: &str =
    "Tile 1:\n                  # \n#    ##    ##    ###\n #  #  #  #  #  #   ";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Hash)]
enum Color {
    Black,
    White,
    Any,
    Monster,
}

impl From<char> for Color {
    fn from(c: char) -> Self {
        match c {
            '#' => Color::Black,
            '.' => Color::White,
            ' ' => Color::Any,
            x => panic!("invalid color: {}", x),
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Color::Black => '#',
            Color::White => '.',
            Color::Any => ' ',
            Color::Monster => 'O',
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Hash)]
struct Edge {
    array: Vec<Color>,
}

impl Edge {
    fn from_array(array: Vec<Color>) -> Self {
        let mut reversed = array.to_owned();
        reversed.reverse();
        let e1 = Edge { array };
        let e2 = Edge { array: reversed };
        if e1 < e2 {
            e1
        } else {
            e2
        }
    }
}

#[derive(Clone)]
struct Tile {
    id: usize,
    array: Vec<Vec<Color>>,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let array = self
            .array
            .iter()
            .map(|row| row.iter().map(Color::to_string).collect::<String>())
            .collect::<Vec<_>>();
        write!(f, "Tile {}:\n{}", self.id, array.join("\n"))
    }
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Tile {
    #[allow(dead_code)]
    fn from_str(s: &str) -> Self {
        let lines = s.lines().collect::<Vec<_>>();
        Self::from_lines(&lines)
    }

    fn from_lines(lines: &[&str]) -> Self {
        let header = lines[0];
        assert_eq!("Tile ", &header[..5]);
        assert_eq!(":", &header[header.len() - 1..]);
        let id = header[5..header.len() - 1].parse().unwrap();
        let array = lines[1..]
            .iter()
            .map(|line| line.chars().map(Color::from).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let num_row = array.len();
        assert!(num_row > 0);
        let num_column = array[0].len();

        for arr in &array[1..] {
            assert_eq!(num_column, arr.len());
        }

        Self { id, array }
    }

    fn transpose(&mut self) {
        let m = self.array.len();
        let n = self.array[0].len();
        self.array = (0..n)
            .map(|i| (0..m).map(|j| self.array[j][i]).collect())
            .collect();
    }

    fn horizontal_flip(&mut self) {
        self.array.iter_mut().for_each(|arr| arr.reverse())
    }

    fn vertical_flip(&mut self) {
        self.array.reverse()
    }

    fn top(&self) -> Edge {
        let array = self.array[0].to_vec();
        Edge::from_array(array)
    }

    fn right(&self) -> Edge {
        let num_column = self.array[0].len();
        let array = self.array.iter().map(|arr| arr[num_column - 1]).collect();
        Edge::from_array(array)
    }

    fn bottom(&self) -> Edge {
        let array = self.array[self.array.len() - 1].to_vec();
        Edge::from_array(array)
    }

    fn left(&self) -> Edge {
        let array = self.array.iter().map(|arr| arr[0]).collect();
        Edge::from_array(array)
    }

    fn edges(&self) -> [Edge; 4] {
        [self.top(), self.right(), self.bottom(), self.left()]
    }

    fn remove_border(&mut self) {
        self.array.remove(0);
        self.array.pop();
        self.array.iter_mut().for_each(|arr| {
            arr.remove(0);
            arr.pop();
        })
    }

    fn orientate_until<F>(&mut self, criterion: F)
    where
        F: Fn(&Tile) -> bool,
    {
        for _ in 0..2 {
            self.transpose();
            if criterion(self) {
                return;
            }

            // v-flip
            self.vertical_flip();
            if criterion(self) {
                return;
            }

            // v-flip && h-flip
            self.horizontal_flip();
            if criterion(self) {
                return;
            }
            self.vertical_flip();

            // h-flip
            if criterion(self) {
                return;
            }
            self.horizontal_flip();
        }
        panic!("failed to find matching orientation");
    }

    fn orientations(&self) -> Vec<Self> {
        let mut t = self.clone();
        let mut r = Vec::new();
        for _ in 0..2 {
            t.transpose();
            r.push(t.clone());

            // v-flip
            t.vertical_flip();
            r.push(t.clone());

            // v-flip && h-flip
            t.horizontal_flip();
            r.push(t.clone());
            t.vertical_flip();

            // h-flip
            r.push(t.clone());
            t.horizontal_flip();
        }
        r
    }
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        if self.id != other.id {
            return false;
        }
        self.orientations().iter().any(|t| t.array == other.array)
    }
}

fn parse(content: &str) -> Vec<Tile> {
    let mut lines = content.lines();
    let mut tiles = Vec::new();
    let mut group = Vec::new();
    for line in &mut lines {
        if line.is_empty() {
            tiles.push(Tile::from_lines(&group));
            group.clear();
        } else {
            group.push(line);
        }
    }

    if !group.is_empty() {
        tiles.push(Tile::from_lines(&group));
    }

    tiles
}

fn rearrange(tiles: &[Tile], edges_map: HashMap<Edge, Vec<&Tile>>, corners: &[&Tile]) -> Tile {
    let n = (tiles.len() as f64).sqrt() as usize;
    let mut result = tiles.chunks(n).map(|c| c.to_vec()).collect::<Vec<_>>();

    // top-left
    let mut top_left = corners[0].clone();
    top_left.orientate_until(|tile| {
        edges_map[&tile.top()].len() == 1 && edges_map[&tile.left()].len() == 1
    });

    result[0][0] = top_left;

    // first row
    for j in 1..n {
        let left = &result[0][j - 1];
        let next_left = left.right();
        let mut next = edges_map[&next_left]
            .iter()
            .find(|t| t.id != left.id)
            .cloned()
            .unwrap()
            .to_owned();

        next.orientate_until(|tile| edges_map[&tile.top()].len() == 1 && tile.left() == next_left);

        result[0][j] = next;
    }

    // remaining rows
    for i in 1..n {
        for j in 0..n {
            let top = &result[i - 1][j];
            let left = match j {
                0 => None,
                j => Some(&result[i][j - 1]),
            };

            let top_edge = top.bottom();

            let mut next = edges_map[&top_edge]
                .iter()
                .find(|t| t.id != top.id)
                .cloned()
                .unwrap()
                .to_owned();

            next.orientate_until(|tile| {
                if let Some(left) = left {
                    tile.left() == left.right() && tile.top() == top_edge
                } else {
                    edges_map[&tile.left()].len() == 1 && tile.top() == top_edge
                }
            });

            result[i][j] = next;
        }
    }

    // combine into one Tile
    result.iter_mut().flatten().for_each(|t| t.remove_border());
    let w = result[0][0].array.len();
    let image = result
        .iter()
        .flat_map(|row| {
            (0..w)
                .map(|i| {
                    row.iter()
                        .flat_map(|c| c.array[i].to_vec())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    Tile {
        id: 0,
        array: image,
    }
}

fn part1(tiles: &[Tile]) -> (usize, Tile) {
    let mut edges_map: HashMap<Edge, Vec<&Tile>> = HashMap::new();
    tiles.iter().for_each(|t| {
        t.edges().iter().for_each(|e| {
            let entry = edges_map.entry(e.to_owned()).or_default();
            entry.push(t);
        });
    });

    let corners = tiles
        .iter()
        .filter(|&t| {
            t.edges()
                .iter()
                .filter(|e1| edges_map[e1].len() == 1)
                .count()
                == 2
        })
        .collect::<Vec<_>>();

    let image = rearrange(tiles, edges_map, &corners);

    let answer = corners.iter().map(|t| t.id).product();

    (answer, image)
}

fn locate_pattern(image: &Tile, pattern: &Tile) -> usize {
    let ih = image.array.len();
    let iw = image.array[0].len();
    let ph = pattern.array.len();
    let pw = pattern.array[0].len();

    let mut monster_cells: HashSet<(usize, usize)> = HashSet::new();

    for i in 0..=ih - ph {
        for j in 0..=iw - pw {
            let mut valid = true;
            let mut cells = HashSet::new();
            'search: for a in 0..ph {
                for b in 0..pw {
                    let c = pattern.array[a][b];
                    if c == Color::Black {
                        if image.array[i + a][j + b] == Color::White {
                            valid = false;
                            break 'search;
                        }
                        cells.insert((i + a, j + b));
                    }
                }
            }
            if valid {
                for c in cells {
                    monster_cells.insert(c);
                }
            }
        }
    }

    if !monster_cells.is_empty() {
        let mut detected = image.to_owned();
        for (i, j) in &monster_cells {
            detected.array[*i][*j] = Color::Monster;
        }
        // println!("{:?}", detected);
    }

    image
        .array
        .iter()
        .flatten()
        .filter(|&c| *c == Color::Black)
        .count()
        - monster_cells.len()
}

fn part2(image: &Tile) -> usize {
    let lines = SEA_MONSTER.lines().collect::<Vec<_>>();
    let sea_monster = Tile::from_lines(&lines);

    sea_monster
        .orientations()
        .iter()
        .map(|p| locate_pattern(image, p))
        .min()
        .unwrap_or(0)
}

pub fn main() {
    let content = std::fs::read_to_string("data/2020/day20").unwrap();
    let tiles = parse(&content);
    // part 1
    let (answer, image) = part1(&tiles);
    println!("day 20 part1: {}", answer);

    // part 2
    println!("day 20 part2: {}", part2(&image));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_input1() {
        let content = std::fs::read_to_string("data/2020/day20-1").unwrap();
        let image_str = std::fs::read_to_string("data/2020/day20-1-1").unwrap();
        let tiles = parse(&content);
        let expected_image = Tile::from_str(&image_str);
        let (answer, image) = part1(&tiles);
        assert_eq!(20899048083289, answer);
        assert_eq!(image, expected_image);
        assert_eq!(273, part2(&image));
    }

    #[test]
    fn tile_transpose() {
        let original = Tile::from_str(
            "Tile 1:
..#
.#.
#.#
###",
        );

        let mut copy = original.to_owned();
        copy.transpose();

        let transposed = Tile::from_str(
            "Tile 1:
..##
.#.#
#.##",
        );

        assert_eq!(transposed.array, copy.array);

        copy.transpose();
        assert_eq!(original.array, copy.array);
    }

    #[test]
    fn tile_flip() {
        let original = Tile::from_str(
            "Tile 1:
..#
.#.
###",
        );

        let mut copy = original.to_owned();
        copy.vertical_flip();

        let expected = Tile::from_str(
            "Tile 1:
###
.#.
..#",
        );

        assert_eq!(expected.array, copy.array);

        let mut copy = original.to_owned();
        copy.horizontal_flip();

        let expected = Tile::from_str(
            "Tile 1:
#..
.#.
###",
        );

        assert_eq!(expected.array, copy.array);
    }

    #[test]
    fn tile_remove_border() {
        let mut original = Tile::from_str(
            "Tile 1:
..##
.#.#
####
####",
        );

        original.remove_border();

        let expected = Tile::from_str(
            "Tile 1:
#.
##",
        );

        assert_eq!(original.array, expected.array);
    }

    #[test]
    fn combine_tiles() {
        let tiles_str = "Tile 1:
..##
..##
####
####

Tile 2:
..##
.#.#
####
####

Tile 3:
..##
.###
#.##
####

Tile 4:
..##
.###
##.#
####";
        let tiles = parse(tiles_str);
        let n = (tiles.len() as f64).sqrt().round() as usize;
        let mut tiles = tiles.chunks(n).map(|c| c.to_vec()).collect::<Vec<_>>();

        tiles.iter_mut().flatten().for_each(|t| t.remove_border());
        let w = tiles[0][0].array.len();
        let image = tiles
            .iter()
            .flat_map(|row| {
                (0..w)
                    .map(|i| {
                        row.iter()
                            .flat_map(|c| c.array[i].to_vec())
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let expected = Tile::from_str(
            "Tile 0:
.##.
####
####
.##.",
        );

        assert_eq!(image, expected.array);
    }
}
