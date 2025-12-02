use crate::ingestion::lexer::*;

pub struct Parser {
    tokens: Vec<Token>,
    cursor: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, cursor: 0 }
    }

    /**
     * Consume the token under the current cursor. Assumes a token is present.
     * Will panic if no token is found.
     */
    pub fn eat(&mut self) -> Token {
        let token = self
            .tokens
            .get(self.cursor)
            .expect("Expected a Token")
            .clone();
        self.cursor += 1;

        token
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic::{catch_unwind, AssertUnwindSafe};

    #[test]
    #[should_panic]
    fn panic_on_no_token() {
        let mut parser = Parser {
            tokens: vec![],
            cursor: 0,
        };
        parser.eat();
    }

    #[test]
    fn eat_consumes_token_under_cursor() {
        let mut parser = Parser {
            tokens: vec![Token::OpenParenthesis, Token::CloseParenthesis],
            cursor: 0,
        };
        let first = parser.eat();
        let second = parser.eat();
        assert_eq!(first, Token::OpenParenthesis);
        assert_eq!(second, Token::CloseParenthesis);

        // now that we've consumed all tokens, next call should panic
        let err = catch_unwind(AssertUnwindSafe(|| parser.eat()));
        assert_eq!(err.is_err(), true)
    }
}
