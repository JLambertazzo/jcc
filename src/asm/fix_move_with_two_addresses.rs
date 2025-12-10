use super::ast as asm;

pub fn use_scratch_register_for_move(
    instructions: &Vec<asm::Instruction>,
) -> Vec<asm::Instruction> {
    let mut result_instructions: Vec<asm::Instruction> = vec![];

    for instruction in instructions.clone() {
        match instruction {
            asm::Instruction::Mov(src, dst) => {
                if let asm::Operand::Stack(src_offset) = src && let asm::Operand::Stack(dst_offset) = dst {
                    result_instructions.push(asm::Instruction::Mov(asm::Operand::Stack(src_offset), asm::Operand::Register(asm::Register::R10)));
                    result_instructions.push(asm::Instruction::Mov(asm::Operand::Register(asm::Register::R10), asm::Operand::Stack(dst_offset)));
                } else {
                    result_instructions.push(asm::Instruction::Mov(src,dst));
                }
            },
            _ => result_instructions.push(instruction),
        }
    }

    result_instructions
}
