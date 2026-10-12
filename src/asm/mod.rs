mod add_stack_allocation;
pub mod ast;
mod fix_instruction_operands;
mod from_tacky;
mod middleware;
mod replace_pseudo;
pub mod to_code;

pub fn tacky_program_to_asm_code(tacky_program: crate::tacky::ast::Program) -> ast::Program {
    let asm_program = from_tacky::translate_program(tacky_program);
    middleware::run_asm_middleware(
        asm_program,
        vec![
            replace_pseudo::replace_pseudoregisters_in_instructions,
            add_stack_allocation::add_stack_allocation_instruction,
            fix_instruction_operands::use_scratch_registers,
        ],
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tacky::ast as tacky;

    /// Linux output ends with a `.note.GNU-stack` section; macOS doesn't.
    fn expected_asm_trailer() -> Vec<String> {
        #[cfg(target_os = "macos")]
        {
            Vec::new()
        }
        #[cfg(not(target_os = "macos"))]
        {
            vec![
                String::new(),
                String::from(".section .note.GNU-stack,\"\",@progbits"),
            ]
        }
    }

    #[test]
    fn parse_simple_tacky_program() {
        let tacky_program = tacky::Program::Program(tacky::Function::Function(
            String::from("main"),
            vec![tacky::Instruction::Return(tacky::Value::Constant(2))],
        ));
        let symbol = format!("{}main", to_code::SYMBOL_PREFIX);
        let expected_lines: Vec<String> = vec![
            format!("  .globl {symbol}"),
            format!("{symbol}:"),
            String::from("  pushq %rbp"),
            String::from("  movq %rsp, %rbp"),
            String::from("  subq $0, %rsp"),
            String::from("  movl $2, %eax"),
            String::from("  movq %rbp, %rsp"),
            String::from("  popq %rbp"),
            String::from("  ret"),
        ]
        .into_iter()
        .chain(expected_asm_trailer())
        .collect();
        let asm = expected_lines.join("\n");
        assert_eq!(
            to_code::asm_program_to_string(tacky_program_to_asm_code(tacky_program)),
            asm + "\n"
        );
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
        let symbol = format!("{}main", to_code::SYMBOL_PREFIX);
        let expected_lines: Vec<String> = vec![
            format!("  .globl {symbol}"),
            format!("{symbol}:"),
            String::from("  pushq %rbp"),
            String::from("  movq %rsp, %rbp"),
            String::from("  subq $12, %rsp"),
            String::from("  movl $2, -4(%rbp)"),
            String::from("  negl -4(%rbp)"),
            String::from("  movl -4(%rbp), %r10d"),
            String::from("  movl %r10d, -8(%rbp)"),
            String::from("  notl -8(%rbp)"),
            String::from("  movl -8(%rbp), %r10d"),
            String::from("  movl %r10d, -12(%rbp)"),
            String::from("  negl -12(%rbp)"),
            String::from("  movl -12(%rbp), %eax"),
            String::from("  movq %rbp, %rsp"),
            String::from("  popq %rbp"),
            String::from("  ret"),
        ]
        .into_iter()
        .chain(expected_asm_trailer())
        .collect();
        let asm = expected_lines.join("\n");
        assert_eq!(
            to_code::asm_program_to_string(tacky_program_to_asm_code(tacky_program)),
            asm + "\n"
        );
    }
}
