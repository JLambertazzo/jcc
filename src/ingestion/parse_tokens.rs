use super::lexer::*;
use super::parser::Parser;
use crate::ast::c::*;

fn parse_constant(parser: &mut Parser) -> Expression {
    let constant_tok = parser.eat();
    assert_eq!(constant_tok.token_type, TokenType::Constant);

    let str_content = constant_tok.content.as_str();
    let i32_val = str_content.parse::<i32>().unwrap();
    Expression::Constant(i32_val)
}

fn parse_expression(parser: &mut Parser) -> Expression {
    parse_constant(parser)
}

fn parse_return(parser: &mut Parser) -> Statement {
    assert_eq!(parser.eat().content, "return");
    let expr = parse_expression(parser);
    assert_eq!(parser.eat().content, ";");

    Statement::Return(expr)
}

fn parse_statement(parser: &mut Parser) -> Statement {
    parse_return(parser)
}

fn parse_function(parser: &mut Parser) -> Function {
    assert_eq!(parser.eat().content, "int");
    let name_tok = parser.eat();
    assert_eq!(name_tok.token_type, TokenType::Identifier);
    assert_eq!(parser.eat().content, "(");
    assert_eq!(parser.eat().content, ")");
    assert_eq!(parser.eat().content, "{");
    let statement = parse_statement(parser);
    assert_eq!(parser.eat().content, "}");

    Function::Function(name_tok.content, statement)
}

pub fn parse_program(parser: &mut Parser) -> Program {
    Program::Program(parse_function(parser))
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! tok {
        ($raw_content:literal, $token_type:expr) => {
            Token {
                token_type: $token_type,
                content: $raw_content.to_string(),
            }
        };
    }

    #[test]
    fn test_parse_constant() {
        let const_token = tok!("1234", TokenType::Constant);
        assert_eq!(
            parse_constant(&mut Parser::new(vec![const_token])),
            Expression::Constant(1234)
        );
    }

    #[test]
    fn test_parse_expression() {
        let const_token = tok!("1234", TokenType::Constant);
        assert_eq!(
            parse_expression(&mut Parser::new(vec![const_token])),
            Expression::Constant(1234)
        );
    }

    #[test]
    fn test_parse_statement() {
        let statement_as_vector = vec![
            tok!("return", TokenType::Keyword),
            tok!("2", TokenType::Constant),
            tok!(";", TokenType::Semicolon),
        ];
        assert_eq!(
            parse_statement(&mut Parser::new(statement_as_vector)),
            Statement::Return(Expression::Constant(2))
        )
    }

    #[test]
    fn test_parse_function() {
        let function_token_vector = vec![
            tok!("int", TokenType::Keyword),
            tok!("function_name", TokenType::Identifier),
            tok!("(", TokenType::OpenParenthesis),
            tok!(")", TokenType::CloseParenthesis),
            tok!("{", TokenType::OpenBrace),
            tok!("return", TokenType::Keyword),
            tok!("2", TokenType::Constant),
            tok!(";", TokenType::Semicolon),
            tok!("}", TokenType::CloseBrace),
        ];
        assert_eq!(
            parse_function(&mut Parser::new(function_token_vector)),
            Function::Function(
                "function_name".to_string(),
                Statement::Return(Expression::Constant(2))
            )
        )
    }

    #[test]
    fn test_parse_program() {
        let program_token_vector = vec![
            tok!("int", TokenType::Keyword),
            tok!("function_name", TokenType::Identifier),
            tok!("(", TokenType::OpenParenthesis),
            tok!(")", TokenType::CloseParenthesis),
            tok!("{", TokenType::OpenBrace),
            tok!("return", TokenType::Keyword),
            tok!("2", TokenType::Constant),
            tok!(";", TokenType::Semicolon),
            tok!("}", TokenType::CloseBrace),
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
