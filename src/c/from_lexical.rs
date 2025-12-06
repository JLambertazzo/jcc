use super::ast::*;
use super::lexer::*;
use super::parser::Parser;

macro_rules! eat_token_of_kind {
    ($parser:expr, $expected:expr) => {{
        let tok = $parser
            .eat()
            .expect(&format!("Expected {:?} but found None", $expected));
        if (get_token_kind(&tok) != $expected) {
            panic!("Expected {:?} but found {:?}", $expected, tok)
        }
        tok
    }};
}

macro_rules! eat_known_token {
    ($parser:expr, $expected:expr) => {
        let tok = $parser
            .eat()
            .expect(&format!("Expected {:?} but found None", $expected));
        if (tok != $expected) {
            panic!("Expected {:?} but found {:?}", $expected, tok)
        }
    };
}

fn parse_unary_op(parser: &mut Parser<Token>) -> UnaryOperator {
    let tok = parser.eat().expect("Expected UnaryOperator but found None");
    match tok {
        Token::Tilde => UnaryOperator::Complement,
        Token::Hyphen => UnaryOperator::Negation,
        _ => panic!("Expected UnaryOperator but found {:?}", tok),
    }
}

fn parse_constant(parser: &mut Parser<Token>) -> Expression {
    let tok = eat_token_of_kind!(parser, TokenKind::Constant);
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

fn parse_expression(parser: &mut Parser<Token>) -> Expression {
    let next_tok = parser
        .peek()
        .expect("Expected expression but no token found");
    match get_token_kind(next_tok) {
        TokenKind::Constant => parse_constant(parser),
        TokenKind::Tilde | TokenKind::Hyphen => {
            let op = parse_unary_op(parser);
            let expr = parse_expression(parser);
            Expression::Unary(op, Box::new(expr))
        }
        TokenKind::OpenParenthesis => {
            eat_known_token!(parser, Token::OpenParenthesis);
            let expr = parse_expression(parser);
            eat_known_token!(parser, Token::CloseParenthesis);
            expr
        }
        _ => panic!("Invalid expression. Cannot begin with {:?}", next_tok),
    }
}

fn parse_return(parser: &mut Parser<Token>) -> Statement {
    eat_known_token!(parser, Token::Keyword(Keyword::Return));
    let expr = parse_expression(parser);
    eat_token_of_kind!(parser, TokenKind::Semicolon);

    Statement::Return(expr)
}

fn parse_statement(parser: &mut Parser<Token>) -> Statement {
    parse_return(parser)
}

fn parse_function(parser: &mut Parser<Token>) -> Function {
    eat_known_token!(parser, Token::Keyword(Keyword::Int));
    let name_tok = eat_token_of_kind!(parser, TokenKind::Identifier);
    let name = match name_tok {
        Token::Identifier(name) => Ok(name),
        _ => Err(format!("{:?} should be an identifier", name_tok)),
    }
    .unwrap();
    eat_token_of_kind!(parser, TokenKind::OpenParenthesis);
    eat_token_of_kind!(parser, TokenKind::CloseParenthesis);
    eat_token_of_kind!(parser, TokenKind::OpenBrace);
    let statement = parse_statement(parser);
    eat_token_of_kind!(parser, TokenKind::CloseBrace);

    Function::Function(name, statement)
}

pub fn parse_program(parser: &mut Parser<Token>) -> Program {
    Program::Program(parse_function(parser))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_program() {
        let program_token_vector = vec![
            Token::Keyword(Keyword::Int),
            Token::Identifier(String::from("function_name")),
            Token::OpenParenthesis,
            Token::CloseParenthesis,
            Token::OpenBrace,
            Token::Keyword(Keyword::Return),
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

    #[test]
    #[should_panic = "Expected Keyword(Int) but found Keyword(Return)"]
    fn panic_on_keyword_in_bad_position() {
        let program_token_vector = vec![
            Token::Keyword(Keyword::Return),
            Token::Identifier(String::from("function_name")),
            Token::OpenParenthesis,
            Token::CloseParenthesis,
            Token::OpenBrace,
            Token::Keyword(Keyword::Return),
            Token::Constant(String::from("2")),
            Token::Semicolon,
            Token::CloseBrace,
        ];
        parse_program(&mut Parser::new(program_token_vector));
    }

    #[test]
    #[should_panic = "Expected Semicolon but found Identifier(\"variable_name\")"]
    fn panic_on_unexpected_token_kind() {
        let program_token_vector = vec![
            Token::Keyword(Keyword::Int),
            Token::Identifier(String::from("function_name")),
            Token::OpenParenthesis,
            Token::CloseParenthesis,
            Token::OpenBrace,
            Token::Keyword(Keyword::Return),
            Token::Constant(String::from("2")),
            Token::Identifier(String::from("variable_name")),
            Token::CloseBrace,
        ];
        parse_program(&mut Parser::new(program_token_vector));
    }

    #[test]
    #[should_panic = "Invalid expression. Cannot begin with OpenBrace"]
    fn panic_on_malformed_expression() {
        let program_token_vector = vec![
            Token::Keyword(Keyword::Int),
            Token::Identifier(String::from("function_name")),
            Token::OpenParenthesis,
            Token::CloseParenthesis,
            Token::OpenBrace,
            Token::Keyword(Keyword::Return),
            Token::OpenBrace,
            Token::Constant(String::from("2")),
            Token::CloseBrace,
            Token::Identifier(String::from("variable_name")),
            Token::CloseBrace,
        ];
        parse_program(&mut Parser::new(program_token_vector));
    }

    #[test]
    fn test_parse_nested_expression_program() {
        let program_token_vector = vec![
            Token::Keyword(Keyword::Int),
            Token::Identifier(String::from("function_name")),
            Token::OpenParenthesis,
            Token::CloseParenthesis,
            Token::OpenBrace,
            Token::Keyword(Keyword::Return),
            Token::Hyphen,
            Token::OpenParenthesis,
            Token::Tilde,
            Token::OpenParenthesis,
            Token::Hyphen,
            Token::Constant(String::from("2")),
            Token::CloseParenthesis,
            Token::CloseParenthesis,
            Token::Semicolon,
            Token::CloseBrace,
        ];
        assert_eq!(
            parse_program(&mut Parser::new(program_token_vector)),
            Program::Program(Function::Function(
                "function_name".to_string(),
                Statement::Return(Expression::Unary(
                    UnaryOperator::Negation,
                    Box::new(Expression::Unary(
                        UnaryOperator::Complement,
                        Box::new(Expression::Unary(
                            UnaryOperator::Negation,
                            Box::new(Expression::Constant(2))
                        ))
                    ))
                ))
            ))
        )
    }
}
