use super::ast as asm;
type Middleware = fn(&Vec<asm::Instruction>) -> Vec<asm::Instruction>;

pub fn run_asm_middleware(program: asm::Program, middleware: Vec<Middleware>) -> asm::Program {
    let mut _program = program;
    for f in middleware {
        let _program = match _program {
            asm::Program::Program(ref func) => match func {
                asm::Function::Function(name, instructions) => {
                    asm::Function::Function(name.clone(), f(instructions))
                }
            },
        };
    }
    _program
}
