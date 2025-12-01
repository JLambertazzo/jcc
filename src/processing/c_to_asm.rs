use crate::ast::{asm, c};

fn parse_expression(expr: c::Expression) -> asm::Operand {
    match expr {
        c::Expression::Constant(value) => asm::Operand::Immediate(value),
    }
}

fn parse_statement(statement: c::Statement) -> Vec<asm::Instruction> {
    match statement {
        c::Statement::Return(expr) => {
            let operand = parse_expression(expr);
            vec![
                asm::Instruction::Mov(operand, asm::Operand::Register),
                asm::Instruction::Ret,
            ]
        }
    }
}

fn parse_function(func: c::Function) -> asm::Function {
    match func {
        c::Function::Function(name, statement) => {
            asm::Function::Function(name, parse_statement(statement))
        }
    }
}

pub fn parse_program(program: c::Program) -> asm::Program {
    match program {
        c::Program::Program(func) => asm::Program::Program(parse_function(func)),
    }
}
