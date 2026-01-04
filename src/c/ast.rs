#[derive(PartialEq, Debug)]
pub enum UnaryOperator {
    Negation,
    Complement,
}

#[derive(PartialEq, Debug)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    LeftShift,
    RightShift,
    BitwiseAnd,
    BitwiseXor,
    BitwiseOr,
}

pub fn binary_operator_precedence(operator: &BinaryOperator) -> i32 {
    match operator {
        BinaryOperator::Multiply => 6,
        BinaryOperator::Divide => 6,
        BinaryOperator::Modulo => 6,
        BinaryOperator::Add => 5,
        BinaryOperator::Subtract => 5,
        BinaryOperator::LeftShift => 4,
        BinaryOperator::RightShift => 4,
        BinaryOperator::BitwiseAnd => 3,
        BinaryOperator::BitwiseXor => 2,
        BinaryOperator::BitwiseOr => 1,
    }
}

#[derive(PartialEq, Debug)]
pub enum Expression {
    Constant(i32),
    Unary(UnaryOperator, Box<Expression>),
    Binary(BinaryOperator, Box<Expression>, Box<Expression>),
}

#[derive(PartialEq, Debug)]
pub enum Statement {
    Return(Expression),
}

#[derive(PartialEq, Debug)]
pub enum Function {
    Function(String, Statement),
}

#[derive(PartialEq, Debug)]
pub enum Program {
    Program(Function),
}
