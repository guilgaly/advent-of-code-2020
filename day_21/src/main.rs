use common::itertools::{all, Itertools};
use common::lazy_static::lazy_static;
use common::reduce::Reduce;
use common::regex::Regex;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

static INPUT: &str = include_str!("input");

fn main() -> Result<(), String> {
    let foods = parse_foods(INPUT)?;
    println!("Found {} foods", foods.len());

    let all_ingredients: HashSet<&String> = foods.iter().flat_map(|f| &f.ingredients).collect();
    let all_allergens: HashSet<&String> = foods.iter().flat_map(|f| &f.allergens).collect();
    println!("all_ingredients: {}", all_ingredients.len());
    println!("all_allergens: {}", all_allergens.len());

    let mut potential_ingredients_by_allergen: HashMap<&String, Vec<&String>> = all_allergens
        .iter()
        .map(|allergen| {
            let foods = foods
                .iter()
                .filter(|f| f.allergens.contains(allergen))
                .collect::<Vec<_>>();
            let potential_ingredients = all_ingredients
                .iter()
                .filter(|ingredient| foods.iter().all(|f| f.ingredients.contains(ingredient)))
                .copied()
                .collect::<Vec<&String>>();
            (*allergen, potential_ingredients)
        })
        .collect();

    let mut ingredients_by_allergen: Vec<(&String, &String)> = vec![];
    while !potential_ingredients_by_allergen.is_empty() {
        let (allergen, ingredient) = &potential_ingredients_by_allergen
            .iter()
            .find_map(|(a, i)| if i.len() == 1 { Some((a, i[0])) } else { None })
            .unwrap_or_else(|| panic!());
        ingredients_by_allergen.push((*allergen, ingredient));
        potential_ingredients_by_allergen.remove(**allergen);
        potential_ingredients_by_allergen
            .values_mut()
            .for_each(|potential_ingredients| {
                potential_ingredients.retain(|i| i != ingredient);
            });
    }
    let ingredients_by_allergen = ingredients_by_allergen;

    for (allergen, ingredient) in &ingredients_by_allergen {
        println!("{}: {}", allergen, ingredient);
    }

    let non_allergenic_ingredients: Vec<&String> = all_ingredients
        .iter()
        .filter(|ingredient| {
            ingredients_by_allergen
                .iter()
                .find(|(_, i)| *ingredient == i)
                .is_none()
        })
        .copied()
        .collect();
    println!(
        "non_allergenic_ingredients: {}",
        non_allergenic_ingredients.len()
    );
    let res_1 = foods
        .iter()
        .flat_map(|f| f.ingredients.iter())
        .filter(|i| non_allergenic_ingredients.contains(i))
        .count();
    println!("Part 1 result: {}", res_1);

    let canonical_list = ingredients_by_allergen
        .iter()
        .sorted_by_key(|(a, i)| a)
        .map(|(a, i)| i)
        .join(",");
    println!("Part 2 result: {}", canonical_list);

    Ok(())
}

fn parse_foods(input: &str) -> Result<Vec<Food>, String> {
    input.lines().map(|line| line.parse::<Food>()).collect()
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Food {
    ingredients: Vec<String>,
    allergens: Vec<String>,
}

impl FromStr for Food {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref REGEX: Regex = Regex::new(r"^([a-z ]+)\(contains ([a-z ,]+)\)$").unwrap();
        }
        REGEX
            .captures(s)
            .and_then(|cap| {
                let ingredients: Vec<String> = cap
                    .get(1)?
                    .as_str()
                    .trim()
                    .split(' ')
                    .map(|s| s.to_owned())
                    .collect();
                let allergens: Vec<String> = cap
                    .get(2)?
                    .as_str()
                    .trim()
                    .split(", ")
                    .map(|s| s.to_owned())
                    .collect();
                Some(Food {
                    ingredients,
                    allergens,
                })
            })
            .ok_or_else(|| format!("Failed to parse: {}", s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
