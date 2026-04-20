use super::ast::*;
use super::lexer::*;
use crate::core::parser::Parser;

macro_rules! eat_token_of_kind {
    ($parser:expr, $expected:pat) => {{
        let tok = $parser
            .eat()
            .expect(&format!("Expected {:?} but found None", stringify!($expected)));
        match &tok {
            $expected => tok,
            _ => panic!("Expected {:?} but found {:?}", stringify!($expected), tok)
        }
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

fn translate_tok_to_unop(tok: &Token) -> Option<UnaryOperator> {
    match tok {
        Token::Tilde => Some(UnaryOperator::Complement),
        Token::Hyphen => Some(UnaryOperator::Negation),
        Token::ExclamationPoint => Some(UnaryOperator::Not),
        _ => None,
    }
}

fn translate_tok_to_binop(tok: &Token) -> Option<BinaryOperator> {
    match tok {
        Token::Star => Some(BinaryOperator::Multiply),
        Token::Slash => Some(BinaryOperator::Divide),
        Token::Modulo => Some(BinaryOperator::Modulo),
        Token::Plus => Some(BinaryOperator::Add),
        Token::Hyphen => Some(BinaryOperator::Subtract),
        Token::Ampersand => Some(BinaryOperator::BitwiseAnd),
        Token::Pipe => Some(BinaryOperator::BitwiseOr),
        Token::Caret => Some(BinaryOperator::BitwiseXor),
        Token::DoubleOpenAngleBracket => Some(BinaryOperator::LeftShift),
        Token::DoubleCloseAngleBracket => Some(BinaryOperator::RightShift),
        Token::DoubleAmpersand => Some(BinaryOperator::LogicalAnd),
        Token::DoublePipe => Some(BinaryOperator::LogicalOr),
        Token::DoubleEqual => Some(BinaryOperator::Equal),
        Token::NotEqual => Some(BinaryOperator::NotEqual),
        Token::OpenAngleBracket => Some(BinaryOperator::LessThan),
        Token::LessThanEqual => Some(BinaryOperator::LessThanOrEqual),
        Token::CloseAngleBracket => Some(BinaryOperator::GreaterThan),
        Token::GreaterThanEqual => Some(BinaryOperator::GreaterThanOrEqual),
        _ => None,
    }
}

fn parse_constant(parser: &mut Parser<Token>) -> Expression {
    let tok = eat_token_of_kind!(parser, Token::Constant(_));
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
    match next_tok {
        Token::Constant(_) => parse_constant(parser),
        Token::OpenParenthesis => {
            eat_known_token!(parser, Token::OpenParenthesis);
            let expr = parse_expression(parser);
            eat_known_token!(parser, Token::CloseParenthesis);
            expr
        }
        _ => {
            let tok = parser.eat().expect("Expected start of expression but found None.");
            let maybe_unop = translate_tok_to_unop(&tok);
            if let Some(unop) = maybe_unop {
                let expr = parse_primary(parser);
                return Expression::Unary(unop, Box::new(expr))
            }
            panic!("Invalid expression. Cannot begin with {:?}", tok)
        }
    }
}

fn is_next_token_binary_op_no_lower_precedence(
    parser: &mut Parser<Token>,
    min_precedence: i32,
) -> bool {
    let tok = parser.peek().expect("Expected a token but found None");
    let binop = translate_tok_to_binop(&tok);
    if let Some(binop_val) = binop {
        return binary_operator_precedence(&binop_val) >= min_precedence;
    }
    return false;
}

fn parse_expression_with_precedence(parser: &mut Parser<Token>, min_precedence: i32) -> Expression {
    let mut expr = parse_primary(parser);
    while is_next_token_binary_op_no_lower_precedence(parser, min_precedence) {
        let tok = parser.eat().expect("Expected binary operator but found None.");
        let operator = translate_tok_to_binop(&tok).expect(format!("Expected binary operator but found {:?}", tok).as_str());
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
    eat_token_of_kind!(parser, Token::Semicolon);

    Statement::Return(expr)
}

fn parse_statement(parser: &mut Parser<Token>) -> Statement {
    parse_return(parser)
}

fn parse_function(parser: &mut Parser<Token>) -> Function {
    eat_known_token!(parser, Token::Keyword(String::from("int")));
    let name_tok = eat_token_of_kind!(parser, Token::Identifier(_));
    let name = match name_tok {
        Token::Identifier(name) => Ok(name),
        _ => Err(format!("{:?} should be an identifier", name_tok)),
    }
    .unwrap();
    eat_token_of_kind!(parser, Token::OpenParenthesis);
    let next = parser.peek();
    if let Some(tok) = next && let Token::Keyword(kwd) = tok && kwd.clone() == String::from("void") {
        eat_known_token!(parser, Token::Keyword(String::from("void")));
        eat_token_of_kind!(parser, Token::CloseParenthesis);
    } else if let Some(tok) = next && let Token::CloseParenthesis = tok {
        eat_token_of_kind!(parser, Token::CloseParenthesis);
    } else {
        panic!("Unexpected token in function args")
    }
    eat_token_of_kind!(parser, Token::OpenBrace);
    let statement = parse_statement(parser);
    eat_token_of_kind!(parser, Token::CloseBrace);

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
    #[should_panic = "Expected \"Token::Semicolon\" but found Identifier(\"variable_name\")"]
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
