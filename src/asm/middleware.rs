use super::ast as asm;
type Middleware = fn(&Vec<asm::Instruction>) -> Vec<asm::Instruction>;

pub fn run_asm_middleware(program: asm::Program, middleware: Vec<Middleware>) -> asm::Program {
    let asm::Program::Program(asm::Function::Function(ref name, ref instructions)) = program;
    let mut modified_instructions = instructions.clone();
    for f in middleware {
        modified_instructions = f(&modified_instructions);
    }
    asm::Program::Program(asm::Function::Function(name.clone(), modified_instructions))
}
