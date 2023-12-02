struct Game {
    id: u32,
    subsets: Vec<[u32; 3]>,
}

fn parse(data: &str) -> Vec<Game> {
    data.trim()
        .lines()
        .map(|line| {
            let (p1, p2) = line.split_once(": ").unwrap();
            let id: u32 = p1.strip_prefix("Game ").unwrap().parse().unwrap();
            let subsets = p2
                .split("; ")
                .map(|parts| {
                    let mut set = [0; 3];
                    parts.split(", ").for_each(|s| {
                        let (quantity, color) = s.split_once(' ').unwrap();
                        let quantity: u32 = quantity.parse().unwrap();
                        match color {
                            "red" => set[0] = quantity,
                            "green" => set[1] = quantity,
                            "blue" => set[2] = quantity,
                            c => panic!("unknown color {c}"),
                        };
                    });
                    set
                })
                .collect();
            Game { id, subsets }
        })
        .collect()
}

fn part1(input: &[Game]) -> u32 {
    const MAX: [u32; 3] = [12, 13, 14];
    input
        .iter()
        .map(|game| {
            if game
                .subsets
                .iter()
                .all(|set: &[u32; 3]| set.iter().zip(&MAX).all(|(s, m)| s <= m))
            {
                game.id
            } else {
                0
            }
        })
        .sum()
}

fn part2(input: &[Game]) -> u32 {
    input
        .iter()
        .map(|game| {
            let mut min = [0; 3];
            game.subsets.iter().for_each(|g| {
                min.iter_mut().zip(g).for_each(|(m, &g)| *m = (*m).max(g));
            });
            min.into_iter().product::<u32>()
        })
        .sum()
}

pub fn main() {
    let data = std::fs::read_to_string("data/2023/day2").unwrap();
    let input = parse(&data);
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let input = parse(data);
        assert_eq!(8, part1(&input));
        assert_eq!(2286, part2(&input));
    }
}
