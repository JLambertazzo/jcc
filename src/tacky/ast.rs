#[derive(PartialEq, Debug, Clone)]
pub enum UnaryOperator {
    Complement,
    Negate,
}

#[derive(PartialEq, Debug, Clone)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    LeftShift,
    RightShift,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Value {
    Constant(i32),
    /**
     * Representing TACKY variables with a string + integer allows us to easily
     * expand nested operations (ex: `-(-2)`) into an inline series of
     * operations with each intermediate value stored in an intermediate
     * variable
     */
    Variable(String, i32),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Instruction {
    Return(Value),
    Unary(UnaryOperator, Value, Value),
    Binary(BinaryOperator, Value, Value, Value),
}

#[derive(PartialEq, Debug)]
pub enum Function {
    Function(String, Vec<Instruction>),
}

#[derive(PartialEq, Debug)]
pub enum Program {
    Program(Function),
}
