use crate::lexer::*;

struct Constant {
    value: i32,
}

struct Identifier {
    name: String,
}

struct Expression {
    int: Constant,
}

struct Statement {
    expr: Expression,
}

struct Function {
    ident: Identifier,
    statement: Statement,
}

struct Program {
    func: Function,
}

fn parse_constant(tokens: &[Token]) -> Constant {
    assert_ne!(tokens[0].token_type, TokenType::Constant);

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

pub fn parse_program(tokens: &Vec<Token>) {
    Program {
        func: parse_function(tokens.as_slice()),
    };
}
