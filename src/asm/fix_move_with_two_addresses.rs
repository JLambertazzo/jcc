use super::ast as asm;

fn use_scratch_registers_for_mov(src: asm::Operand, dst: asm::Operand) -> Vec<asm::Instruction> {
    let mut result_instructions: Vec<asm::Instruction> = vec![];
    
    if let asm::Operand::Stack(src_offset) = src && let asm::Operand::Stack(dst_offset) = dst {
        result_instructions.push(asm::Instruction::Mov(asm::Operand::Stack(src_offset), asm::Operand::Register(asm::Register::R10)));
        result_instructions.push(asm::Instruction::Mov(asm::Operand::Register(asm::Register::R10), asm::Operand::Stack(dst_offset)));
    } else {
        result_instructions.push(asm::Instruction::Mov(src,dst));
    }

    result_instructions
}

fn use_scratch_register_for_idivl(
    op: asm::Operand
) -> Vec<asm::Instruction> {
    let mut result_instructions: Vec<asm::Instruction> = vec![];
    match op {
        asm::Operand::Immediate(_) => {
            result_instructions.push(asm::Instruction::Mov(op.clone(), asm::Operand::Register(asm::Register::R10)));
            result_instructions.push(asm::Instruction::Idiv(asm::Operand::Register(asm::Register::R10)))
        },
        _ => result_instructions.push(asm::Instruction::Idiv(op))
    }
    result_instructions
}

fn use_scratch_register_for_add_sub(
    operator: asm::BinaryOperator,
    src: asm::Operand,
    dst: asm::Operand
) -> Vec<asm::Instruction> {
    let mut result_instructions: Vec<asm::Instruction> = vec![];
    
    if let asm::Operand::Stack(src_offset) = src && let asm::Operand::Stack(dst_offset) = dst {
        result_instructions.push(asm::Instruction::Mov(asm::Operand::Stack(src_offset), asm::Operand::Register(asm::Register::R10)));
        let applying_instruction = match operator {
            asm::BinaryOperator::Add => asm::Instruction::Binary(asm::BinaryOperator::Add, asm::Operand::Register(asm::Register::R10), asm::Operand::Stack(dst_offset)),
            asm::BinaryOperator::Sub => asm::Instruction::Binary(asm::BinaryOperator::Sub, asm::Operand::Register(asm::Register::R10), asm::Operand::Stack(dst_offset)),
            asm::BinaryOperator::Mul => panic!("Unexpected multiplication instruction while fixing add/sub instructions")
        };
        result_instructions.push(applying_instruction)
    } else {
        result_instructions.push(asm::Instruction::Mov(src,dst));
    }

    result_instructions
}

fn use_scratch_register_for_mul(
    src: asm::Operand,
    dst: asm::Operand
) -> Vec<asm::Instruction> {
    let mut result_instructions: Vec<asm::Instruction> = vec![];

    // can't multiply on a stack pointer, regardless of src operand type
    if let asm::Operand::Stack(_) = dst {
        result_instructions.push(asm::Instruction::Mov(dst.clone(), asm::Operand::Register(asm::Register::R11)));
        result_instructions.push(asm::Instruction::Binary(asm::BinaryOperator::Mul, src, asm::Operand::Register(asm::Register::R11)));
        result_instructions.push(asm::Instruction::Mov(asm::Operand::Register(asm::Register::R11), dst))
    } else {
        result_instructions.push(asm::Instruction::Binary(asm::BinaryOperator::Mul, src, dst))
    }

    result_instructions
}

pub fn use_scratch_registers(
    instructions: &Vec<asm::Instruction>,
) -> Vec<asm::Instruction> {
    let mut result_instructions: Vec<asm::Instruction> = vec![];

    for instruction in instructions.clone() {
        match instruction {
            asm::Instruction::Mov(src, dst) => {
                result_instructions.extend(use_scratch_registers_for_mov(src, dst))
            },
            asm::Instruction::Idiv(op) => {
                result_instructions.extend(use_scratch_register_for_idivl(op))
            },
            asm::Instruction::Binary(operator, src, dst) => {
                match operator {
                    asm::BinaryOperator::Add => {
                        result_instructions.extend(use_scratch_register_for_add_sub(operator, src, dst))
                    },
                    asm::BinaryOperator::Sub => {
                        result_instructions.extend(use_scratch_register_for_add_sub(operator, src, dst))
                    },
                    asm::BinaryOperator::Mul => {
                        result_instructions.extend(use_scratch_register_for_mul(src, dst))
                    }
                }
            },
            _ => result_instructions.push(instruction),
        }
    }

    result_instructions
}

