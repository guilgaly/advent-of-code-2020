mod actions;
mod part_1;
mod part_2;

use crate::actions::Action;
use crate::part_1::part_1;
use crate::part_2::part_2;

static INPUT: &str = include_str!("input");

fn main() -> Result<(), String> {
    let actions = INPUT
        .lines()
        .map(|line| line.parse::<Action>())
        .collect::<Result<Vec<Action>, String>>()?;

    println!("Part 1 result: {}", part_1(&actions));

    println!("Part 2 result: {}", part_2(&actions));

    Ok(())
}
