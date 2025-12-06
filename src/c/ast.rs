#[derive(PartialEq, Debug)]
pub enum UnaryOperator {
    Negation,
    Complement,
}

#[derive(PartialEq, Debug)]
pub enum Expression {
    Constant(i32),
    Unary(UnaryOperator, Box<Expression>),
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
