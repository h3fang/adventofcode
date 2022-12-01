use std::str::FromStr;

use ahash::AHashMap as HashMap;

#[derive(Debug, PartialEq, Eq)]
enum Event {
    BeginShfit(u16),
    WakeUp,
    FallAsleep,
}

#[derive(Debug)]
struct Record {
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    event: Event,
}

#[derive(Debug)]
struct InvalidRecord;

impl FromStr for Record {
    type Err = InvalidRecord;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (datetime, event) = s.split_once("] ").ok_or(InvalidRecord)?;
        let (md, hm) = datetime[1..].split_once(' ').ok_or(InvalidRecord)?;
        let (month, day) = md[5..].split_once('-').ok_or(InvalidRecord)?;
        let (hour, minute) = hm.split_once(':').ok_or(InvalidRecord)?;
        let month = month.parse().map_err(|_| InvalidRecord)?;
        let day = day.parse().map_err(|_| InvalidRecord)?;
        let hour = hour.parse().map_err(|_| InvalidRecord)?;
        let minute = minute.parse().map_err(|_| InvalidRecord)?;
        let event = match event {
            "falls asleep" => Event::FallAsleep,
            "wakes up" => Event::WakeUp,
            e => {
                let id = e.split_ascii_whitespace().nth(1).ok_or(InvalidRecord)?;
                let id = id[1..].parse().map_err(|_| InvalidRecord)?;
                Event::BeginShfit(id)
            }
        };
        Ok(Self {
            month,
            day,
            hour,
            minute,
            event,
        })
    }
}

fn build_table(mut records: Vec<Record>) -> HashMap<u16, [i32; 60]> {
    records.sort_unstable_by_key(|r| (r.month, r.day, r.hour, r.minute));
    let mut table = HashMap::new();
    let mut current = 0;
    for (i, r) in records.iter().enumerate() {
        match r.event {
            Event::BeginShfit(n) => {
                current = n;
                table.entry(current).or_insert([0; 60]);
            }
            Event::WakeUp => {}
            Event::FallAsleep => {
                let e = table.entry(current).or_insert([0; 60]);
                if r.hour != 0
                    || records[i + 1].hour != 0
                    || r.minute >= records[i + 1].minute
                    || records[i + 1].event != Event::WakeUp
                {
                    println!(
                        "{}:{}, {}:{}, {:?}",
                        r.hour,
                        r.minute,
                        records[i + 1].hour,
                        records[i + 1].minute,
                        records[i + 1].event
                    );
                }
                for j in r.minute..records[i + 1].minute {
                    e[j as usize] += 1;
                }
            }
        }
    }
    table
}

fn part1(table: &HashMap<u16, [i32; 60]>) -> usize {
    let id = *table
        .iter()
        .max_by_key(|(_, v)| v.iter().sum::<i32>())
        .unwrap()
        .0;
    let minute = table
        .get(&id)
        .unwrap()
        .iter()
        .enumerate()
        .max_by_key(|e| e.1)
        .unwrap()
        .0;
    id as usize * minute
}

fn part2(table: &HashMap<u16, [i32; 60]>) -> usize {
    let id = table
        .iter()
        .map(|(&id, t)| (id, t.iter().enumerate().max_by_key(|e| e.1).unwrap()))
        .max_by_key(|e| e.1 .1)
        .unwrap();
    id.0 as usize * id.1 .0
}

pub fn main() {
    let data = std::fs::read_to_string("data/2018/day4").unwrap();
    let records: Vec<Record> = data.lines().map(|line| line.parse().unwrap()).collect();
    let table = build_table(records);
    println!("part1: {}", part1(&table));
    println!("part2: {}", part2(&table));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let data = "[1518-11-01 00:00] Guard #10 begins shift\n[1518-11-01 00:05] falls asleep\n[1518-11-01 00:25] wakes up\n[1518-11-01 00:30] falls asleep\n[1518-11-01 00:55] wakes up\n[1518-11-01 23:58] Guard #99 begins shift\n[1518-11-02 00:40] falls asleep\n[1518-11-02 00:50] wakes up\n[1518-11-03 00:05] Guard #10 begins shift\n[1518-11-03 00:24] falls asleep\n[1518-11-03 00:29] wakes up\n[1518-11-04 00:02] Guard #99 begins shift\n[1518-11-04 00:36] falls asleep\n[1518-11-04 00:46] wakes up\n[1518-11-05 00:03] Guard #99 begins shift\n[1518-11-05 00:45] falls asleep\n[1518-11-05 00:55] wakes up".to_string();
        let records: Vec<Record> = data.lines().map(|line| line.parse().unwrap()).collect();
        let table = build_table(records);
        assert_eq!(240, part1(&table));
        assert_eq!(4455, part2(&table));
    }
}
