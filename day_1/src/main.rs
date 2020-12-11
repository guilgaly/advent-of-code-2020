use common::itertools::Itertools;
use std::error::Error;
use std::num::ParseIntError;

static INPUT: &str = include_str!("input");

fn main() -> Result<(), Box<dyn Error>> {
    let expenses: Vec<i32> = parse_expenses(INPUT)?;

    let execute = |name: &str, f: fn(&[i32]) -> Option<i32>| {
        let maybe_result = common::time_execution(name, || f(&expenses));
        match maybe_result {
            Some(result) => println!("{} result: {}", name, result),
            None => println!("{} result not found", name),
        };
    };

    // Simple implementation
    execute("Part 1", part_1);
    execute("Part 2", part_2);

    // Alternate implementation using itertools::structs::Combinations ;
    // simple and flexible but a lot less performant because it allocates a
    // bunch of vectors.
    execute("Part 1 (alt)", |exp| part_n_alt(exp, 2));
    execute("Part 2 (alt)", |exp| part_n_alt(exp, 3));

    Ok(())
}

fn parse_expenses(input: &str) -> Result<Vec<i32>, ParseIntError> {
    input.lines().map(|line| line.parse::<i32>()).collect()
}

fn part_1(expenses: &[i32]) -> Option<i32> {
    expenses.iter().enumerate().find_map(|(i, x)| {
        expenses
            .split_at(i)
            .1
            .iter()
            .find_map(|y| if x + y == 2020 { Some(x * y) } else { None })
    })
}

fn part_2(expenses: &[i32]) -> Option<i32> {
    expenses.iter().enumerate().find_map(|(i, x)| {
        expenses
            .split_at(i)
            .1
            .iter()
            .enumerate()
            .find_map(|(j, y)| {
                expenses.split_at(j).1.iter().find_map(|z| {
                    if x + y + z == 2020 {
                        Some(x * y * z)
                    } else {
                        None
                    }
                })
            })
    })
}

fn part_n_alt(expenses: &[i32], n: usize) -> Option<i32> {
    expenses.iter().combinations(n).find_map(|combination| {
        if combination.iter().copied().sum::<i32>() == 2020 {
            Some(combination.iter().copied().product())
        } else {
            None
        }
    })
}
