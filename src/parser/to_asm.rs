use super::ast::{Expression, Function, Program, Statement};

fn expression_as_asm(expr: Expression) -> String {
    match expr {
        Expression::Constant(i) => format!("${}", i),
    }
}

fn statement_as_asm(statement: Statement) -> String {
    match statement {
        Statement::Return(expr) => {
            let mut return_asm = String::new();
            return_asm.push_str(&format!("movl {}, %eax\n", expression_as_asm(expr)));
            return_asm.push_str("ret\n");
            return_asm
        }
    }
}

fn function_as_asm(func: Function) -> String {
    match func {
        Function::Function(name, statement) => {
            let mut function_asm = String::new();
            function_asm.push_str(&format!("  .globl {}\n", name));
            function_asm.push_str(&format!("{}:\n", name));
            function_asm.push_str(&statement_as_asm(statement));
            function_asm
        }
    }
}

pub fn program_as_asm(program: Program) -> String {
    match program {
        Program::Program(func) => function_as_asm(func),
    }
}
