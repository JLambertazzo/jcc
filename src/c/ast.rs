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
        BinaryOperator::Multiply => 50,
        BinaryOperator::Divide => 50,
        BinaryOperator::Modulo => 50,
        BinaryOperator::Add => 45,
        BinaryOperator::Subtract => 45,
        BinaryOperator::LeftShift => 40,
        BinaryOperator::RightShift => 40,
        BinaryOperator::BitwiseAnd => 25,
        BinaryOperator::BitwiseXor => 20,
        BinaryOperator::BitwiseOr => 15,
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
