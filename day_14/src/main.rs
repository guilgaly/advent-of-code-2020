use common::lazy_static::lazy_static;
use common::regex::Regex;

use std::collections::HashMap;
use std::str::FromStr;

static INPUT: &str = include_str!("input");

fn main() -> Result<(), String> {
    let program = parse_input(INPUT)?;

    println!("Part 1 result: {}", part_1(&program));

    println!("Part 2 result: {}", part_2(&program));

    Ok(())
}

fn part_1(program: &[ProgramLine]) -> u64 {
    let mut mask: Vec<BitmaskBit> = Vec::new();
    let mut memory: HashMap<u64, u64> = HashMap::new();
    for line in program.iter() {
        match line {
            ProgramLine::Mask(m) => mask = m.clone(),
            ProgramLine::Mem { target, value } => {
                let value = to_bit_36(*value);
                let value = mask
                    .iter()
                    .zip(value)
                    .map(|(m, v)| match m {
                        BitmaskBit::Zero => false,
                        BitmaskBit::One => true,
                        BitmaskBit::X => v,
                    })
                    .collect::<Vec<bool>>();
                let value = from_bit_36(&value);
                memory.insert(*target, value);
            }
        }
    }
    memory.values().sum::<u64>()
}

fn part_2(program: &[ProgramLine]) -> u64 {
    let mut mask: Vec<BitmaskBit> = Vec::new();
    let mut memory: HashMap<u64, u64> = HashMap::new();
    for line in program.iter() {
        match line {
            ProgramLine::Mask(m) => mask = m.clone(),
            ProgramLine::Mem { target, value } => {
                let target = to_bit_36(*target);
                let target = mask.iter().zip(target).map(|(m, t)| match m {
                    BitmaskBit::Zero => Some(t),
                    BitmaskBit::One => Some(true),
                    BitmaskBit::X => None,
                });
                let zero: Vec<Vec<bool>> = vec![vec![]];
                let targets = target.fold(zero, |acc, maybe_bit| match maybe_bit {
                    Some(bit) => acc
                        .iter()
                        .map(|t| {
                            let mut copy = t.clone();
                            copy.push(bit);
                            copy
                        })
                        .collect(),
                    None => acc
                        .iter()
                        .flat_map(|t| {
                            let mut copy_1 = t.clone();
                            let mut copy_2 = t.clone();
                            copy_1.push(false);
                            copy_2.push(true);
                            vec![copy_1, copy_2]
                        })
                        .collect(),
                });
                for target in targets {
                    let target = from_bit_36(&target);
                    memory.insert(target, *value);
                }
            }
        }
    }
    memory.values().sum::<u64>()
}

fn to_bit_36(v: u64) -> Vec<bool> {
    let bin = format!("{:b}", v);
    let padded_bin = format!("{:0>36}", bin);
    padded_bin.chars().map(|c| c != '0').collect()
}

fn from_bit_36(v: &[bool]) -> u64 {
    v.iter().fold(0, |acc, &b| acc * 2 + if b { 1 } else { 0 })
}

fn parse_input(input: &str) -> Result<Vec<ProgramLine>, String> {
    input
        .lines()
        .map(|line| line.parse::<ProgramLine>())
        .collect()
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum ProgramLine {
    Mask(Vec<BitmaskBit>),
    Mem { target: u64, value: u64 },
}

impl FromStr for ProgramLine {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref REGEX_MASK: Regex = Regex::new(r"^mask = ([01X]{36})$").unwrap();
            static ref REGEX_MEM: Regex = Regex::new(r"^mem\[([0-9]+)] = ([0-9]+)$").unwrap();
        }

        fn maybe_as_mask(s: &str) -> Option<ProgramLine> {
            REGEX_MASK.captures(s).and_then(|cap_mask| {
                let mask_str = cap_mask.get(1)?.as_str();
                let mask: Vec<BitmaskBit> = mask_str
                    .chars()
                    .map(|c| match c {
                        '0' => BitmaskBit::Zero,
                        '1' => BitmaskBit::One,
                        _ => BitmaskBit::X,
                    })
                    .collect();
                Some(ProgramLine::Mask(mask))
            })
        }

        fn maybe_as_mem(s: &str) -> Option<ProgramLine> {
            REGEX_MEM.captures(s).and_then(|cap_line| {
                let target = cap_line.get(1)?.as_str().parse::<u64>().ok()?;
                let value = cap_line.get(2)?.as_str().parse::<u64>().ok()?;
                Some(ProgramLine::Mem { target, value })
            })
        }

        maybe_as_mask(s)
            .or_else(|| maybe_as_mem(s))
            .ok_or(format!("Not a valid program line: {}", s))
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum BitmaskBit {
    Zero,
    One,
    X,
}

#[cfg(test)]
mod tests {

    #[test]
    fn test() {}
}
