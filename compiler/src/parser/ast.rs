use super::Parser;
use crate::lexer::*;

#[derive(Debug, PartialEq)]
pub struct Constant {
    value: i32,
}

#[derive(Debug, PartialEq)]
pub struct Identifier {
    name: String,
}
#[derive(Debug, PartialEq)]
pub struct Expression {
    int: Constant,
}
#[derive(Debug, PartialEq)]
pub struct Statement {
    expr: Expression,
}
#[derive(Debug, PartialEq)]
pub struct Function {
    ident: Identifier,
    statement: Statement,
}
#[derive(Debug, PartialEq)]
pub struct Program {
    func: Function,
}

fn parse_constant(parser: &mut Parser) -> Constant {
    let constant_tok = parser.eat();
    assert_eq!(constant_tok.token_type, TokenType::Constant);

    let str_content = constant_tok.content.as_str();
    let i32_val = str_content.parse::<i32>().unwrap();
    Constant { value: i32_val }
}

fn parse_expression(parser: &mut Parser) -> Expression {
    Expression {
        int: parse_constant(parser),
    }
}

fn parse_identifier(parser: &mut Parser) -> Identifier {
    let identifier_tok = parser.eat();
    assert_eq!(identifier_tok.token_type, TokenType::Identifier);

    Identifier {
        name: identifier_tok.content.to_string(),
    }
}

fn parse_statement(parser: &mut Parser) -> Statement {
    assert_eq!(parser.eat().content, "return");
    let expr = parse_expression(parser);
    assert_eq!(parser.eat().content, ";");

    Statement { expr: expr }
}

fn parse_function(parser: &mut Parser) -> Function {
    assert_eq!(parser.eat().content, "int");
    let ident = parse_identifier(parser);
    assert_eq!(parser.eat().content, "(");
    assert_eq!(parser.eat().content, ")");
    assert_eq!(parser.eat().content, "{");
    let statement = parse_statement(parser);
    assert_eq!(parser.eat().content, "}");

    Function {
        ident: ident,
        statement: statement,
    }
}

pub fn parse_program(parser: &mut Parser) -> Program {
    Program {
        func: parse_function(parser),
    }
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
            Constant { value: 1234 }
        );
    }

    #[test]
    fn test_parse_identifier() {
        let ident_token = tok!("function_name", TokenType::Identifier);
        assert_eq!(
            parse_identifier(&mut Parser::new(vec![ident_token])),
            Identifier {
                name: String::from("function_name")
            }
        );
    }

    #[test]
    fn test_parse_expression() {
        let const_token = tok!("1234", TokenType::Constant);
        assert_eq!(
            parse_expression(&mut Parser::new(vec![const_token])),
            Expression {
                int: Constant { value: 1234 }
            }
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
            Statement {
                expr: Expression {
                    int: Constant { value: 2 }
                }
            }
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
            Function {
                ident: Identifier {
                    name: "function_name".to_string()
                },
                statement: Statement {
                    expr: Expression {
                        int: Constant { value: 2 }
                    }
                }
            }
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
            Program {
                func: Function {
                    ident: Identifier {
                        name: "function_name".to_string()
                    },
                    statement: Statement {
                        expr: Expression {
                            int: Constant { value: 2 }
                        }
                    }
                }
            }
        )
    }
}
