use ahash::AHashMap as HashMap;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, char as ch, digit1, space1},
    combinator::{map_res, opt, recognize, rest},
    sequence::tuple,
    IResult,
};

fn number(input: &str) -> IResult<&str, usize> {
    map_res(recognize(digit1), str::parse)(input)
}

fn color(input: &str) -> IResult<&str, String> {
    let (r, (a, b, c)) = tuple((alpha1, space1, alpha1))(input)?;
    Ok((r, format!("{}{}{}", a, b, c)))
}

fn split(bags: &str) -> Vec<(usize, String)> {
    if bags == "no other bags." {
        return Vec::new();
    }

    bags.split(", ")
        .map(|s| {
            let (_, (num, _, c, _, _)) =
                tuple((number, ch(' '), color, tag(" bag"), opt(ch('s'))))(s).unwrap();
            (num, c)
        })
        .collect()
}

fn contains_shiny_gold(
    bag: &str,
    map: &HashMap<String, Vec<(usize, String)>>,
    table: &mut HashMap<String, bool>,
) -> bool {
    if bag == "shiny gold" {
        return true;
    }
    if let Some(&r) = table.get(bag) {
        return r;
    }
    for (_, node) in map
        .get(bag)
        .unwrap_or_else(|| panic!("map doesn't contain: {}", bag))
    {
        let r = match table.get(node) {
            Some(&v) => v,
            None => contains_shiny_gold(node, map, table),
        };

        if r {
            table.insert(bag.to_string(), true);
            return true;
        }
    }
    table.insert(bag.to_string(), false);
    false
}

fn count_contained_bags(
    bag: &str,
    map: &HashMap<String, Vec<(usize, String)>>,
    table: &mut HashMap<String, usize>,
) -> usize {
    let n = map
        .get(bag)
        .unwrap_or_else(|| panic!("map doesn't contain: {}", bag))
        .iter()
        .map(|(n, node)| {
            n * match table.get(node) {
                Some(&v) => v,
                None => count_contained_bags(node, map, table),
            }
        })
        .sum::<usize>()
        + 1;
    table.insert(bag.to_string(), n);
    n
}

pub fn main() {
    let mut map = HashMap::new();
    let data = std::fs::read_to_string("data/2020/day7").unwrap();
    for line in data.lines().filter(|line| !line.is_empty()) {
        let (_, (c, _, r)) = tuple((color, tag(" bags contain "), rest))(line).unwrap();
        let others = split(r);
        map.insert(c, others);
    }

    // part 1
    let mut table = HashMap::new();
    let n = map
        .keys()
        .filter(|&k| k != "shiny gold" && contains_shiny_gold(k, &map, &mut table))
        .count();
    println!("day7 part1: {}", n);

    // part 2
    let mut table = HashMap::new();
    let n = count_contained_bags("shiny gold", &map, &mut table) - 1;
    println!("day7 part2: {}", n);
}
