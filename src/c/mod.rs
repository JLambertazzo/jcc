mod ast;
mod from_lexical;
mod lexer;
mod parser;
pub mod to_asm;

pub fn process_program(input: String) -> ast::Program {
    let tokens = lexer::lex_contents(input);
    let mut parser = parser::Parser::new(tokens);
    from_lexical::parse_program(&mut parser)
}
