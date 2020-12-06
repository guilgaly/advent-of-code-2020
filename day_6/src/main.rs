use itertools::Itertools;
use reduce::Reduce;
use std::collections::HashSet;

static INPUT: &str = include_str!("input");

fn main() {
    let answers_count: usize = INPUT
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .flat_map(|line| line.chars())
                .sorted()
                .dedup()
                .count()
        })
        .sum();
    println!("Part 1 response: {}", answers_count);

    let answers_count_2: usize = INPUT
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|line| line.chars().collect::<HashSet<char>>())
                .reduce(|acc, next| acc.intersection(&next).copied().collect())
                .map(|answers| answers.len())
                .unwrap_or_else(|| 0)
        })
        .sum();
    println!("Part 2 response: {}", answers_count_2);
}
