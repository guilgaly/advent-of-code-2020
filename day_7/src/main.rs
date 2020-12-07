use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};

static INPUT: &str = include_str!("input");

type BagRules = HashMap<String, HashMap<String, i32>>;

fn main() -> Result<(), String> {
    let rules = INPUT
        .lines()
        .map(|line| parse_rule(line))
        .collect::<Result<BagRules, String>>()?;
    println!("Found {} rules", rules.len());

    let res = find_containers_for_color("shiny gold", &rules);
    println!("Part 1 result: {}", res.len());

    let count = count_bags("shiny gold", &rules);
    println!("Part 2 result: {}", count - 1); // -1 to avoid counting the shiny gold bag itself

    Ok(())
}

fn find_containers_for_color<'a>(color: &str, rules: &'a BagRules) -> HashSet<&'a String> {
    let res: HashSet<&String> = rules
        .iter()
        .filter_map(|(container, can_contain)| {
            if can_contain.contains_key(color) {
                Some(container)
            } else {
                None
            }
        })
        .collect();
    if res.is_empty() {
        res
    } else {
        res.iter().fold(res.clone(), |acc, next| {
            acc.union(&find_containers_for_color(next, rules))
                .copied()
                .collect()
        })
    }
}

fn count_bags(color: &str, rules: &BagRules) -> i32 {
    rules
        .get(color)
        .map(|contains| {
            1 + contains.iter().fold(0, |acc, (color, count)| {
                acc + count * count_bags(color, rules)
            })
        })
        .unwrap_or(0)
}

fn parse_rule(line: &str) -> Result<(String, HashMap<String, i32>), String> {
    lazy_static! {
        static ref REGEX_1: Regex =
            Regex::new(r"^([a-z ]+) bags contain (no other bags|([0-9]+ [a-z ]+(, )?)+)\.$")
                .unwrap();
        static ref REGEX_2: Regex = Regex::new(r"^([0-9]+) ([a-z ]+) bags?$").unwrap();
    }

    REGEX_1
        .captures(line)
        .and_then(|cap| {
            let color = cap.get(1)?.as_str().to_owned();
            let contains = cap
                .get(2)?
                .as_str()
                .split(", ")
                .flat_map(|contained_bags_str| {
                    REGEX_2.captures(contained_bags_str).and_then(|cap| {
                        let count = cap.get(1)?.as_str().parse::<i32>().ok()?;
                        let color = cap.get(2)?.as_str().to_owned();
                        Some((color, count))
                    })
                })
                .collect::<HashMap<String, i32>>();
            Some((color, contains))
        })
        .ok_or(format!("Failed to parse line {}", line))
}
