use common::lazy_static::lazy_static;
use common::regex::Regex;

static INPUT: &str = include_str!("input");

fn main() -> Result<(), String> {
    let passwords = parse_passwords(INPUT)?;
    println!("Passwords count: {}", passwords.len());

    let valid_passwords_count_old = passwords
        .iter()
        .filter(|pwd| validate_password_old(pwd))
        .count();

    println!(
        "Part 1 - there are {} valid passwords",
        valid_passwords_count_old
    );

    let valid_passwords_count_new = passwords.iter().try_fold(0, |acc, elt| {
        validate_password_new(elt).map(|valid| if valid { acc + 1 } else { acc })
    })?;

    println!(
        "Part 2 - there are {} valid passwords",
        valid_passwords_count_new
    );

    Ok(())
}

fn parse_passwords(input: &str) -> Result<Vec<PasswordInfo>, String> {
    lazy_static! {
        static ref PWD_REGEX: Regex = Regex::new(r"([0-9]+)-([0-9]+) ([a-z]): ([a-z]+)").unwrap();
    }

    input
        .lines()
        .map(|line| {
            PWD_REGEX
                .captures(line)
                .and_then(|cap| {
                    let min = cap.get(1)?.as_str().parse::<usize>().ok()?;
                    let max = cap.get(2)?.as_str().parse::<usize>().ok()?;
                    let expected_letter = cap.get(3)?.as_str().chars().next()?;
                    let password = cap.get(4)?.as_str().to_string();
                    Some(PasswordInfo {
                        min,
                        max,
                        character: expected_letter,
                        password,
                    })
                })
                .ok_or(format!("Failed to parse line {}", line))
        })
        .collect()
}

/// Returns true if valid, false otherwise
fn validate_password_old(password_info: &PasswordInfo) -> bool {
    let occurences = password_info
        .password
        .matches(password_info.character)
        .count();
    occurences >= password_info.min && occurences <= password_info.max
}

/// Returns true if valid, false otherwise
fn validate_password_new(
    PasswordInfo {
        min,
        max,
        character,
        password,
    }: &PasswordInfo,
) -> Result<bool, String> {
    let match_1 = password
        .chars()
        .nth(min - 1)
        .ok_or(format!("Character n°{} not found in {}", min, password))?;
    let match_2 = password
        .chars()
        .nth(max - 1)
        .ok_or(format!("Character n°{} not found in {}", max, password))?;

    Ok((match_1 == *character) != (match_2 == *character))
}

#[derive(Debug, PartialEq)]
struct PasswordInfo {
    min: usize,
    max: usize,
    character: char,
    password: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_LIST: &str = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

    fn expected_pwds() -> [PasswordInfo; 3] {
        [
            PasswordInfo {
                min: 1,
                max: 3,
                character: 'a',
                password: "abcde".to_owned(),
            },
            PasswordInfo {
                min: 1,
                max: 3,
                character: 'b',
                password: "cdefg".to_owned(),
            },
            PasswordInfo {
                min: 2,
                max: 9,
                character: 'c',
                password: "ccccccccc".to_owned(),
            },
        ]
    }

    #[test]
    fn test_parse_passwords() -> Result<(), String> {
        assert_eq!(parse_passwords(TEST_LIST)?, expected_pwds());
        Ok(())
    }

    #[test]
    fn test_validate_password_old() {
        let pwds = expected_pwds();
        assert!(validate_password_old(&pwds[0]));
        assert!(!validate_password_old(&pwds[1]));
        assert!(validate_password_old(&pwds[2]));
    }

    #[test]
    fn test_validate_password_new() -> Result<(), String> {
        let pwds = expected_pwds();
        assert!(validate_password_new(&pwds[0])?);
        assert!(!validate_password_new(&pwds[1])?);
        assert!(!validate_password_new(&pwds[2])?);
        Ok(())
    }
}
