pub struct Parser<T> {
    tokens: Vec<T>,
    cursor: usize,
}

impl<T: Clone> Parser<T> {
    pub fn new(tokens: Vec<T>) -> Self {
        Parser { tokens, cursor: 0 }
    }

    /**
     * Tries to consume a token. Returns None if no token could be extracted
     */
    pub fn eat(&mut self) -> Option<T> {
        match self.tokens.get(self.cursor) {
            Some(token) => {
                self.cursor += 1;
                Some(token.clone())
            }
            None => None,
        }
    }

    /**
     * Return a reference to the next lexical token's value without consuming
     */
    pub fn peek(&self) -> Option<&T> {
        self.tokens.get(self.cursor)
    }

    /**
     * Return a reference to the lexical token after the value under the cursor
     * Does not consume any value
     */
    pub fn peek_ahead(&self) -> Option<&T> {
        self.tokens.get(self.cursor + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::lexer::Token;

    #[test]
    #[should_panic = "custom expectation error message"]
    fn panic_on_no_token() {
        let tokens: Vec<Token> = vec![];
        let mut parser = Parser::new(tokens);
        parser.eat().expect("custom expectation error message");
    }

    #[test]
    fn eat_consumes_token_under_cursor() {
        let mut parser = Parser {
            tokens: vec![Token::OpenParenthesis, Token::CloseParenthesis],
            cursor: 0,
        };
        let first = parser.eat();
        let second = parser.eat();
        assert_eq!(first, Some(Token::OpenParenthesis));
        assert_eq!(second, Some(Token::CloseParenthesis));

        // now that we've consumed all tokens, next call should be None
        assert_eq!(parser.eat(), None);
    }

    #[test]
    fn peek_does_not_consume_token() {
        let parser = Parser {
            tokens: vec![Token::Semicolon],
            cursor: 0,
        };
        let mut next_token = parser.peek();
        for _ in 1..10 {
            next_token = parser.peek();
        }

        assert_eq!(next_token, Some(&Token::Semicolon));
        assert_eq!(parser.cursor, 0);
    }
}
