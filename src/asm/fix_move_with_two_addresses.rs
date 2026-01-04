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

fn use_scratch_register_for_binop(
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
            asm::BinaryOperator::And => asm::Instruction::Binary(asm::BinaryOperator::And, asm::Operand::Register(asm::Register::R10), asm::Operand::Stack(dst_offset)),
            asm::BinaryOperator::Xor => asm::Instruction::Binary(asm::BinaryOperator::Xor, asm::Operand::Register(asm::Register::R10), asm::Operand::Stack(dst_offset)),
            asm::BinaryOperator::Or => asm::Instruction::Binary(asm::BinaryOperator::Or, asm::Operand::Register(asm::Register::R10), asm::Operand::Stack(dst_offset)),
            _ => panic!("Unexpected multiplication instruction while fixing add/sub instructions")
        };
        result_instructions.push(applying_instruction)
    } else {
        result_instructions.push(asm::Instruction::Binary(operator, src, dst));
    }

    result_instructions
}

// shift count should always be read from %cl
fn use_scratch_register_for_shift(
    operator: asm::BinaryOperator,
    cnt: asm::Operand,
    dst: asm::Operand,
) -> Vec<asm::Instruction> {
    let shift_instruction = match operator {
        asm::BinaryOperator::Sar => asm::Instruction::Binary(asm::BinaryOperator::Sar, asm::Operand::Register(asm::Register::CL), dst),
        asm::BinaryOperator::Sal => asm::Instruction::Binary(asm::BinaryOperator::Sal, asm::Operand::Register(asm::Register::CL), dst),
        _ => panic!("Unexpected non-shift binary operator.")
    };
    vec![
        asm::Instruction::Mov(cnt, asm::Operand::Register(asm::Register::CX)),
        shift_instruction
    ]
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
                    asm::BinaryOperator::Mul => {
                        result_instructions.extend(use_scratch_register_for_mul(src, dst))
                    },
                    asm::BinaryOperator::Sal | asm::BinaryOperator::Sar => {
                        result_instructions.extend(use_scratch_register_for_shift(operator, src, dst))
                    }
                    _ => {
                        result_instructions.extend(use_scratch_register_for_binop(operator, src, dst))
                    }
                }
            },
            _ => result_instructions.push(instruction),
        }
    }

    result_instructions
}

