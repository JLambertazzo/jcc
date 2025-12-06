use crate::c;
use crate::tacky;

fn translate_expression(expr: c::ast::Expression) -> tacky::ast::Value {
    match expr {
        c::ast::Expression::Constant(value) => tacky::ast::Value::Constant(value),
        c::ast::Expression::Unary(_op, _exp) => todo!(),
    }
}

fn translate_statement(statement: c::ast::Statement) -> Vec<tacky::ast::Instruction> {
    match statement {
        c::ast::Statement::Return(expr) => {
            let value = translate_expression(expr);
            vec![tacky::ast::Instruction::Return(value)]
        }
    }
}

fn translate_function(func: c::ast::Function) -> tacky::ast::Function {
    match func {
        c::ast::Function::Function(name, statement) => {
            tacky::ast::Function::Function(name, translate_statement(statement))
        }
    }
}

pub fn translate_program(program: c::ast::Program) -> tacky::ast::Program {
    match program {
        c::ast::Program::Program(func) => tacky::ast::Program::Program(translate_function(func)),
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
            tacky::ast::Program::Program(tacky::ast::Function::Function(
                String::from("foo"),
                vec![tacky::ast::Instruction::Return(
                    tacky::ast::Value::Constant(2)
                )]
            ))
        )
    }
}
