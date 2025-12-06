#[derive(PartialEq, Debug)]
pub enum UnaryOperator {
    Complement,
    Negate,
}

#[derive(PartialEq, Debug)]
pub enum Value {
    Constant(i32),
    Variable(String),
}

#[derive(PartialEq, Debug)]
pub enum Instruction {
    Return(Value),
    Unary(UnaryOperator, Value, Value),
}

#[derive(PartialEq, Debug)]
pub enum Function {
    Function(String, Vec<Instruction>),
}

#[derive(PartialEq, Debug)]
pub enum Program {
    Program(Function),
}
