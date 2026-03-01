// Refines generic lexical tokens into C-specific, semantically meaningful tokens

use crate::core::{lexer, parser::Parser};

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Identifier(String),
    Constant(String),
    Keyword(String),
    OpenParenthesis,  // (
    CloseParenthesis, // )
    OpenBrace,        // {
    CloseBrace,       // }
    Semicolon,        // ;
    BitwiseNegation,  // ~
    Minus,            // -
    Plus,             // +
    Star,             // * no semantic meaning yet, could be times or ptr deref
    Divide,           // /
    Modulo,           // %
    BitwiseAnd,       // &
    BitwiseOr,        // |
    BitwiseXor,       // ^
    LessThan,         // <
    LessThanEqual,    // <=
    LeftShift,        // <<
    GreaterThan,      // >
    GreaterThanEqual, // >=
    RightShift,       // >>
    LogicalNot,       // !
    NotEqual,         // !=
    EqualTo,          // ==
    LogicalAnd,       // &&
    LogicalOr,        // ||
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Identifier,
    Constant,
    Keyword,
    OpenParenthesis,  // (
    CloseParenthesis, // )
    OpenBrace,        // {
    CloseBrace,       // }
    Semicolon,        // ;
    BitwiseNegation,  // ~
    Minus,            // -
    Plus,             // +
    Star,             // * no semantic meaning yet, could be times or ptr deref
    Divide,           // /
    Modulo,           // %
    BitwiseAnd,       // &
    BitwiseOr,        // |
    BitwiseXor,       // ^
    LessThan,         // <
    LessThanEqual,    // <=
    LeftShift,        // <<
    GreaterThan,      // >
    GreaterThanEqual, // >=
    RightShift,       // >>
    LogicalNot,       // !
    NotEqual,         // !=
    EqualTo,          // ==
    LogicalAnd,       // &&
    LogicalOr,        // ||
}

pub fn get_token_kind(tok: &Token) -> TokenKind {
    match tok {
        Token::Identifier(_) => TokenKind::Identifier,
        Token::Constant(_) => TokenKind::Constant,
        Token::Keyword(_) => TokenKind::Keyword,
        Token::OpenParenthesis => TokenKind::OpenParenthesis,
        Token::CloseParenthesis => TokenKind::CloseParenthesis,
        Token::OpenBrace => TokenKind::OpenBrace,
        Token::CloseBrace => TokenKind::CloseBrace,
        Token::Semicolon => TokenKind::Semicolon,
        Token::BitwiseNegation => TokenKind::BitwiseNegation,
        Token::Minus => TokenKind::Minus,
        Token::Plus => TokenKind::Plus,
        Token::Star => TokenKind::Star,
        Token::Divide => TokenKind::Divide,
        Token::Modulo => TokenKind::Modulo,
        Token::BitwiseAnd => TokenKind::BitwiseAnd,
        Token::BitwiseOr => TokenKind::BitwiseOr,
        Token::BitwiseXor => TokenKind::BitwiseXor,
        Token::LessThan => TokenKind::LessThan,
        Token::LessThanEqual => TokenKind::LessThanEqual,
        Token::GreaterThan => TokenKind::GreaterThan,
        Token::GreaterThanEqual => TokenKind::GreaterThanEqual,
        Token::LogicalNot => TokenKind::LogicalNot,
        Token::NotEqual => TokenKind::NotEqual,
        Token::EqualTo => TokenKind::EqualTo,
        Token::LogicalAnd => TokenKind::LogicalAnd,
        Token::LogicalOr => TokenKind::LogicalOr,
        Token::LeftShift => TokenKind::LeftShift,
        Token::RightShift => TokenKind::RightShift,
    }
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
            lexer::Token::Tilde => Token::BitwiseNegation,
            lexer::Token::Hyphen => Token::Minus,
            lexer::Token::Plus => Token::Plus,
            lexer::Token::Star => Token::Star,
            lexer::Token::Slash => Token::Divide,
            lexer::Token::Modulo => Token::Modulo,
            lexer::Token::Ampersand => match parser.peek() {
                Some(&lexer::Token::Ampersand) => {
                    parser.eat();
                    Token::LogicalAnd
                }
                _ => Token::BitwiseAnd,
            },
            lexer::Token::Pipe => match parser.peek() {
                Some(&lexer::Token::Pipe) => {
                    parser.eat();
                    Token::LogicalOr
                }
                _ => Token::BitwiseOr,
            },
            lexer::Token::Caret => Token::BitwiseXor,
            lexer::Token::OpenAngleBracket => match parser.peek() {
                Some(&lexer::Token::EqualSign) => {
                    parser.eat();
                    Token::LessThanEqual
                }
                Some(&lexer::Token::OpenAngleBracket) => {
                    parser.eat();
                    Token::LeftShift
                }
                _ => Token::LessThan,
            },
            lexer::Token::CloseAngleBracket => match parser.peek() {
                Some(&lexer::Token::EqualSign) => {
                    parser.eat();
                    Token::GreaterThanEqual
                }
                Some(&lexer::Token::CloseParenthesis) => {
                    parser.eat();
                    Token::RightShift
                }
                _ => Token::GreaterThan,
            },
            lexer::Token::ExclamationMark => match parser.peek() {
                Some(&lexer::Token::EqualSign) => {
                    parser.eat();
                    Token::NotEqual
                }
                _ => Token::LogicalNot,
            },
            lexer::Token::EqualSign => match parser.peek() {
                Some(&lexer::Token::EqualSign) => {
                    parser.eat();
                    Token::EqualTo
                }
                _ => panic!("Unsupported token ="),
            },
        };
        refined.push(matched);
    }
    refined
}
