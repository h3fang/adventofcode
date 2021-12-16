use ahash::AHashMap as HashMap;
use ahash::AHashSet as HashSet;
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct Ingredient {
    name: String,
}

fn parse(content: &str) -> Vec<(HashSet<&str>, HashSet<&str>)> {
    content
        .lines()
        .map(|line| {
            let parts = line.split(" (contains ").collect::<Vec<_>>();
            let ingredients = parts[0].split(' ').collect();
            let allergens = parts[1][..parts[1].len() - 1].split(", ").collect();
            (ingredients, allergens)
        })
        .collect()
}

fn part1<'a>(
    data: &[(HashSet<&'a str>, HashSet<&'a str>)],
) -> (usize, HashMap<&'a str, HashSet<&'a str>>) {
    // map: allergen -> set(ingredient)
    let mut map: HashMap<&str, HashSet<&str>> = HashMap::new();
    // find candidates for allergen
    data.iter().for_each(|(ingredients, allergens)| {
        for &allergen in allergens {
            let entry = map.entry(allergen).or_insert_with(|| ingredients.clone());
            *entry = entry.intersection(ingredients).cloned().collect();
        }
    });

    // find out relationship between allergen and ingredient
    let mut done = HashSet::new();
    loop {
        map.iter_mut().for_each(|(_, ingredients)| {
            if ingredients.len() > 1 {
                *ingredients = ingredients
                    .iter()
                    .filter(|&ing| !done.contains(ing))
                    .cloned()
                    .collect();
            } else {
                done.insert(ingredients.iter().next().unwrap().to_owned());
            }
        });

        if map.values().all(|v| v.len() == 1) {
            break;
        }
    }

    // answer
    let ingredients: Vec<&str> = data.iter().flat_map(|(i, _)| i).cloned().collect();
    let all = ingredients.iter().cloned().collect::<HashSet<_>>();
    let bad = map.values().flat_map(|v| v.iter()).cloned().collect();
    let safe = all.difference(&bad).collect::<HashSet<_>>();
    let n = ingredients.iter().filter(|i| safe.contains(i)).count();
    (n, map)
}

fn part2(dangerous: &HashMap<&str, HashSet<&str>>) -> String {
    let dangerous = dangerous
        .iter()
        .map(|(k, v)| (*k, v.iter().next().unwrap().to_owned()))
        .collect::<BTreeMap<&str, &str>>();
    dangerous.values().cloned().collect::<Vec<_>>().join(",")
}

pub fn main() {
    let content = std::fs::read_to_string("data/2020/day21").unwrap();
    let data = parse(&content);
    // part 1
    let (n, dangerous) = part1(&data);
    println!("day 21 part1: {}", n);

    // part 2
    println!("day 21 part1: {}", part2(&dangerous));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let content = std::fs::read_to_string("data/2020/day21-1").unwrap();
        let data = parse(&content);
        assert_eq!(5, part1(&data).0);
    }
}
