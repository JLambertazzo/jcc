use super::lexer::*;
use super::parser::Parser;
use crate::ast::c::*;

fn parse_constant(parser: &mut Parser) -> Expression {
    let tok = parser.eat();
    match tok {
        Token::Constant(val) => {
            let i32_val = val
                .parse::<i32>()
                .expect(&format!("{val} should be an integer"));
            Ok(Expression::Constant(i32_val))
        }
        _ => Err(format!("{:?} should be a constant", tok)),
    }
    .unwrap()
}

fn parse_expression(parser: &mut Parser) -> Expression {
    parse_constant(parser)
}

fn parse_return(parser: &mut Parser) -> Statement {
    assert_eq!(parser.eat(), Token::Keyword(String::from("return")));
    let expr = parse_expression(parser);
    assert_eq!(parser.eat(), Token::Semicolon);

    Statement::Return(expr)
}

fn parse_statement(parser: &mut Parser) -> Statement {
    parse_return(parser)
}

fn parse_function(parser: &mut Parser) -> Function {
    assert_eq!(parser.eat(), Token::Keyword("int".to_string()));
    let name_tok = parser.eat();
    let name = match name_tok {
        Token::Identifier(name) => Ok(name),
        _ => Err(format!("{:?} should be an identifier", name_tok)),
    }
    .unwrap();
    assert_eq!(parser.eat(), Token::OpenParenthesis);
    assert_eq!(parser.eat(), Token::CloseParenthesis);
    assert_eq!(parser.eat(), Token::OpenBrace);
    let statement = parse_statement(parser);
    assert_eq!(parser.eat(), Token::CloseBrace);

    Function::Function(name, statement)
}

pub fn parse_program(parser: &mut Parser) -> Program {
    Program::Program(parse_function(parser))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_program() {
        let program_token_vector = vec![
            Token::Keyword(String::from("int")),
            Token::Identifier(String::from("function_name")),
            Token::OpenParenthesis,
            Token::CloseParenthesis,
            Token::OpenBrace,
            Token::Keyword(String::from("return")),
            Token::Constant(String::from("2")),
            Token::Semicolon,
            Token::CloseBrace,
        ];
        assert_eq!(
            parse_program(&mut Parser::new(program_token_vector)),
            Program::Program(Function::Function(
                "function_name".to_string(),
                Statement::Return(Expression::Constant(2))
            ))
        )
    }
}
