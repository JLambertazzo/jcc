mod ast;
mod from_lexical;
pub mod lexer;
pub mod to_tacky;

use crate::core::parser;

use std::process;

pub fn process_program(input: String, lex_only: bool) -> ast::Program {
    let tokens = lexer::lex_contents(input);
    if lex_only {
        // stop here & mark as success if we only want lexing
        process::exit(0);
    }
    let mut parser = parser::Parser::new(tokens);
    from_lexical::parse_program(&mut parser)
}
