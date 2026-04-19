mod ast;
mod from_lexical;
pub mod to_tacky;

use crate::core::{lexer, parser};
use regex::Regex;

use std::process;

pub fn process_program(input: String, lex_only: bool) -> ast::Program {
    let tokens = lexer::lex_contents(
        input,
        &lexer::LanguageSpec {
            identifier_rgx: Regex::new(r"^[a-zA-Z_]\w*\b$").unwrap(),
            constant_rgx: Regex::new(r"^[0-9]+\b$").unwrap(),
            keyword_rgx: Regex::new(r"^(int|return|void)$").unwrap(),
        },
    );
    if lex_only {
        // stop here & mark as success if we only want lexing
        process::exit(0);
    }
    let mut parser = parser::Parser::new(tokens);
    from_lexical::parse_program(&mut parser)
}
