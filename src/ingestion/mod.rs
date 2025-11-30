mod lexer;
mod parse_tokens;
mod parser;

pub fn process_program(input: String) -> crate::ast::c::Program {
    let tokens = lexer::lex_contents(input);
    let mut parser = parser::Parser::new(tokens);
    parse_tokens::parse_program(&mut parser)
}
