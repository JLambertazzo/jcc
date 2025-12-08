use crate::c;
use crate::tacky;

fn translate_unary_operator(op: c::ast::UnaryOperator) -> tacky::ast::UnaryOperator {
    match op {
        c::ast::UnaryOperator::Negation => tacky::ast::UnaryOperator::Negate,
        c::ast::UnaryOperator::Complement => tacky::ast::UnaryOperator::Complement,
    }
}

fn translate_expression(
    expr: c::ast::Expression,
) -> (Vec<tacky::ast::Instruction>, tacky::ast::Value) {
    match expr {
        c::ast::Expression::Constant(value) => (vec![], tacky::ast::Value::Constant(value)),
        c::ast::Expression::Unary(op, inner_expr) => {
            let (mut inner_instructions, inner_value) = translate_expression(*inner_expr);
            let variable = match inner_value {
                tacky::ast::Value::Constant(_val) => {
                    tacky::ast::Value::Variable(String::from("negation_result"), 0)
                }
                tacky::ast::Value::Variable(ref name, i) => {
                    tacky::ast::Value::Variable(name.clone(), i + 1)
                }
            };
            inner_instructions.push(tacky::ast::Instruction::Unary(
                translate_unary_operator(op),
                inner_value,
                variable.clone(),
            ));
            (inner_instructions, variable)
        }
    }
}

fn translate_statement(statement: c::ast::Statement) -> Vec<tacky::ast::Instruction> {
    match statement {
        c::ast::Statement::Return(expr) => {
            let (inner_instructions, value) = translate_expression(expr);
            [
                inner_instructions.as_slice(),
                &[tacky::ast::Instruction::Return(value)],
            ]
            .concat()
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
