mod ast;
mod parser;

use crate::lexer::Token;

pub use parser::Parser;

pub fn parse(tokens: Vec<Token>) -> ast::Program {
    let mut parser = parser::Parser::new(tokens);
    ast::parse_program(&mut parser)
}
