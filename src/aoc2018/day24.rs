use ahash::HashMap;

#[derive(Clone)]
struct Group {
    units: i64,
    hp: i64,
    attack: i64,
    damage_type: u8,
    immune: u8,
    weak: u8,
    initiative: i8,
}

impl Group {
    fn effective_power(&self) -> i64 {
        self.units * self.attack
    }

    fn deal_damage(&self, enemy: &Group) -> i64 {
        let multiplier = if self.damage_type & enemy.immune > 0 {
            0
        } else if self.damage_type & enemy.weak > 0 {
            2
        } else {
            1
        };
        multiplier * self.effective_power()
    }

    fn select_target(&self, enemy: &Army, selected: &mut [bool]) -> Option<usize> {
        let t = enemy
            .groups
            .iter()
            .enumerate()
            .filter(|(i, _)| !selected[*i])
            .map(|(i, enemy)| {
                (
                    self.deal_damage(enemy),
                    enemy.effective_power(),
                    enemy.initiative,
                    i,
                )
            })
            .max();
        if let Some((dmg, _, _, t)) = t
            && dmg > 0
        {
            selected[t] = true;
            return Some(t);
        }
        None
    }

    fn attack(&self, enemy: &mut Group) -> i64 {
        let dmg = self.deal_damage(enemy);
        let n = (enemy.units - dmg / enemy.hp).max(0);
        let killed = enemy.units - n;
        enemy.units = n;
        killed
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum System {
    Immune,
    Infection,
}

#[derive(Clone)]
struct Army {
    system: System,
    groups: Vec<Group>,
}

impl Army {
    fn select_targets(&self, enemy: &Army) -> Vec<(i8, System, usize, usize)> {
        let mut ids: Vec<(i64, i8, usize)> = self
            .groups
            .iter()
            .enumerate()
            .map(|(i, g)| (-g.effective_power(), -g.initiative, i))
            .collect();
        ids.sort_unstable();
        let mut selected = vec![false; enemy.groups.len()];
        ids.into_iter()
            .filter_map(|(_, _, i)| {
                self.groups[i]
                    .select_target(enemy, &mut selected)
                    .map(|j| (-self.groups[i].initiative, self.system, i, j))
            })
            .collect()
    }
}

fn parse_group<'a>(line: &'a str, damage_types: &mut HashMap<&'a str, usize>) -> Group {
    let (count, line) = line.split_once(" units each with ").unwrap();
    let count = count.parse().unwrap();
    let (hp, mut line) = line.split_once(" hit points ").unwrap();
    let hp = hp.parse().unwrap();

    let (mut immune, mut weak) = (0, 0);
    if line.starts_with('(') {
        let (special, remain) = line
            .trim_start_matches('(')
            .split_once(") with an attack that does ")
            .unwrap();
        line = remain;

        for p in special.split("; ") {
            let (tag, list) = p.split_once(" to ").unwrap();
            let m = if tag == "immune" {
                &mut immune
            } else {
                &mut weak
            };
            for t in list.split(", ") {
                let len = damage_types.len();
                let e = damage_types.entry(t).or_insert(len);
                *m |= 1 << *e;
            }
        }
    } else {
        line = line.trim_start_matches("with an attack that does ");
    }

    let (attack, initiative) = line.split_once(" damage at initiative ").unwrap();
    let (attack, type_) = attack.split_once(" ").unwrap();
    let attack = attack.parse().unwrap();
    let len = damage_types.len();
    let damage_type = *damage_types.entry(type_).or_insert(len);
    let damage_type = 1 << damage_type;
    let initiative = initiative.parse().unwrap();
    Group {
        units: count,
        hp,
        attack,
        damage_type,
        immune,
        weak,
        initiative,
    }
}

fn parse_groups<'a>(lines: &'a str, damage_types: &mut HashMap<&'a str, usize>) -> Army {
    let mut lines = lines.lines();
    let system = lines.next().unwrap();
    let system = match system {
        "Immune System:" => System::Immune,
        "Infection:" => System::Infection,
        _ => unreachable!(),
    };
    let groups = lines.map(|line| parse_group(line, damage_types)).collect();
    Army { system, groups }
}

fn parse(data: &str) -> (Army, Army) {
    let (a, b) = data.trim().split_once("\n\n").unwrap();
    let mut damage_types = HashMap::default();
    (
        parse_groups(a, &mut damage_types),
        parse_groups(b, &mut damage_types),
    )
}

fn fight(mut immune: Army, mut infection: Army) -> (i64, i64) {
    while !immune.groups.is_empty() && !infection.groups.is_empty() {
        let mut targets = immune.select_targets(&infection);
        targets.extend(infection.select_targets(&immune));
        targets.sort_unstable();

        let mut killed = 0;
        for (_, system, i, t) in targets {
            match system {
                System::Immune => killed += immune.groups[i].attack(&mut infection.groups[t]),
                System::Infection => killed += infection.groups[i].attack(&mut immune.groups[t]),
            }
        }
        if killed == 0 {
            break;
        }
        immune.groups.retain(|g| g.units > 0);
        infection.groups.retain(|g| g.units > 0);
    }
    let a = immune.groups.iter().map(|g| g.units).sum();
    let b = infection.groups.iter().map(|g| g.units).sum();
    (a, b)
}

fn part1(immune: Army, infection: Army) -> i64 {
    let (a, b) = fight(immune, infection);
    a + b
}

fn part2(immune: Army, infection: Army) -> i64 {
    let (mut l, mut r, mut result) = (1, i32::MAX as i64, 0);
    while l <= r {
        let m = (l + r) / 2;
        let (mut imm, inf) = (immune.clone(), infection.clone());
        imm.groups.iter_mut().for_each(|g| g.attack += m);
        let (a, b) = fight(imm, inf);
        if a > 0 && b == 0 {
            result = a;
            r = m - 1;
        } else {
            l = m + 1;
        }
    }
    result
}

pub fn main() {
    let data = std::fs::read_to_string("data/2018/day24").unwrap();
    let (immune, infection) = parse(&data);
    println!("part1: {}", part1(immune.clone(), infection.clone()));
    println!("part2: {}", part2(immune, infection));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "
Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4";
        let (immune, infection) = parse(data);
        assert_eq!(5216, part1(immune.clone(), infection.clone()));
        assert_eq!(51, part2(immune, infection));
    }

    #[test]
    fn case2() {
        let data = std::fs::read_to_string("data/2018/day24").unwrap();
        let (immune, infection) = parse(&data);
        assert_eq!(9328, part1(immune.clone(), infection.clone()));
        assert_eq!(2172, part2(immune, infection));
    }
}
