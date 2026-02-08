use super::ast::*;
use crate::core::lexer::*;
use crate::core::parser::Parser;

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

fn parse_binary_op(parser: &mut Parser<Token>) -> BinaryOperator {
    let tok = parser
        .eat()
        .expect("Expected BinaryOperator but found None");
    match tok {
        Token::Star => BinaryOperator::Multiply,
        Token::Slash => BinaryOperator::Divide,
        Token::Modulo => BinaryOperator::Modulo,
        Token::Plus => BinaryOperator::Add,
        Token::Hyphen => BinaryOperator::Subtract,
        Token::Ampersand => BinaryOperator::BitwiseAnd,
        Token::Pipe => BinaryOperator::BitwiseOr,
        Token::Caret => BinaryOperator::BitwiseXor,
        // expecting two of the same bracket otherwise we have invalid input
        Token::OpenAngleBracket | Token::CloseAngleBracket => {
            let next_tok = parser.peek().expect("Expected a token but found None");
            if get_token_kind(&tok) != get_token_kind(&next_tok) {
                panic!("Expected two of same angle bracket but found {:?} and {:?}", tok, next_tok)
            }
            // next token is valid. eat it as well
            parser.eat().expect("Expected token but found None.");

            match get_token_kind(&tok) {
                TokenKind::OpenAngleBracket => BinaryOperator::LeftShift,
                TokenKind::CloseAngleBracket => BinaryOperator::RightShift,
                _ => panic!("Unexpected change in token kind.")
            }
        },
        _ => panic!("Expected BinaryOperator but found {:?}", tok),
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

fn parse_primary(parser: &mut Parser<Token>) -> Expression {
    let next_tok = parser
        .peek()
        .expect("Expected expression but no token found");
    match get_token_kind(next_tok) {
        TokenKind::Constant => parse_constant(parser),
        TokenKind::Tilde | TokenKind::Hyphen => {
            let op = parse_unary_op(parser);
            let expr = parse_primary(parser);
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

fn is_next_token_binary_op_no_lower_precedence(
    parser: &mut Parser<Token>,
    min_precedence: i32,
) -> bool {
    let tok = parser.peek().expect("Expected a token but found None");
    let binop = match tok {
        Token::Star => BinaryOperator::Multiply,
        Token::Slash => BinaryOperator::Divide,
        Token::Modulo => BinaryOperator::Modulo,
        Token::Plus => BinaryOperator::Add,
        Token::Hyphen => BinaryOperator::Subtract,
        Token::Ampersand => BinaryOperator::BitwiseAnd,
        Token::Pipe => BinaryOperator::BitwiseOr,
        Token::Caret => BinaryOperator::BitwiseXor,
        // expecting two of the same bracket otherwise we have invalid input
        Token::OpenAngleBracket | Token::CloseAngleBracket => {
            let next_tok = parser.peek_ahead().expect("Expected a token but found None");
            if get_token_kind(tok) != get_token_kind(next_tok) {
                panic!("Expected two of same angle bracket but found {:?} and {:?}", tok, next_tok)
            }

            match get_token_kind(tok) {
                TokenKind::OpenAngleBracket => BinaryOperator::LeftShift,
                TokenKind::CloseAngleBracket => BinaryOperator::RightShift,
                _ => panic!("Unexpected change in token kind.")
            }
        },
        _ => {
            return false;
        }
    };
    binary_operator_precedence(&binop) >= min_precedence
}

fn parse_expression_with_precedence(parser: &mut Parser<Token>, min_precedence: i32) -> Expression {
    let mut expr = parse_primary(parser);
    while is_next_token_binary_op_no_lower_precedence(parser, min_precedence) {
        let operator = parse_binary_op(parser);
        let rhs =
            parse_expression_with_precedence(parser, binary_operator_precedence(&operator) + 1);
        expr = Expression::Binary(operator, Box::new(expr), Box::new(rhs));
    }
    expr
}

fn parse_expression(parser: &mut Parser<Token>) -> Expression {
    parse_expression_with_precedence(parser, 0)
}

fn parse_return(parser: &mut Parser<Token>) -> Statement {
    eat_known_token!(parser, Token::Keyword(String::from("return")));
    let expr = parse_expression(parser);
    eat_token_of_kind!(parser, TokenKind::Semicolon);

    Statement::Return(expr)
}

fn parse_statement(parser: &mut Parser<Token>) -> Statement {
    parse_return(parser)
}

fn parse_function(parser: &mut Parser<Token>) -> Function {
    eat_known_token!(parser, Token::Keyword(String::from("int")));
    let name_tok = eat_token_of_kind!(parser, TokenKind::Identifier);
    let name = match name_tok {
        Token::Identifier(name) => Ok(name),
        _ => Err(format!("{:?} should be an identifier", name_tok)),
    }
    .unwrap();
    eat_token_of_kind!(parser, TokenKind::OpenParenthesis);
    let next = parser.peek();
    if let Some(tok) = next && let Token::Keyword(kwd) = tok && kwd.clone() == String::from("void") {
        eat_known_token!(parser, Token::Keyword(String::from("void")));
        eat_token_of_kind!(parser, TokenKind::CloseParenthesis);
    } else if let Some(tok) = next && let Token::CloseParenthesis = tok {
        eat_token_of_kind!(parser, TokenKind::CloseParenthesis);
    } else {
        panic!("Unexpected token in function args")
    }
    eat_token_of_kind!(parser, TokenKind::OpenBrace);
    let statement = parse_statement(parser);
    eat_token_of_kind!(parser, TokenKind::CloseBrace);

    Function::Function(name, statement)
}

pub fn parse_program(parser: &mut Parser<Token>) -> Program {
    let program = Program::Program(parse_function(parser));
    if let Some(tok) = parser.peek() {
        panic!("Parsed entire program but found extra content starting with token {:?}", tok)
    };
    program
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

    #[test]
    #[should_panic = "Expected Keyword(\"int\") but found Keyword(\"return\")"]
    fn panic_on_keyword_in_bad_position() {
        let program_token_vector = vec![
            Token::Keyword(String::from("return")),
            Token::Identifier(String::from("function_name")),
            Token::OpenParenthesis,
            Token::CloseParenthesis,
            Token::OpenBrace,
            Token::Keyword(String::from("return")),
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
            Token::Keyword(String::from("int")),
            Token::Identifier(String::from("function_name")),
            Token::OpenParenthesis,
            Token::CloseParenthesis,
            Token::OpenBrace,
            Token::Keyword(String::from("return")),
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
            Token::Keyword(String::from("int")),
            Token::Identifier(String::from("function_name")),
            Token::OpenParenthesis,
            Token::CloseParenthesis,
            Token::OpenBrace,
            Token::Keyword(String::from("return")),
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
            Token::Keyword(String::from("int")),
            Token::Identifier(String::from("function_name")),
            Token::OpenParenthesis,
            Token::CloseParenthesis,
            Token::OpenBrace,
            Token::Keyword(String::from("return")),
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

    #[test]
    fn test_parse_many_binary_expressions() {
        let program_token_vector = vec![
            Token::Keyword(String::from("int")),
            Token::Identifier("main".to_string()),
            Token::OpenParenthesis,
            Token::CloseParenthesis,
            Token::OpenBrace,
            Token::Keyword(String::from("return")),
            Token::OpenParenthesis,
            Token::Constant(String::from("1")),
            Token::Plus,
            Token::Constant(String::from("2")),
            Token::CloseParenthesis,
            Token::Star,
            Token::OpenParenthesis,
            Token::Constant(String::from("4")),
            Token::Hyphen,
            Token::Constant(String::from("3")),
            Token::CloseParenthesis,
            Token::Slash,
            Token::OpenParenthesis,
            Token::Constant(String::from("3")),
            Token::Modulo,
            Token::Constant(String::from("2")),
            Token::CloseParenthesis,
            Token::Semicolon,
            Token::CloseBrace,
        ];
        assert_eq!(
            parse_program(&mut Parser::new(program_token_vector)),
            Program::Program(Function::Function(
                "main".to_string(),
                Statement::Return(Expression::Binary(
                    BinaryOperator::Divide,
                    Box::new(Expression::Binary(
                        BinaryOperator::Multiply,
                        Box::new(Expression::Binary(
                            BinaryOperator::Add,
                            Box::new(Expression::Constant(1)),
                            Box::new(Expression::Constant(2)),
                        )),
                        Box::new(Expression::Binary(
                            BinaryOperator::Subtract,
                            Box::new(Expression::Constant(4)),
                            Box::new(Expression::Constant(3)),
                        )),
                    )),
                    Box::new(Expression::Binary(
                        BinaryOperator::Modulo,
                        Box::new(Expression::Constant(3)),
                        Box::new(Expression::Constant(2)),
                    )),
                ))
            ))
        )
    }

    #[test]
    fn parse_binary_expression_with_nested_unary() {
        let program_token_vector = vec![
            Token::Keyword(String::from("int")),
            Token::Identifier("main".to_string()),
            Token::OpenParenthesis,
            Token::CloseParenthesis,
            Token::OpenBrace,
            Token::Keyword(String::from("return")),
            Token::Tilde,
            Token::Constant(String::from("2")),
            Token::Plus,
            Token::Hyphen,
            Token::Constant(String::from("3")),
            Token::Semicolon,
            Token::CloseBrace,
        ];
        assert_eq!(
            parse_program(&mut Parser::new(program_token_vector)),
            Program::Program(Function::Function(
                String::from("main"),
                Statement::Return(Expression::Binary(
                    BinaryOperator::Add,
                    Box::new(Expression::Unary(
                        UnaryOperator::Complement,
                        Box::new(Expression::Constant(2))
                    )),
                    Box::new(Expression::Unary(
                        UnaryOperator::Negation,
                        Box::new(Expression::Constant(3))
                    ))
                ))
            ))
        )
    }

    #[test]
    fn applies_correct_order_of_operations() {
        let program_token_vector = vec![
            Token::Keyword(String::from("int")),
            Token::Identifier("main".to_string()),
            Token::OpenParenthesis,
            Token::CloseParenthesis,
            Token::OpenBrace,
            Token::Keyword(String::from("return")),
            Token::Constant(String::from("1")),
            Token::Plus,
            Token::Constant(String::from("2")), // \
            Token::Star,
            Token::Constant(String::from("3")), // /
            Token::Hyphen,
            Token::Constant(String::from("4")), // \
            Token::Slash,
            Token::Constant(String::from("5")), // /
            Token::Plus,
            Token::Constant(String::from("6")), // \
            Token::Modulo,
            Token::Constant(String::from("7")), // /
            Token::Hyphen,
            Token::Constant(String::from("1")),
            Token::Semicolon,
            Token::CloseBrace,
        ];
        assert_eq!(
            parse_program(&mut Parser::new(program_token_vector)),
            Program::Program(Function::Function(
                String::from("main"),
                Statement::Return(Expression::Binary(
                    BinaryOperator::Subtract,
                    Box::new(Expression::Binary(
                        BinaryOperator::Add,
                        Box::new(Expression::Binary(
                            BinaryOperator::Subtract,
                            Box::new(Expression::Binary(
                                BinaryOperator::Add,
                                Box::new(Expression::Constant(1)),
                                Box::new(Expression::Binary(
                                    BinaryOperator::Multiply,
                                    Box::new(Expression::Constant(2)),
                                    Box::new(Expression::Constant(3))
                                ))
                            )),
                            Box::new(Expression::Binary(
                                BinaryOperator::Divide,
                                Box::new(Expression::Constant(4)),
                                Box::new(Expression::Constant(5))
                            ))
                        )),
                        Box::new(Expression::Binary(
                            BinaryOperator::Modulo,
                            Box::new(Expression::Constant(6)),
                            Box::new(Expression::Constant(7)),
                        ))
                    )),
                    Box::new(Expression::Constant(1))
                ))
            ))
        )
    }
}
