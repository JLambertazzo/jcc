use super::ast::{Expression, Function, Program, Statement};

enum Indent {
    Space(usize),
}

const INDENT: Indent = Indent::Space(2);

fn get_indentation_str(indentation: Indent, times: usize) -> String {
    match indentation {
        Indent::Space(n) => " ".repeat(n * times),
    }
}

fn indent(n: usize, content: String) -> String {
    if n == 0 {
        return content;
    }

    let indentation_str = get_indentation_str(INDENT, n);

    let indented = indentation_str.clone()
        + content
            .replace("\n", &format!("\n{}", indentation_str))
            .trim_end_matches(' ');

    indent(n - 1, indented)
}

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
            function_asm.push_str(&indent(1, format!(".globl {}\n", name)));
            function_asm.push_str(&format!("{}:\n", name));
            function_asm.push_str(&indent(1, statement_as_asm(statement)));
            function_asm
        }
    }
}

pub fn program_as_asm(program: Program) -> String {
    match program {
        Program::Program(func) => function_as_asm(func),
    }
}
