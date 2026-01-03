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
            fix_move_with_two_addresses::use_scratch_registers,
        ],
    );
    to_code::asm_program_to_string(processed_asm_program)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tacky::ast as tacky;

    #[test]
    fn parse_simple_tacky_program() {
        let tacky_program = tacky::Program::Program(tacky::Function::Function(
            String::from("main"),
            vec![tacky::Instruction::Return(tacky::Value::Constant(2))],
        ));
        let asm = [
            "  .globl main",
            "main:",
            "  pushq %rbp",
            "  movq %rsp, %rbp",
            "  subq $0, %rsp",
            "  movl $2, %eax",
            "  movq %rbp, %rsp",
            "  popq %rbp",
            "  ret",
            "",
            ".section .note.GNU-stack,\"\",@progbits",
        ]
        .join("\n");
        assert_eq!(tacky_program_to_asm_code(tacky_program), asm + "\n");
    }

    #[test]
    fn parser_multiple_unary_ops() {
        let tacky_program = tacky::Program::Program(tacky::Function::Function(
            String::from("main"),
            vec![
                tacky::Instruction::Unary(
                    tacky::UnaryOperator::Negate,
                    tacky::Value::Constant(2),
                    tacky::Value::Variable(String::from("unary"), 0),
                ),
                tacky::Instruction::Unary(
                    tacky::UnaryOperator::Complement,
                    tacky::Value::Variable(String::from("unary"), 0),
                    tacky::Value::Variable(String::from("unary"), 1),
                ),
                tacky::Instruction::Unary(
                    tacky::UnaryOperator::Negate,
                    tacky::Value::Variable(String::from("unary"), 1),
                    tacky::Value::Variable(String::from("unary"), 2),
                ),
                tacky::Instruction::Return(tacky::Value::Variable(String::from("unary"), 2)),
            ],
        ));
        let asm = [
            "  .globl main",
            "main:",
            "  pushq %rbp",
            "  movq %rsp, %rbp",
            "  subq $12, %rsp",
            "  movl $2, -4(%rbp)",
            "  negl -4(%rbp)",
            "  movl -4(%rbp), %r10d",
            "  movl %r10d, -8(%rbp)",
            "  notl -8(%rbp)",
            "  movl -8(%rbp), %r10d",
            "  movl %r10d, -12(%rbp)",
            "  negl -12(%rbp)",
            "  movl -12(%rbp), %eax",
            "  movq %rbp, %rsp",
            "  popq %rbp",
            "  ret",
            "",
            ".section .note.GNU-stack,\"\",@progbits",
        ]
        .join("\n");
        assert_eq!(tacky_program_to_asm_code(tacky_program), asm + "\n");
    }
}
