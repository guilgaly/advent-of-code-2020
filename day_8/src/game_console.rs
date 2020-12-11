use std::collections::HashSet;
use std::convert::TryFrom;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

pub fn execute_program(program: &Program) -> Result<ProgramTermination, String> {
    let end = (program.len() - 1) as i64;
    let mut current_address: i64 = 0;
    let mut accumulator: i64 = 0;
    let mut already_executed: HashSet<i64> = HashSet::new();

    loop {
        if already_executed.contains(&current_address) {
            return Ok(ProgramTermination::InfiniteLoop(accumulator));
        } else if current_address == end {
            return Ok(ProgramTermination::Finished(accumulator));
        }

        already_executed.insert(current_address);

        let address = usize::try_from(current_address)
            .map_err(|_| format!("Illegal address {}", current_address))?;
        let instruction = program
            .get(address)
            .ok_or(format!("Unknown address {}", current_address))?;
        match instruction {
            Instruction::Acc(v) => {
                accumulator += v;
                current_address += 1;
            }
            Instruction::Jmp(v) => {
                current_address += v;
            }
            Instruction::Nop(_) => {
                current_address += 1;
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ProgramTermination {
    InfiniteLoop(i64),
    Finished(i64),
}

// Let's have fun with implementing a newtype pattern for the Program...

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Program(Vec<Instruction>);

impl FromStr for Program {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instructions = s
            .lines()
            .map(|line| line.parse::<Instruction>())
            .collect::<Result<Vec<Instruction>, String>>()?;
        Ok(Program(instructions))
    }
}

impl Deref for Program {
    type Target = Vec<Instruction>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Program {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Instruction {
    Acc(i64),
    Jmp(i64),
    Nop(i64),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();
        let name = split.next().ok_or("Missing instruction name")?;
        let value_str = split.next().ok_or("Missing instruction value")?;
        let value = value_str
            .parse::<i64>()
            .map_err(|_| format!("Invalid instruction value {}", value_str))?;
        match name {
            "acc" => Ok(Instruction::Acc(value)),
            "jmp" => Ok(Instruction::Jmp(value)),
            "nop" => Ok(Instruction::Nop(value)),
            _ => Err(format!("Invalid instruction name {}", name)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Instruction::*;

    #[test]
    fn test_parse_program() -> Result<(), String> {
        let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        let expected = Program(vec![
            Nop(0),
            Acc(1),
            Jmp(4),
            Acc(3),
            Jmp(-3),
            Acc(-99),
            Acc(1),
            Jmp(-4),
            Acc(6),
        ]);
        let actual = input.parse::<Program>()?;
        assert_eq!(actual, expected);
        Ok(())
    }
}
