#[derive(PartialEq, Debug, Clone)]
pub enum Register {
    AX,
    R10,
}

#[derive(PartialEq, Debug)]
pub enum UnaryOperator {
    Neg,
    Not,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Operand {
    Immediate(i32),
    Register(Register),
    Pseudo(String),
    Stack(i32),
}

#[derive(PartialEq, Debug)]
pub enum Instruction {
    UnaryOp(UnaryOperator, Operand),
    Mov(Operand, Operand),
    AllocateStack(i32),
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
