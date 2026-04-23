use super::ast as asm;
use crate::tacky::ast as tacky;

fn maybe_get_condition_code(op: &tacky::BinaryOperator) -> Option<asm::CondCode> {
    match op {
        tacky::BinaryOperator::Equal => Some(asm::CondCode::E),
        tacky::BinaryOperator::NotEqual => Some(asm::CondCode::NE),
        tacky::BinaryOperator::LessThan => Some(asm::CondCode::L),
        tacky::BinaryOperator::LessThanEqual => Some(asm::CondCode::LE),
        tacky::BinaryOperator::GreaterThan => Some(asm::CondCode::G),
        tacky::BinaryOperator::GreaterThanEqual => Some(asm::CondCode::GE),
        _ => None,
    }
}

fn translate_unary_op(op: tacky::UnaryOperator) -> asm::UnaryOperator {
    match op {
        tacky::UnaryOperator::Complement => asm::UnaryOperator::Not,
        tacky::UnaryOperator::Negate => asm::UnaryOperator::Neg,
        _ => panic!(
            "Operator {:?} does not have a corresponding unary operator in asm",
            op
        ),
    }
}

fn generate_unary_asm_instruction(
    op: tacky::UnaryOperator,
    src: tacky::Value,
    dst: tacky::Value,
) -> Vec<asm::Instruction> {
    let dst_operand = translate_value(dst);
    match op {
        tacky::UnaryOperator::Not => vec![
            asm::Instruction::Cmp(asm::Operand::Immediate(0), translate_value(src)),
            asm::Instruction::Mov(asm::Operand::Immediate(0), dst_operand.clone()),
            asm::Instruction::SetCC(asm::CondCode::E, dst_operand),
        ],
        _ => vec![
            asm::Instruction::Mov(translate_value(src), dst_operand.clone()),
            asm::Instruction::UnaryOp(translate_unary_op(op), dst_operand),
        ],
    }
}

fn maybe_get_binary_op(op: &tacky::BinaryOperator) -> Option<asm::BinaryOperator> {
    match op {
        tacky::BinaryOperator::Add => Some(asm::BinaryOperator::Add),
        tacky::BinaryOperator::Subtract => Some(asm::BinaryOperator::Sub),
        tacky::BinaryOperator::Multiply => Some(asm::BinaryOperator::Mul),
        tacky::BinaryOperator::LeftShift => Some(asm::BinaryOperator::Sal),
        tacky::BinaryOperator::RightShift => Some(asm::BinaryOperator::Sar),
        tacky::BinaryOperator::BitwiseAnd => Some(asm::BinaryOperator::And),
        tacky::BinaryOperator::BitwiseXor => Some(asm::BinaryOperator::Xor),
        tacky::BinaryOperator::BitwiseOr => Some(asm::BinaryOperator::Or),
        _ => None,
    }
}

fn generate_binary_asm_instruction(
    op: tacky::BinaryOperator,
    src1: tacky::Value,
    src2: tacky::Value,
    dst: tacky::Value,
) -> Vec<asm::Instruction> {
    let dst_operand = translate_value(dst);
    if let Some(cond_code) = maybe_get_condition_code(&op) {
        return vec![
            asm::Instruction::Cmp(translate_value(src2), translate_value(src1)),
            asm::Instruction::Mov(asm::Operand::Immediate(0), dst_operand.clone()),
            asm::Instruction::SetCC(cond_code, dst_operand),
        ];
    }

    if let Some(binop) = maybe_get_binary_op(&op) {
        return vec![
            // all current binops are associative (+,-,*)
            asm::Instruction::Mov(translate_value(src1), dst_operand.clone()),
            asm::Instruction::Binary(binop, translate_value(src2), dst_operand),
        ];
    }

    panic!(
        "Failed to resolve binary operator {:?} to any asm instructions",
        op
    )
}

fn translate_value(value: tacky::Value) -> asm::Operand {
    match value {
        tacky::Value::Constant(i) => asm::Operand::Immediate(i),
        tacky::Value::Variable(name, i) => asm::Operand::Pseudo(format!("{name}.{i}")),
    }
}

fn translate_instruction(instruction: tacky::Instruction) -> Vec<asm::Instruction> {
    match instruction {
        tacky::Instruction::Label(ident) => vec![asm::Instruction::Label(ident)],
        tacky::Instruction::Copy(src, dst) => vec![asm::Instruction::Mov(
            translate_value(src),
            translate_value(dst),
        )],
        tacky::Instruction::Jump(target) => vec![asm::Instruction::Jmp(target)],
        tacky::Instruction::JumpIfZero(cond, target) => vec![
            asm::Instruction::Cmp(asm::Operand::Immediate(0), translate_value(cond)),
            asm::Instruction::JmpCC(asm::CondCode::E, target),
        ],
        tacky::Instruction::JumpIfNotZero(cond, target) => vec![
            asm::Instruction::Cmp(asm::Operand::Immediate(0), translate_value(cond)),
            asm::Instruction::JmpCC(asm::CondCode::NE, target),
        ],
        tacky::Instruction::Return(value) => vec![
            asm::Instruction::Mov(
                translate_value(value),
                asm::Operand::Register(asm::Register::AX),
            ),
            asm::Instruction::Ret,
        ],
        tacky::Instruction::Unary(op, src, dst) => generate_unary_asm_instruction(op, src, dst),
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
            _ => generate_binary_asm_instruction(op, a, b, dst),
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
