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

fn parse_constant(tokens: &[Token]) -> Constant {
    assert_eq!(tokens[0].token_type, TokenType::Constant);

    let str_content = tokens[0].content.as_str();
    let i32_val = str_content.parse::<i32>().unwrap();
    Constant { value: i32_val }
}

fn parse_expression(tokens: &[Token]) -> Expression {
    Expression {
        int: parse_constant(&tokens[0..1]),
    }
}

fn parse_identifier(tokens: &[Token]) -> Identifier {
    assert_eq!(tokens[0].token_type, TokenType::Identifier);

    Identifier {
        name: tokens[0].content.to_string(),
    }
}

fn parse_statement(tokens: &[Token]) -> Statement {
    assert_eq!(tokens[0].content, "return");
    let expr = parse_expression(&tokens[1..2]);
    assert_eq!(tokens[2].content, ";");

    Statement { expr: expr }
}

fn parse_function(tokens: &[Token]) -> Function {
    assert_eq!(tokens[0].content, "int");
    let ident = parse_identifier(&tokens[1..2]);
    assert_eq!(tokens[2].content, "(");
    assert_eq!(tokens[3].content, ")");
    assert_eq!(tokens[4].content, "{");
    let statement = parse_statement(&tokens[5..8]);
    assert_eq!(tokens[8].content, "}");

    Function {
        ident: ident,
        statement: statement,
    }
}

pub fn parse_program(tokens: &Vec<Token>) -> Program {
    Program {
        func: parse_function(tokens.as_slice()),
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
        assert_eq!(parse_constant(&vec![const_token]), Constant { value: 1234 });
    }

    #[test]
    fn test_parse_identifier() {
        let ident_token = tok!("function_name", TokenType::Identifier);
        assert_eq!(
            parse_identifier(&vec![ident_token]),
            Identifier {
                name: String::from("function_name")
            }
        );
    }

    #[test]
    fn test_parse_expression() {
        let const_token = tok!("1234", TokenType::Constant);
        assert_eq!(
            parse_expression(&vec![const_token]),
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
            parse_statement(&statement_as_vector,),
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
            parse_function(&function_token_vector),
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
            parse_program(&program_token_vector),
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
