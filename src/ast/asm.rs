#[derive(PartialEq, Debug)]
pub enum Operand {
    Immediate(i32),
    Register,
}

#[derive(PartialEq, Debug)]
pub enum Instruction {
    Mov(Operand, Operand),
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
