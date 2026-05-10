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
        Token::Identifier(name) => {
            let name = name.clone();
            eat_token_of_kind!(parser, Token::Identifier(_));
            Expression::Var(name)
        },
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
        let tok = parser.eat().expect("Expected operator in expression but found None.");
        match tok {
            Token::EqualSign => {
                eat_known_token!(parser, Token::EqualSign);
                // TODO add = precedence to map
                let rhs = parse_expression_with_precedence(parser, 1);
                expr = Expression::Assignment(Box::new(expr), Box::new(rhs));
            },
            _ => {
                let operator = translate_tok_to_binop(&tok).expect(format!("Expected binary operator but found {:?}", tok).as_str());
                let rhs =
                    parse_expression_with_precedence(parser, binary_operator_precedence(&operator) + 1);
                expr = Expression::Binary(operator, Box::new(expr), Box::new(rhs));
            }
        }
    }
    expr
}

fn parse_expression(parser: &mut Parser<Token>) -> Expression {
    parse_expression_with_precedence(parser, 0)
}

// way too bulky - simplify!
fn parse_block(parser: &mut Parser<Token>) -> Block {
    let next_tok = parser.peek().expect("Expected block but found None");
    if let Token::Keyword(kwd) = next_tok.clone() && kwd == String::from("int") {
        eat_known_token!(parser, Token::Keyword(String::from("int")));
        let var_name_tok = eat_token_of_kind!(parser, Token::Identifier(_));
        // duplicate check, above macro should handle it. Why not??
        let var_name = match var_name_tok {
            Token::Identifier(name) => name,
            _ => panic!("Failed to retrieve identifier name")
        };
        return match parser.peek().expect("Expected assignment or semicolon but found None") {
            &Token::EqualSign => Block::Declaration(var_name, Some(parse_expression(parser))),
            &Token::Semicolon => Block::Declaration(var_name, None),
            _ => {
                let tok = parser.eat().expect("Token disappeared between peek and eat?");
                panic!("Expected = or ; but found {:?}", tok)
            }
        }
    }

    if let Token::Keyword(kwd) = next_tok.clone() && kwd == String::from("return") {
        eat_known_token!(parser, Token::Keyword(String::from("return")));
        let expr = parse_expression(parser);
        eat_token_of_kind!(parser, Token::Semicolon);
        return Block::Statement(Statement::Return(expr));
    }

    if let Token::Semicolon = next_tok {
        eat_known_token!(parser, Token::Semicolon);
        return Block::Statement(Statement::Null);
    }

    let expr = parse_expression(parser);
    return Block::Statement(Statement::Expression(expr));
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
    let mut blocks: Vec<Block> = vec![];
    while parser.peek() != Some(&Token::CloseBrace) {
        blocks.push(parse_block(parser));
    }
    eat_token_of_kind!(parser, Token::CloseBrace);

    Function::Function(name, blocks)
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
}
