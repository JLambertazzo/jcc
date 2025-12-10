mod add_stack_allocation;
pub mod ast;
mod fix_move_with_two_addresses;
mod from_tacky;
mod middleware;
mod replace_pseudo;
mod to_code;

pub fn tacky_program_to_asm_code(tacky_program: crate::tacky::ast::Program) -> String {
    let asm_program = from_tacky::translate_program(tacky_program);
    let processed_asm_program = middleware::run_asm_middleware(
        asm_program,
        vec![
            replace_pseudo::replace_pseudoregisters_in_instructions,
            add_stack_allocation::add_stack_allocation_instruction,
            fix_move_with_two_addresses::use_scratch_register_for_move,
        ],
    );
    to_code::asm_program_to_string(processed_asm_program)
}
