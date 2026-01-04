use super::ast as asm;
use crate::tacky::ast as tacky;

fn translate_unary_op(op: tacky::UnaryOperator) -> asm::UnaryOperator {
    match op {
        tacky::UnaryOperator::Complement => asm::UnaryOperator::Not,
        tacky::UnaryOperator::Negate => asm::UnaryOperator::Neg,
    }
}

fn translate_binary_op(op: tacky::BinaryOperator) -> asm::BinaryOperator {
    match op {
        tacky::BinaryOperator::Add => asm::BinaryOperator::Add,
        tacky::BinaryOperator::Subtract => asm::BinaryOperator::Sub,
        tacky::BinaryOperator::Multiply => asm::BinaryOperator::Mul,
        tacky::BinaryOperator::LeftShift => asm::BinaryOperator::Sal,
        tacky::BinaryOperator::RightShift => asm::BinaryOperator::Sar,
        tacky::BinaryOperator::BitwiseAnd => asm::BinaryOperator::And,
        tacky::BinaryOperator::BitwiseXor => asm::BinaryOperator::Xor,
        tacky::BinaryOperator::BitwiseOr => asm::BinaryOperator::Or,
        _ => panic!(
            "Operator {:?} does not have a corresponding binary operator in asm",
            op
        ),
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
        tacky::Instruction::Binary(op, a, b, dst) => match op {
            tacky::BinaryOperator::Modulo => vec![
                asm::Instruction::Mov(
                    translate_value(a),
                    asm::Operand::Register(asm::Register::AX),
                ),
                asm::Instruction::Cdq,
                asm::Instruction::Idiv(translate_value(b)),
                asm::Instruction::Mov(
                    asm::Operand::Register(asm::Register::DX),
                    translate_value(dst),
                ),
            ],
            tacky::BinaryOperator::Divide => vec![
                asm::Instruction::Mov(
                    translate_value(a),
                    asm::Operand::Register(asm::Register::AX),
                ),
                asm::Instruction::Cdq,
                asm::Instruction::Idiv(translate_value(b)),
                asm::Instruction::Mov(
                    asm::Operand::Register(asm::Register::AX),
                    translate_value(dst),
                ),
            ],
            _ => {
                let dst_operand = translate_value(dst);
                vec![
                    // all current binops are associative (+,-,*)
                    asm::Instruction::Mov(translate_value(a), dst_operand.clone()),
                    asm::Instruction::Binary(
                        translate_binary_op(op),
                        translate_value(b),
                        dst_operand,
                    ),
                ]
            }
        },
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
