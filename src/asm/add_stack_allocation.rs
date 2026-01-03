use super::ast as asm;
use std::cmp::max;

fn operand_stack_offset(operand: &asm::Operand) -> i32 {
    match operand {
        asm::Operand::Stack(offset) => offset.clone(),
        _ => -1,
    }
}

fn get_instruction_max_stack_offset(instruction: &asm::Instruction) -> i32 {
    match instruction {
        asm::Instruction::Mov(src, dst) => {
            max(operand_stack_offset(src), operand_stack_offset(dst))
        }
        asm::Instruction::UnaryOp(_, operand) => operand_stack_offset(operand),
        asm::Instruction::Binary(_, src, dst) => {
            max(operand_stack_offset(src), operand_stack_offset(dst))
        }
        asm::Instruction::Idiv(denominator) => operand_stack_offset(denominator),
        asm::Instruction::AllocateStack(_) => -1,
        asm::Instruction::Ret => -1,
        asm::Instruction::Cdq => -1,
    }
}

pub fn add_stack_allocation_instruction(
    instructions: &Vec<asm::Instruction>,
) -> Vec<asm::Instruction> {
    let mut stack_appetite = 0;
    for instruction in instructions {
        let max_from_instruction = get_instruction_max_stack_offset(instruction);
        stack_appetite = max(stack_appetite, max_from_instruction);
    }

    let mut result_instructions = vec![asm::Instruction::AllocateStack(stack_appetite)];
    result_instructions.extend(instructions.clone());
    result_instructions
}
