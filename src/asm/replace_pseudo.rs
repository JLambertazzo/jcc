use super::ast as asm;
use std::collections::HashMap;

fn replace_pseudoregisters_in_instructions(
    instructions: Vec<asm::Instruction>,
) -> Vec<asm::Instruction> {
    let mut instructions_without_pseudo: Vec<asm::Instruction> = vec![];
    let mut stack_offset_table: HashMap<String, i32> = HashMap::new();
    let mut curr_offset: i32 = 0;

    let mut replace_pseudoregister = |operand: asm::Operand| {
        if let asm::Operand::Pseudo(ref name) = operand {
            let known_offset = stack_offset_table.get(name);
            if let Some(offset) = known_offset {
                return asm::Operand::Stack(*offset);
            }

            curr_offset = curr_offset + 4;
            stack_offset_table.insert(name.clone(), curr_offset);
            return asm::Operand::Stack(curr_offset);
        }

        operand
    };

    for instruction in instructions {
        let instruction_without_pseudo = match instruction {
            asm::Instruction::Ret => instruction.clone(),
            asm::Instruction::AllocateStack(_) => instruction.clone(),
            asm::Instruction::Mov(src, dst) => {
                asm::Instruction::Mov(replace_pseudoregister(src), replace_pseudoregister(dst))
            }
            asm::Instruction::UnaryOp(op, operand) => {
                asm::Instruction::UnaryOp(op, replace_pseudoregister(operand))
            }
        };
        instructions_without_pseudo.push(instruction_without_pseudo);
    }
    instructions_without_pseudo
}

pub fn replace_pseudoregisters_in_asm_program(program: asm::Program) -> asm::Program {
    match program {
        asm::Program::Program(func) => asm::Program::Program(match func {
            asm::Function::Function(name, instructions) => asm::Function::Function(
                name.clone(),
                replace_pseudoregisters_in_instructions(instructions),
            ),
        }),
    }
}
