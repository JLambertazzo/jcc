use crate::ast::{asm, c};

fn translate_expression(expr: c::Expression) -> asm::Operand {
    match expr {
        c::Expression::Constant(value) => asm::Operand::Immediate(value),
    }
}

fn translate_statement(statement: c::Statement) -> Vec<asm::Instruction> {
    match statement {
        c::Statement::Return(expr) => {
            let operand = translate_expression(expr);
            vec![
                asm::Instruction::Mov(operand, asm::Operand::Register),
                asm::Instruction::Ret,
            ]
        }
    }
}

fn translate_function(func: c::Function) -> asm::Function {
    match func {
        c::Function::Function(name, statement) => {
            asm::Function::Function(name, translate_statement(statement))
        }
    }
}

pub fn translate_program(program: c::Program) -> asm::Program {
    match program {
        c::Program::Program(func) => asm::Program::Program(translate_function(func)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translate_simple_program() {
        assert_eq!(
            translate_program(c::Program::Program(c::Function::Function(
                String::from("foo"),
                c::Statement::Return(c::Expression::Constant(2))
            ))),
            asm::Program::Program(asm::Function::Function(
                String::from("foo"),
                vec![
                    asm::Instruction::Mov(asm::Operand::Immediate(2), asm::Operand::Register),
                    asm::Instruction::Ret
                ]
            ))
        )
    }
}
