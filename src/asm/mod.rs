pub mod ast;
mod from_tacky;
mod replace_pseudo;
mod to_code;

pub fn tacky_program_to_asm_code(tacky_program: crate::tacky::ast::Program) -> String {
    let asm_program = from_tacky::translate_program(tacky_program);
    let asm_program_pseudoregisters_replaced =
        replace_pseudo::replace_pseudoregisters_in_asm_program(asm_program);
    to_code::asm_program_to_string(asm_program_pseudoregisters_replaced)
}
