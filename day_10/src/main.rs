use common::itertools::Itertools;
use std::collections::HashMap;

static INPUT: &str = include_str!("input");

fn main() -> Result<(), String> {
    let adapters = INPUT
        .lines()
        .map(|line| {
            line.parse::<i32>()
                .map_err(|_| format!("Not a valid number: {}", line))
        })
        .sorted()
        .collect::<Result<Vec<i32>, String>>()?;

    println!("Part 1 result: {}", part_1(&adapters));

    part_2(&adapters).iter().for_each(|res_2| {
        println!("Part 2 result: {}", res_2);
    });

    Ok(())
}

fn part_1(adapters: &[i32]) -> i32 {
    let mut diff_1 = 0;
    let mut diff_3 = 1; // including last adapter to laptop
    adapters.iter().fold(0, |acc, adapter| {
        if adapter - acc == 1 {
            diff_1 += 1;
        } else if adapter - acc == 3 {
            diff_3 += 1;
        }
        *adapter
    });
    diff_1 * diff_3
}

fn part_2(adapters: &[i32]) -> Option<i64> {
    let mut ways: HashMap<i32, i64> = HashMap::new();
    ways.insert(0, 1);
    for a in adapters.iter() {
        let n_ways = ways.get(&(a - 1)).copied().unwrap_or(0)
            + ways.get(&(a - 2)).copied().unwrap_or(0)
            + ways.get(&(a - 3)).copied().unwrap_or(0);
        ways.insert(*a, n_ways);
    }
    adapters.last().and_then(|last| ways.get(last).copied())
}

#[cfg(test)]
mod tests {
    use super::*;

    static SHORT: [i32; 11] = [1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19];

    static LONG: [i32; 31] = [
        1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31, 32, 33, 34, 35, 38,
        39, 42, 45, 46, 47, 48, 49,
    ];

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&SHORT), 7 * 5);
        assert_eq!(part_1(&LONG), 22 * 10);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&SHORT), Some(8));
        assert_eq!(part_2(&LONG), Some(19208));
    }
}
