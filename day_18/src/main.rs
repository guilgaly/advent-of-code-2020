mod parser_generator;

use crate::parser_generator::{eval_flat, eval_reversed};

static INPUT: &str = include_str!("input");

fn main() -> Result<(), String> {
    exec("Part 1", INPUT, |line| eval_flat(line))?;
    exec("Part 2", INPUT, |line| eval_reversed(line))?;

    Ok(())
}

fn exec(
    name: &str,
    input: &str,
    eval_line: impl Fn(&str) -> Result<usize, String>,
) -> Result<(), String> {
    let result = input
        .lines()
        .map(|line| eval_line(line))
        .sum::<Result<usize, String>>()?;
    println!("{} result: {}", name, result);
    Ok(())
}
