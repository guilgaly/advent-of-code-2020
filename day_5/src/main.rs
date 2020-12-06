use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;

static INPUT: &str = include_str!("input");

fn main() -> Result<(), String> {
    let seat_ids = INPUT
        .lines()
        .map(|line| {
            lazy_static! {
                static ref REGEX: Regex = Regex::new(r"([FB]{7})([LR]{3})").unwrap();
            }
            REGEX
                .captures(line)
                .and_then(|cap| {
                    let row_code = cap.get(1)?.as_str();
                    let column_code = cap.get(2)?.as_str();

                    let row_number = find_row_number(row_code);
                    let column_number = find_column_number(column_code);

                    Some(row_number * 8 + column_number)
                })
                .ok_or(format!("Failed to parse line {}", line))
        })
        .collect::<Result<HashSet<i32>, String>>()?;

    let max_seat_id = seat_ids.iter().max().unwrap();
    println!("Part 1 result: {}", max_seat_id);

    let all_ids: HashSet<i32> = (0..*max_seat_id).collect();
    let possible_seats: HashSet<i32> = all_ids
        .symmetric_difference(&seat_ids)
        .filter(|id| seat_ids.contains(&(**id - 1)) && seat_ids.contains(&(**id + 1)))
        .copied()
        .collect();
    if possible_seats.len() == 1 {
        println!("Part 2 result: {:?}", possible_seats);
    } else {
        println!("Part 2 result not found; got: {:?}", possible_seats);
    }

    Ok(())
}

fn find_row_number(code: &str) -> i32 {
    fn recurs((start, end): (i32, i32), remaining: &mut Vec<char>) -> i32 {
        match remaining.pop() {
            None => start,
            Some('F') => recurs(first_half(start, end), remaining),
            Some('B') => recurs(second_half(start, end), remaining),
            _ => panic!(),
        }
    }

    let mut remaining: Vec<char> = code.chars().rev().collect();
    let search_space = (0, 127);

    recurs(search_space, &mut remaining)
}

fn find_column_number(code: &str) -> i32 {
    fn recurs((start, end): (i32, i32), remaining: &mut Vec<char>) -> i32 {
        match remaining.pop() {
            None => start,
            Some('L') => recurs(first_half(start, end), remaining),
            Some('R') => recurs(second_half(start, end), remaining),
            _ => panic!(),
        }
    }

    let mut remaining: Vec<char> = code.chars().rev().collect();
    let search_space = (0, 7);

    recurs(search_space, &mut remaining)
}

fn first_half(start: i32, end: i32) -> (i32, i32) {
    (start, end - (end - start + 1) / 2)
}

fn second_half(start: i32, end: i32) -> (i32, i32) {
    (start + (end - start + 1) / 2, end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_row_number() {
        assert_eq!(find_row_number("FBFBBFF"), 44);
        assert_eq!(find_row_number("BFFFBBF"), 70);
        assert_eq!(find_row_number("FFFBBBF"), 14);
        assert_eq!(find_row_number("BBFFBBF"), 102);
    }

    #[test]
    fn test_find_column_number() {
        assert_eq!(find_column_number("RLR"), 5);
        assert_eq!(find_column_number("RRR"), 7);
        assert_eq!(find_column_number("RLL"), 4);
    }
}
