use super::ast::*;

const INDENT: &str = "  ";

// Currently only using bottom 32 bits of AX, R10 registers. Will need to
// accomodate a requested size later on
fn get_register_name(register: Register) -> String {
    match register {
        Register::AX => String::from("%eax"),
        Register::R10 => String::from("%r10d"),
    }
}

fn unary_op_to_string(operator: UnaryOperator) -> String {
    match operator {
        UnaryOperator::Neg => String::from("negl"),
        UnaryOperator::Not => String::from("notl"),
    }
}

fn operand_to_string(operand: Operand) -> String {
    match operand {
        Operand::Immediate(i) => format!("${}", i),
        Operand::Register(register) => get_register_name(register),
        Operand::Pseudo(_name) => panic!("Pseudoregisters cannot be emitted to code"),
        Operand::Stack(offset) => format!("-{}(%rbp)", offset),
    }
}

fn instruction_to_string(instruction: Instruction) -> String {
    match instruction {
        Instruction::UnaryOp(op, operand) => format!(
            "{INDENT}{} {}\n",
            unary_op_to_string(op),
            operand_to_string(operand)
        ),
        Instruction::AllocateStack(size) => format!("{INDENT}subq ${}, %rsp\n", size),
        Instruction::Mov(src, dest) => format!(
            "{INDENT}movl {}, {}\n",
            operand_to_string(src),
            operand_to_string(dest)
        ),
        Instruction::Ret => [
            format!("{INDENT}movq %rbp, %rsp\n"),
            format!("{INDENT}popq %rbp\n"),
            format!("{INDENT}ret\n"),
        ]
        .join(""),
    }
}

fn function_to_string(func: Function) -> String {
    match func {
        Function::Function(name, instructions) => {
            let mut instruction_strings: Vec<String> = vec![];
            for instruction in instructions.into_iter() {
                instruction_strings.push(instruction_to_string(instruction));
            }

            let function_header = format!("{INDENT}pushq %rbp\n{INDENT}movq %rsp, %rbp\n");

            format!(
                "{INDENT}.globl {name}\n{name}:\n{function_header}{}",
                instruction_strings.join("")
            )
        }
    }
}

pub fn asm_program_to_string(program: Program) -> String {
    match program {
        Program::Program(func) => format!(
            "{}\n.section .note.GNU-stack,\"\",@progbits\n",
            function_to_string(func)
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_asm_program_to_string() {
        assert_eq!(
            asm_program_to_string(Program::Program(Function::Function(
                String::from("main"),
                vec![
                    Instruction::AllocateStack(0),
                    Instruction::Mov(Operand::Immediate(2), Operand::Register(Register::AX)),
                    Instruction::Ret
                ]
            ))),
            format!("{INDENT}.globl main\nmain:\n{INDENT}pushq %rbp\n{INDENT}movq %rsp, %rbp\n{INDENT}subq $0, %rsp\n{INDENT}movl $2, %eax\n{INDENT}movq %rbp, %rsp\n{INDENT}popq %rbp\n{INDENT}ret\n\n.section .note.GNU-stack,\"\",@progbits\n")
        )
    }
}
