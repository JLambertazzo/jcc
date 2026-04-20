#[derive(PartialEq, Debug)]
pub enum UnaryOperator {
    Negation,
    Complement,
    Not,
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
    LogicalAnd,
    LogicalOr,
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
}

pub fn binary_operator_precedence(operator: &BinaryOperator) -> i32 {
    match operator {
        BinaryOperator::Multiply => 50,
        BinaryOperator::Divide => 50,
        BinaryOperator::Modulo => 50,
        BinaryOperator::Add => 45,
        BinaryOperator::Subtract => 45,
        BinaryOperator::LeftShift => 40,
        BinaryOperator::RightShift => 40,
        BinaryOperator::LessThan => 35,
        BinaryOperator::LessThanOrEqual => 35,
        BinaryOperator::GreaterThan => 35,
        BinaryOperator::GreaterThanOrEqual => 35,
        BinaryOperator::Equal => 30,
        BinaryOperator::NotEqual => 30,
        BinaryOperator::BitwiseAnd => 25,
        BinaryOperator::BitwiseXor => 20,
        BinaryOperator::BitwiseOr => 15,
        BinaryOperator::LogicalAnd => 10,
        BinaryOperator::LogicalOr => 5,
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
