mod ast;
mod parser;
mod to_asm;

use crate::lexer::Token;

pub use parser::Parser;

pub fn parse(tokens: Vec<Token>) -> String {
    let mut parser = parser::Parser::new(tokens);
    let program_as_ast = ast::parse_program(&mut parser);
    to_asm::program_as_asm(program_as_ast)
}
