use super::ast::*;

const INDENT: &str = "  ";

// Currently only using bottom 32 bits of AX, R10 registers. Will need to
// accomodate a requested size later on
fn get_register_name(register: Register, bytes: i32) -> String {
    if bytes == 4 {
        return match register {
            Register::AX => String::from("%eax"),
            Register::DX => String::from("%edx"),
            Register::R10 => String::from("%r10d"),
            Register::R11 => String::from("%r11d"),
            Register::CX => String::from("%ecx"),
            Register::CL => String::from("%cl"),
        };
    }

    if bytes == 1 {
        return match register {
            Register::AX => String::from("%al"),
            Register::DX => String::from("%dl"),
            Register::R10 => String::from("%r10b"),
            Register::R11 => String::from("%r11b"),
            _ => panic!("Register {:?} can't be written in 1-byte variant", register),
        };
    }

    panic!("Unable to write register operating on {:?} bytes", bytes)
}

fn unary_op_to_string(operator: UnaryOperator) -> String {
    match operator {
        UnaryOperator::Neg => String::from("negl"),
        UnaryOperator::Not => String::from("notl"),
    }
}

fn binary_op_to_string(operator: BinaryOperator) -> String {
    match operator {
        BinaryOperator::Add => String::from("addl"),
        BinaryOperator::Sub => String::from("subl"),
        BinaryOperator::Mul => String::from("imull"),
        BinaryOperator::Sar => String::from("sarl"),
        BinaryOperator::Sal => String::from("sall"),
        BinaryOperator::And => String::from("andl"),
        BinaryOperator::Xor => String::from("xorl"),
        BinaryOperator::Or => String::from("orl"),
    }
}

fn cond_code_to_string(cond_code: CondCode) -> String {
    match cond_code {
        CondCode::E => String::from("e"),
        CondCode::NE => String::from("ne"),
        CondCode::L => String::from("l"),
        CondCode::LE => String::from("le"),
        CondCode::G => String::from("g"),
        CondCode::GE => String::from("ge"),
    }
}

fn operand_to_string(operand: Operand, bytes: i32) -> String {
    match operand {
        Operand::Immediate(i) => format!("${}", i),
        Operand::Register(register) => get_register_name(register, bytes),
        Operand::Pseudo(_name) => panic!("Pseudoregisters cannot be emitted to code"),
        Operand::Stack(offset) => format!("-{}(%rbp)", offset),
    }
}

fn instruction_to_string(instruction: Instruction) -> String {
    match instruction {
        Instruction::UnaryOp(op, operand) => format!(
            "{INDENT}{} {}\n",
            unary_op_to_string(op),
            operand_to_string(operand, 4)
        ),
        Instruction::Binary(op, src, dst) => format!(
            "{INDENT}{} {}, {}\n",
            binary_op_to_string(op),
            operand_to_string(src, 4),
            operand_to_string(dst, 4)
        ),
        Instruction::AllocateStack(size) => format!("{INDENT}subq ${}, %rsp\n", size),
        Instruction::Mov(src, dest) => format!(
            "{INDENT}movl {}, {}\n",
            operand_to_string(src, 4),
            operand_to_string(dest, 4)
        ),
        Instruction::Ret => [
            format!("{INDENT}movq %rbp, %rsp\n"),
            format!("{INDENT}popq %rbp\n"),
            format!("{INDENT}ret\n"),
        ]
        .join(""),
        Instruction::Cdq => format!("{INDENT}cdq\n"),
        Instruction::Idiv(denominator) => {
            format!("{INDENT}idivl {}\n", operand_to_string(denominator, 4))
        }
        Instruction::Label(ident) => format!(".L{ident}:\n"),
        Instruction::Jmp(ident) => format!("{INDENT}jmp .L{ident}\n"),
        Instruction::JmpCC(cond_code, ident) => {
            let cc = cond_code_to_string(cond_code);
            format!("{INDENT}j{cc} .L{ident}\n")
        }
        Instruction::SetCC(cond_code, op) => {
            let cc = cond_code_to_string(cond_code);
            let op_string = operand_to_string(op, 1);
            format!("{INDENT}set{cc} {op_string}\n")
        }
        Instruction::Cmp(op1, op2) => {
            let op1_string = operand_to_string(op1, 4);
            let op2_string = operand_to_string(op2, 4);
            format!("{INDENT}cmpl {op1_string}, {op2_string}\n")
        }
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
