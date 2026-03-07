// Refines generic lexical tokens into C-specific, semantically meaningful tokens

use crate::core::{lexer, parser::Parser};

/**
 * At the lexer stage we can't yet tell the ary-ness of an operator.
 * - could be unary or binary (-2 or 2 - 2). The advantage of this
 * extra lexical layer is to apply C-specific groupings. For example,
 * preferring >= over >. It's a quick pass over the generic lexer that
 * allows the core lexer to stay generic and the parser to avoid lookaheads.
 */
#[derive(Debug, PartialEq, Clone)]
pub enum LexicalOperator {
    Complement,
    LogicalNot,
    Add,
    Minus, // could be Negation or Minus
    Multiply,
    Divide,
    Modulo,
    LeftShift,
    RightShift,
    BitwiseAnd,
    BitwiseXor,
    BitwiseOr,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
    LogicalAnd,
    LogicalOr,
    NotEqual,
    EqualTo,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Identifier(String),
    Constant(String),
    Keyword(String),
    Operator(LexicalOperator),
    OpenParenthesis,  // (
    CloseParenthesis, // )
    OpenBrace,        // {
    CloseBrace,       // }
    Semicolon,        // ;
}

pub fn refine_lexical_token(input: Vec<lexer::Token>) -> Vec<Token> {
    let mut parser = Parser::new(input);
    let mut refined = Vec::new();
    while let Some(tok) = parser.eat() {
        let matched = match tok {
            lexer::Token::Identifier(ident) => Token::Identifier(ident),
            lexer::Token::Constant(constant) => Token::Constant(constant),
            lexer::Token::Keyword(kwd) => Token::Keyword(kwd),
            lexer::Token::OpenParenthesis => Token::OpenParenthesis,
            lexer::Token::CloseParenthesis => Token::CloseParenthesis,
            lexer::Token::OpenBrace => Token::OpenBrace,
            lexer::Token::CloseBrace => Token::CloseBrace,
            lexer::Token::Semicolon => Token::Semicolon,
            lexer::Token::Tilde => Token::Operator(LexicalOperator::Complement),
            lexer::Token::Hyphen => Token::Operator(LexicalOperator::Minus),
            lexer::Token::Plus => Token::Operator(LexicalOperator::Add),
            lexer::Token::Star => Token::Operator(LexicalOperator::Multiply),
            lexer::Token::Slash => Token::Operator(LexicalOperator::Divide),
            lexer::Token::Modulo => Token::Operator(LexicalOperator::Modulo),
            lexer::Token::Ampersand => match parser.peek() {
                Some(&lexer::Token::Ampersand) => {
                    parser.eat();
                    Token::Operator(LexicalOperator::LogicalAnd)
                }
                _ => Token::Operator(LexicalOperator::BitwiseAnd),
            },
            lexer::Token::Pipe => match parser.peek() {
                Some(&lexer::Token::Pipe) => {
                    parser.eat();
                    Token::Operator(LexicalOperator::LogicalOr)
                }
                _ => Token::Operator(LexicalOperator::BitwiseOr),
            },
            lexer::Token::Caret => Token::Operator(LexicalOperator::BitwiseXor),
            lexer::Token::OpenAngleBracket => match parser.peek() {
                Some(&lexer::Token::EqualSign) => {
                    parser.eat();
                    Token::Operator(LexicalOperator::LessThanEqual)
                }
                Some(&lexer::Token::OpenAngleBracket) => {
                    parser.eat();
                    Token::Operator(LexicalOperator::LeftShift)
                }
                _ => Token::Operator(LexicalOperator::LessThan),
            },
            lexer::Token::CloseAngleBracket => match parser.peek() {
                Some(&lexer::Token::EqualSign) => {
                    parser.eat();
                    Token::Operator(LexicalOperator::GreaterThanEqual)
                }
                Some(&lexer::Token::CloseParenthesis) => {
                    parser.eat();
                    Token::Operator(LexicalOperator::RightShift)
                }
                _ => Token::Operator(LexicalOperator::GreaterThan),
            },
            lexer::Token::ExclamationMark => match parser.peek() {
                Some(&lexer::Token::EqualSign) => {
                    parser.eat();
                    Token::Operator(LexicalOperator::NotEqual)
                }
                _ => Token::Operator(LexicalOperator::LogicalNot),
            },
            lexer::Token::EqualSign => match parser.peek() {
                Some(&lexer::Token::EqualSign) => {
                    parser.eat();
                    Token::Operator(LexicalOperator::EqualTo)
                }
                _ => panic!("Unsupported token ="),
            },
        };
        refined.push(matched);
    }
    refined
}
