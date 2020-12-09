mod game_console;

use game_console::*;

static INPUT: &str = include_str!("input");

fn main() -> Result<(), String> {
    let program = INPUT.parse::<Program>()?;

    // PART 1
    match execute_program(&program) {
        Ok(ProgramTermination::InfiniteLoop(v)) => {
            println!("Part 1 result: {}", v);
        }
        other => {
            println!("Part 1 unexpected result {:?}", other);
        }
    }

    // PART 2
    let res_2 = program.iter().enumerate().find_map(|(i, instr)| {
        let clone_with_updated_instruction = |instr: Instruction| {
            let mut new_progr = program.clone();
            new_progr[i] = instr;
            Some(new_progr)
        };

        let new_program = match instr {
            Instruction::Acc(_) => None,
            Instruction::Jmp(v) => clone_with_updated_instruction(Instruction::Nop(*v)),
            Instruction::Nop(v) => clone_with_updated_instruction(Instruction::Jmp(*v)),
        };

        new_program.and_then(|prog| match execute_program(&prog) {
            Ok(ProgramTermination::Finished(res)) => Some(res),
            _ => None,
        })
    });
    match res_2 {
        Some(v) => {
            println!("Part 2 result: {}", v);
        }
        None => {
            println!("Part 2 result not found");
        }
    }

    Ok(())
}
