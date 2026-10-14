mod ast;
mod from_lexical;
pub mod lexer;
pub mod to_tacky;

use crate::core::parser;

use std::process;

pub fn process_program(input: String, lex_only: bool, parse_only: bool) -> ast::Program {
    // C frontend is responsible for:
    //   * lexing
    //   * parsing
    //   * semantic validation
    //
    // pass the *_only flags to control execution. Useful for testing
    let tokens = lexer::lex_contents(input);
    if lex_only {
        process::exit(0);
    }
    let mut parser = parser::Parser::new(tokens);
    let program = from_lexical::parse_program(&mut parser);
    if parse_only {
        process::exit(0);
    }
    program
}
