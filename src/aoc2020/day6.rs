use hashbrown::HashSet;

fn count(lines: &[&str]) -> usize {
    let chars = lines
        .iter()
        .flat_map(|line| line.chars())
        .collect::<HashSet<_>>();
    chars.len()
}

fn count_part2(lines: &[&str]) -> usize {
    let shortest = lines.iter().map(|line| line.len()).min().unwrap();
    let shortest = lines.iter().find(|line| line.len() == shortest).unwrap();
    let chars = lines
        .iter()
        .map(|s| s.chars().collect::<HashSet<_>>())
        .collect::<Vec<_>>();
    shortest
        .chars()
        .filter(|c| chars.iter().all(|ch| ch.contains(c)))
        .count()
}

pub fn main() {
    let mut lines = Vec::new();
    let mut n = 0;
    let mut n_part2 = 0;
    let content = std::fs::read_to_string("data/2020/day6").unwrap();
    for line in content.lines() {
        if line.is_empty() {
            n += count(&lines);
            n_part2 += count_part2(&lines);
            lines.clear();
        } else {
            lines.push(line);
        }
    }
    if !lines.is_empty() {
        n += count(&lines);
        n_part2 += count_part2(&lines);
    }
    println!("day6 part1: {}\nday6 part2: {}", n, n_part2);
}
