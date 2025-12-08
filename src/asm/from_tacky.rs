use super::ast as asm;
use crate::tacky::ast as tacky;

fn translate_unary_op(op: tacky::UnaryOperator) -> asm::UnaryOperator {
    match op {
        tacky::UnaryOperator::Complement => asm::UnaryOperator::Neg,
        tacky::UnaryOperator::Negate => asm::UnaryOperator::Not,
    }
}

fn translate_value(value: tacky::Value) -> asm::Operand {
    match value {
        tacky::Value::Constant(i) => asm::Operand::Immediate(i),
        tacky::Value::Variable(name, i) => asm::Operand::Pseudo(format!("{name}.{i}")),
    }
}

fn translate_instruction(instruction: tacky::Instruction) -> Vec<asm::Instruction> {
    match instruction {
        tacky::Instruction::Return(value) => vec![
            asm::Instruction::Mov(
                translate_value(value),
                asm::Operand::Register(asm::Register::AX),
            ),
            asm::Instruction::Ret,
        ],
        tacky::Instruction::Unary(op, src, dst) => {
            let dst_operand = translate_value(dst);
            vec![
                asm::Instruction::Mov(translate_value(src), dst_operand.clone()),
                asm::Instruction::UnaryOp(translate_unary_op(op), dst_operand),
            ]
        }
    }
}

fn translate_function(func: tacky::Function) -> asm::Function {
    match func {
        tacky::Function::Function(name, instructions) => {
            let asm_instructions: Vec<asm::Instruction> = instructions
                .into_iter()
                .flat_map(|inst| translate_instruction(inst))
                .collect();

            asm::Function::Function(name, asm_instructions)
        }
    }
}

pub fn translate_program(program: tacky::Program) -> asm::Program {
    match program {
        tacky::Program::Program(func) => asm::Program::Program(translate_function(func)),
    }
}
