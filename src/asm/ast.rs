#[derive(PartialEq, Debug, Clone)]
pub enum Register {
    AX,
    DX,
    R10,
    R11,
    CL,
    CX,
}

#[derive(PartialEq, Debug, Clone)]
pub enum UnaryOperator {
    Neg,
    Not,
}

#[derive(PartialEq, Debug, Clone)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Sar,
    Sal,
    And,
    Or,
    Xor,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Operand {
    Immediate(i32),
    Register(Register),
    Pseudo(String),
    Stack(i32),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Instruction {
    UnaryOp(UnaryOperator, Operand),
    Binary(BinaryOperator, Operand, Operand),
    Mov(Operand, Operand),
    AllocateStack(i32),
    Idiv(Operand),
    Cdq,
    Ret,
}

#[derive(PartialEq, Debug)]
pub enum Function {
    Function(String, Vec<Instruction>),
}

#[derive(PartialEq, Debug)]
pub enum Program {
    Program(Function),
}
