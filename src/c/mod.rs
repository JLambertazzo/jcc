mod ast;
mod from_lexical;
pub mod to_tacky;

use crate::core::{lexer, parser};
use regex::Regex;

pub fn process_program(input: String) -> ast::Program {
    let tokens = lexer::lex_contents(
        input,
        &lexer::LanguageSpec {
            identifier_rgx: Regex::new(r"^[a-zA-Z_]\w*\b$").unwrap(),
            constant_rgx: Regex::new(r"^[0-9]+\b$").unwrap(),
            keyword_rgx: Regex::new(r"^(int|return|void)$").unwrap(),
        },
    );
    let mut parser = parser::Parser::new(tokens);
    from_lexical::parse_program(&mut parser)
}
