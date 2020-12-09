use common::itertools::Itertools;

static INPUT: &str = include_str!("input");

fn main() -> Result<(), String> {
    let numbers = INPUT
        .lines()
        .map(|line| {
            line.parse::<i64>()
                .map_err(|_| format!("{} is not a valid number", line))
        })
        .collect::<Result<Vec<i64>, String>>()?;

    match find_invalid_number(&numbers, 25) {
        None => println!("Part 1 - invalid number not found"),
        Some(number) => {
            println!("Part 1 result: {}", number);
            match find_contiguous_sum(&numbers, number) {
                None => println!("Part 2 - contiguous sum not found"),
                Some(v) => println!("Part 2 result: {}", v),
            }
        }
    }

    Ok(())
}

fn find_invalid_number(numbers: &[i64], set_size: usize) -> Option<i64> {
    numbers.windows(set_size + 1).find_map(|window| {
        let sum_found = window
            .iter()
            .take(set_size)
            .combinations(2)
            .map(|combination| combination.iter().copied().sum::<i64>())
            .find(|sum| *sum == window[set_size]);
        match sum_found {
            Some(_) => None,
            None => Some(window[set_size]),
        }
    })
}

fn find_contiguous_sum(numbers: &[i64], number: i64) -> Option<i64> {
    (2..numbers.len()).find_map(|range| {
        numbers.windows(range).find_map(|window| {
            if window.iter().sum::<i64>() == number {
                let smallest = window.iter().min()?;
                let largest = window.iter().max()?;
                Some(smallest + largest)
            } else {
                None
            }
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_NUMBERS: [i64; 20] = [
        35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576,
    ];

    #[test]
    fn test_find_invalid_number() {
        assert_eq!(find_invalid_number(&TEST_NUMBERS, 5), Some(127));
    }

    #[test]
    fn test_find_contiguous_sum() {
        assert_eq!(find_contiguous_sum(&TEST_NUMBERS, 127), Some(62));
    }
}
