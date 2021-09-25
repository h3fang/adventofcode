mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

use anyhow::Result;

fn main() -> Result<()> {
    day1::main("data/day1")?;
    day2::main("data/day2")?;
    day3::main("data/day3")?;
    day4::main("data/day4")?;
    day5::main("data/day5")?;
    day6::main("data/day6")?;
    day7::main("data/day7")?;
    day8::main("data/day8")?;
    day9::main("data/day9")?;

    Ok(())
}
