use crate::c;
use crate::tacky;

fn translate_unary_operator(op: c::ast::UnaryOperator) -> tacky::ast::UnaryOperator {
    match op {
        c::ast::UnaryOperator::Negation => tacky::ast::UnaryOperator::Negate,
        c::ast::UnaryOperator::Complement => tacky::ast::UnaryOperator::Complement,
        c::ast::UnaryOperator::Not => tacky::ast::UnaryOperator::Not,
    }
}

fn translate_binary_operator(op: c::ast::BinaryOperator) -> tacky::ast::BinaryOperator {
    match op {
        c::ast::BinaryOperator::Add => tacky::ast::BinaryOperator::Add,
        c::ast::BinaryOperator::Subtract => tacky::ast::BinaryOperator::Subtract,
        c::ast::BinaryOperator::Multiply => tacky::ast::BinaryOperator::Multiply,
        c::ast::BinaryOperator::Divide => tacky::ast::BinaryOperator::Divide,
        c::ast::BinaryOperator::Modulo => tacky::ast::BinaryOperator::Modulo,
        c::ast::BinaryOperator::LeftShift => tacky::ast::BinaryOperator::LeftShift,
        c::ast::BinaryOperator::RightShift => tacky::ast::BinaryOperator::RightShift,
        c::ast::BinaryOperator::BitwiseAnd => tacky::ast::BinaryOperator::BitwiseAnd,
        c::ast::BinaryOperator::BitwiseXor => tacky::ast::BinaryOperator::BitwiseXor,
        c::ast::BinaryOperator::BitwiseOr => tacky::ast::BinaryOperator::BitwiseOr,
        c::ast::BinaryOperator::LogicalAnd => tacky::ast::BinaryOperator::LogicalAnd,
        c::ast::BinaryOperator::LogicalOr => tacky::ast::BinaryOperator::LogicalOr,
        c::ast::BinaryOperator::Equal => tacky::ast::BinaryOperator::Equal,
        c::ast::BinaryOperator::NotEqual => tacky::ast::BinaryOperator::NotEqual,
        c::ast::BinaryOperator::LessThan => tacky::ast::BinaryOperator::LessThan,
        c::ast::BinaryOperator::LessThanOrEqual => tacky::ast::BinaryOperator::LessThanEqual,
        c::ast::BinaryOperator::GreaterThan => tacky::ast::BinaryOperator::GreaterThan,
        c::ast::BinaryOperator::GreaterThanOrEqual => tacky::ast::BinaryOperator::GreaterThanEqual,
    }
}

fn name_binary_result(
    op: &tacky::ast::BinaryOperator,
    v1: &tacky::ast::Value,
    v2: &tacky::ast::Value,
) -> String {
    let result_type = match op {
        tacky::ast::BinaryOperator::Add => "Sum",
        tacky::ast::BinaryOperator::Subtract => "Difference",
        tacky::ast::BinaryOperator::Multiply => "Product",
        tacky::ast::BinaryOperator::Divide => "Quotient",
        tacky::ast::BinaryOperator::Modulo => "Remainder",
        tacky::ast::BinaryOperator::LeftShift => "LeftShift",
        tacky::ast::BinaryOperator::RightShift => "RightShift",
        tacky::ast::BinaryOperator::BitwiseAnd => "BitAnd",
        tacky::ast::BinaryOperator::BitwiseOr => "BitOr",
        tacky::ast::BinaryOperator::BitwiseXor => "BitXor",
        tacky::ast::BinaryOperator::LogicalAnd => "LogicalAnd",
        tacky::ast::BinaryOperator::LogicalOr => "LogicalOr",
        tacky::ast::BinaryOperator::Equal => "Equality",
        tacky::ast::BinaryOperator::NotEqual => "NonEquality",
        tacky::ast::BinaryOperator::LessThan => "LessThan",
        tacky::ast::BinaryOperator::LessThanEqual => "LessThanEqual",
        tacky::ast::BinaryOperator::GreaterThan => "GreaterThan",
        tacky::ast::BinaryOperator::GreaterThanEqual => "GreaterThanEqual",
    };
    let v1_name = match v1 {
        tacky::ast::Value::Variable(name, _i) => name.to_string(),
        tacky::ast::Value::Constant(i) => i.to_string(),
    };
    let v2_name = match v2 {
        tacky::ast::Value::Variable(name, _i) => name.to_string(),
        tacky::ast::Value::Constant(i) => i.to_string(),
    };
    format!("{result_type}Of{v1_name}And{v2_name}")
}

fn label_binary_step(
    op: &tacky::ast::BinaryOperator,
    v1: &tacky::ast::Value,
    v2: &tacky::ast::Value,
    step_name: &str,
) -> String {
    format!("{}_{}", name_binary_result(op, v1, v2), step_name)
}

fn generate_binop_instructions(
    tacky_op: tacky::ast::BinaryOperator,
    inner1: Vec<tacky::ast::Instruction>,
    inner1_dst: tacky::ast::Value,
    inner2: Vec<tacky::ast::Instruction>,
    inner2_dst: tacky::ast::Value,
    dst: tacky::ast::Value,
) -> Vec<tacky::ast::Instruction> {
    match tacky_op {
        tacky::ast::BinaryOperator::LogicalAnd => [
            inner1,
            vec![tacky::ast::Instruction::JumpIfZero(
                inner1_dst.clone(),
                label_binary_step(&tacky_op, &inner1_dst, &inner2_dst, "FALSE"),
            )],
            inner2,
            vec![
                tacky::ast::Instruction::JumpIfZero(
                    inner2_dst.clone(),
                    label_binary_step(&tacky_op, &inner1_dst, &inner2_dst, "FALSE"),
                ),
                tacky::ast::Instruction::Copy(tacky::ast::Value::Constant(1), dst.clone()),
                tacky::ast::Instruction::Jump(label_binary_step(
                    &tacky_op,
                    &inner1_dst,
                    &inner2_dst,
                    "END",
                )),
                tacky::ast::Instruction::Label(label_binary_step(
                    &tacky_op,
                    &inner1_dst,
                    &inner2_dst,
                    "FALSE",
                )),
                tacky::ast::Instruction::Copy(tacky::ast::Value::Constant(0), dst),
                tacky::ast::Instruction::Label(label_binary_step(
                    &tacky_op,
                    &inner1_dst,
                    &inner2_dst,
                    "END",
                )),
            ],
        ]
        .concat(),
        tacky::ast::BinaryOperator::LogicalOr => [
            inner1,
            vec![tacky::ast::Instruction::JumpIfNotZero(
                inner1_dst.clone(),
                label_binary_step(&tacky_op, &inner1_dst, &inner2_dst, "TRUE"),
            )],
            inner2,
            vec![
                tacky::ast::Instruction::JumpIfNotZero(
                    inner2_dst.clone(),
                    label_binary_step(&tacky_op, &inner1_dst, &inner2_dst, "TRUE"),
                ),
                tacky::ast::Instruction::Copy(tacky::ast::Value::Constant(0), dst.clone()),
                tacky::ast::Instruction::Jump(label_binary_step(
                    &tacky_op,
                    &inner1_dst,
                    &inner2_dst,
                    "END",
                )),
                tacky::ast::Instruction::Label(label_binary_step(
                    &tacky_op,
                    &inner1_dst,
                    &inner2_dst,
                    "TRUE",
                )),
                tacky::ast::Instruction::Copy(tacky::ast::Value::Constant(1), dst),
                tacky::ast::Instruction::Label(label_binary_step(
                    &tacky_op,
                    &inner1_dst,
                    &inner2_dst,
                    "END",
                )),
            ],
        ]
        .concat(),
        _ => [
            inner1,
            inner2,
            vec![tacky::ast::Instruction::Binary(
                tacky_op,
                inner1_dst,
                inner2_dst,
                dst.clone(),
            )],
        ]
        .concat(),
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
                    tacky::ast::Value::Variable(String::from("unary"), 0)
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
        c::ast::Expression::Binary(op, v1, v2) => {
            let (inner_instructions_v1, inner_value_v1) = translate_expression(*v1);
            let (inner_instructions_v2, inner_value_v2) = translate_expression(*v2);
            let tacky_op = translate_binary_operator(op);
            let dst = tacky::ast::Value::Variable(
                name_binary_result(&tacky_op, &inner_value_v1, &inner_value_v2),
                0,
            );
            let instructions = generate_binop_instructions(
                tacky_op,
                inner_instructions_v1,
                inner_value_v1,
                inner_instructions_v2,
                inner_value_v2,
                dst.clone(),
            );
            (instructions, dst)
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

    #[test]
    fn test_translate_nested_unary_op() {
        let c_program = c::ast::Program::Program(c::ast::Function::Function(
            String::from("main"),
            c::ast::Statement::Return(c::ast::Expression::Unary(
                c::ast::UnaryOperator::Complement,
                Box::new(c::ast::Expression::Unary(
                    c::ast::UnaryOperator::Negation,
                    Box::new(c::ast::Expression::Unary(
                        c::ast::UnaryOperator::Complement,
                        Box::new(c::ast::Expression::Constant(2)),
                    )),
                )),
            )),
        ));
        let tacky_program = tacky::ast::Program::Program(tacky::ast::Function::Function(
            String::from("main"),
            vec![
                tacky::ast::Instruction::Unary(
                    tacky::ast::UnaryOperator::Complement,
                    tacky::ast::Value::Constant(2),
                    tacky::ast::Value::Variable(String::from("unary"), 0),
                ),
                tacky::ast::Instruction::Unary(
                    tacky::ast::UnaryOperator::Negate,
                    tacky::ast::Value::Variable(String::from("unary"), 0),
                    tacky::ast::Value::Variable(String::from("unary"), 1),
                ),
                tacky::ast::Instruction::Unary(
                    tacky::ast::UnaryOperator::Complement,
                    tacky::ast::Value::Variable(String::from("unary"), 1),
                    tacky::ast::Value::Variable(String::from("unary"), 2),
                ),
                tacky::ast::Instruction::Return(tacky::ast::Value::Variable(
                    String::from("unary"),
                    2,
                )),
            ],
        ));
        assert_eq!(translate_program(c_program), tacky_program);
    }

    #[test]
    fn translate_binary_operation_chain() {
        let c_program = c::ast::Program::Program(c::ast::Function::Function(
            String::from("main"),
            c::ast::Statement::Return(c::ast::Expression::Binary(
                c::ast::BinaryOperator::Add,
                Box::new(c::ast::Expression::Constant(1)),
                Box::new(c::ast::Expression::Binary(
                    c::ast::BinaryOperator::Multiply,
                    Box::new(c::ast::Expression::Constant(2)),
                    Box::new(c::ast::Expression::Constant(3)),
                )),
            )),
        ));
        assert_eq!(
            translate_program(c_program),
            tacky::ast::Program::Program(tacky::ast::Function::Function(
                String::from("main"),
                vec![
                    tacky::ast::Instruction::Binary(
                        tacky::ast::BinaryOperator::Multiply,
                        tacky::ast::Value::Constant(2),
                        tacky::ast::Value::Constant(3),
                        tacky::ast::Value::Variable(String::from("ProductOf2And3"), 0)
                    ),
                    tacky::ast::Instruction::Binary(
                        tacky::ast::BinaryOperator::Add,
                        tacky::ast::Value::Constant(1),
                        tacky::ast::Value::Variable(String::from("ProductOf2And3"), 0),
                        tacky::ast::Value::Variable(String::from("SumOf1AndProductOf2And3"), 0)
                    ),
                    tacky::ast::Instruction::Return(tacky::ast::Value::Variable(
                        String::from("SumOf1AndProductOf2And3"),
                        0
                    ))
                ]
            ))
        );
    }
}
