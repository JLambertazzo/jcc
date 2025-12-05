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
     * Tries to consume a token. Returns None if no token could be extracted
     */
    pub fn eat(&mut self) -> Option<Token> {
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
    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.cursor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic = "custom expectation error message"]
    fn panic_on_no_token() {
        let mut parser = Parser {
            tokens: vec![],
            cursor: 0,
        };
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
