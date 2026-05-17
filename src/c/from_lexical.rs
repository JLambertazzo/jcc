use super::ast::*;
use super::lexer::*;
use crate::core::parser::Parser;

macro_rules! eat_token_of_kind {
    ($parser:expr, $expected:pat) => {{
        let tok = $parser.eat().expect(&format!(
            "Expected {:?} but found None",
            stringify!($expected)
        ));
        match &tok {
            $expected => tok,
            _ => panic!("Expected {:?} but found {:?}", stringify!($expected), tok),
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
        Token::DoubleEqual => Some(BinaryOperator::IsEqual),
        Token::NotEqual => Some(BinaryOperator::NotEqual),
        Token::OpenAngleBracket => Some(BinaryOperator::LessThan),
        Token::LessThanEqual => Some(BinaryOperator::LessThanOrEqual),
        Token::CloseAngleBracket => Some(BinaryOperator::GreaterThan),
        Token::GreaterThanEqual => Some(BinaryOperator::GreaterThanOrEqual),
        Token::EqualSign => Some(BinaryOperator::Equal),
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
        }
        _ => {
            let tok = parser
                .eat()
                .expect("Expected start of expression but found None.");
            let maybe_unop = translate_tok_to_unop(&tok);
            if let Some(unop) = maybe_unop {
                let expr = parse_primary(parser);
                return Expression::Unary(unop, Box::new(expr));
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
        let tok = parser
            .eat()
            .expect("Expected operator in expression but found None.");
        match tok {
            Token::EqualSign => {
                let rhs = parse_expression_with_precedence(parser, 1);
                expr = Expression::Assignment(Box::new(expr), Box::new(rhs));
            }
            _ => {
                let operator = translate_tok_to_binop(&tok)
                    .expect(format!("Expected binary operator but found {:?}", tok).as_str());
                let rhs = parse_expression_with_precedence(
                    parser,
                    binary_operator_precedence(&operator) + 1,
                );
                expr = Expression::Binary(operator, Box::new(expr), Box::new(rhs));
            }
        }
    }
    expr
}

fn parse_expression(parser: &mut Parser<Token>) -> Expression {
    parse_expression_with_precedence(parser, 0)
}

// parse Block::Statement. We currently support 3 types of statements
// 1. Null statements defined by a single semicolon
// 2. Expressions defined as <expr>;
// 3. Return statements defined as return <expr>;
fn parse_statement(parser: &mut Parser<Token>) -> Block {
    match parser.peek() {
        Some(&Token::Semicolon) => {
            eat_known_token!(parser, Token::Semicolon);
            Block::Statement(Statement::Null)
        }
        Some(Token::Keyword(key)) if key == "return" => {
            eat_known_token!(parser, Token::Keyword(String::from("return")));
            let expr = parse_expression(parser);
            eat_known_token!(parser, Token::Semicolon);
            Block::Statement(Statement::Return(expr))
        }
        Some(_) => {
            let expr = parse_expression(parser);
            eat_known_token!(parser, Token::Semicolon);
            Block::Statement(Statement::Expression(expr))
        }
        None => panic!("Expected statement but no tokens found"),
    }
}

fn parse_declaration(parser: &mut Parser<Token>) -> Block {
    // Parse a declaration. Declarations must start with the variable's type
    // currently we only support declaring int variables
    eat_known_token!(parser, Token::Keyword(String::from("int")));
    let Some(Token::Identifier(var_name)) = parser.eat() else {
        panic!("Expected variable name identifier")
    };
    let expr = match parser.peek() {
        Some(&Token::EqualSign) => {
            eat_known_token!(parser, Token::EqualSign);
            let expr = parse_expression(parser);
            eat_known_token!(parser, Token::Semicolon);
            Some(expr)
        }
        Some(&Token::Semicolon) => None,
        Some(tok) => panic!("Expected = or ; but found {:?}", tok),
        None => panic!("Expected = or ; but found None"),
    };
    Block::Declaration(var_name, expr)
}

fn parse_block(parser: &mut Parser<Token>) -> Block {
    match parser.peek() {
        // currently only variables of type int can be declared
        Some(Token::Keyword(key)) if key == "int" => parse_declaration(parser),
        Some(_) => parse_statement(parser),
        None => panic!("Expected a block but no tokens found"),
    }
}

fn parse_function(parser: &mut Parser<Token>) -> Function {
    eat_known_token!(parser, Token::Keyword(String::from("int")));
    let Some(Token::Identifier(name)) = parser.eat() else {
        panic!("Expected function name identifier")
    };
    eat_token_of_kind!(parser, Token::OpenParenthesis);
    // parse args.. currently only (void) is supported
    if parser.peek() == Some(&Token::Keyword(String::from("void"))) {
        parser.eat();
    }
    eat_known_token!(parser, Token::CloseParenthesis);
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
        panic!(
            "Parsed entire program but found extra content starting with token {:?}",
            tok
        )
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
