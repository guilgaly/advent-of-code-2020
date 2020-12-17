mod part_1;
mod part_2;

use crate::part_1::part_1;
use crate::part_2::part_2;

static INPUT: &str = include_str!("input");

fn main() -> Result<(), String> {
    println!("Part 1 result: {}", part_1(INPUT)?);

    println!("Part 2 result: {}", part_2(INPUT)?);

    Ok(())
}
