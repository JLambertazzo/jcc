use crate::ast::asm::*;

const INDENT: &str = "  ";

fn operand_to_string(operand: Operand) -> String {
    match operand {
        Operand::Immediate(i) => format!("${}", i),
        Operand::Register => String::from("%eax"),
    }
}

fn instruction_to_string(instruction: Instruction) -> String {
    match instruction {
        Instruction::Mov(src, dest) => format!(
            "{INDENT}movl {}, {}\n",
            operand_to_string(src),
            operand_to_string(dest)
        ),
        Instruction::Ret => format!("{INDENT}ret\n"),
    }
}

fn function_to_string(func: Function) -> String {
    match func {
        Function::Function(name, instructions) => {
            let mut instruction_strings: Vec<String> = vec![];
            for instruction in instructions.into_iter() {
                instruction_strings.push(instruction_to_string(instruction));
            }

            format!(
                "{INDENT}.globl {name}\n{name}:\n{}",
                instruction_strings.join("")
            )
        }
    }
}

pub fn asm_program_to_string(program: Program) -> String {
    match program {
        Program::Program(func) => function_to_string(func),
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
                    Instruction::Mov(Operand::Immediate(2), Operand::Register),
                    Instruction::Ret
                ]
            ))),
            format!("{INDENT}.globl main\nmain:\n  movl $2, %eax\n  ret\n")
        )
    }
}
