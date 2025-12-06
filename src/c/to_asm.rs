use crate::{asm, c};

fn translate_expression(expr: c::ast::Expression) -> asm::ast::Operand {
    match expr {
        c::ast::Expression::Constant(value) => asm::ast::Operand::Immediate(value),
        c::ast::Expression::Unary(_op, _exp) => todo!(),
    }
}

fn translate_statement(statement: c::ast::Statement) -> Vec<asm::ast::Instruction> {
    match statement {
        c::ast::Statement::Return(expr) => {
            let operand = translate_expression(expr);
            vec![
                asm::ast::Instruction::Mov(operand, asm::ast::Operand::Register),
                asm::ast::Instruction::Ret,
            ]
        }
    }
}

fn translate_function(func: c::ast::Function) -> asm::ast::Function {
    match func {
        c::ast::Function::Function(name, statement) => {
            asm::ast::Function::Function(name, translate_statement(statement))
        }
    }
}

pub fn translate_program(program: c::ast::Program) -> asm::ast::Program {
    match program {
        c::ast::Program::Program(func) => asm::ast::Program::Program(translate_function(func)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translate_simple_program() {
        assert_eq!(
            translate_program(c::ast::Program::Program(c::ast::Function::Function(
                String::from("foo"),
                c::ast::Statement::Return(c::ast::Expression::Constant(2))
            ))),
            asm::ast::Program::Program(asm::ast::Function::Function(
                String::from("foo"),
                vec![
                    asm::ast::Instruction::Mov(
                        asm::ast::Operand::Immediate(2),
                        asm::ast::Operand::Register
                    ),
                    asm::ast::Instruction::Ret
                ]
            ))
        )
    }
}
