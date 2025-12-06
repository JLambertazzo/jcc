use regex::Regex;

#[derive(Debug, PartialEq, Clone)]
pub enum Keyword {
    Return,
    Int,
}

fn lex_keyword(input: String) -> Keyword {
    match input.as_str() {
        "return" => Some(Keyword::Return),
        "int" => Some(Keyword::Int),
        _ => None,
    }
    .expect(&format!("{input} should be a known keyword"))
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Identifier(String),
    Constant(String),
    Keyword(Keyword),
    OpenParenthesis,
    CloseParenthesis,
    OpenBrace,
    CloseBrace,
    Semicolon,
    Tilde,
    Hyphen,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Identifier,
    Constant,
    Keyword,
    OpenParenthesis,
    CloseParenthesis,
    OpenBrace,
    CloseBrace,
    Semicolon,
    Tilde,
    Hyphen,
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
        Token::Tilde => TokenKind::Tilde,
        Token::Hyphen => TokenKind::Hyphen,
    }
}

fn classify_token(token_content: &str) -> Token {
    /*
     * Given regex patterns for each token type.
     * Any other value you encounter at this point should be handled by exact match
     */

    let identifier_rgx = Regex::new(r"^[a-zA-Z_]\w*\b$").unwrap();
    let constant_rgx = Regex::new(r"^[0-9]+\b$").unwrap();
    let keyword_rgx = Regex::new(r"^(int|return)$").unwrap();

    let content_copy: String = token_content.to_string();

    if keyword_rgx.is_match(token_content) {
        return Token::Keyword(lex_keyword(content_copy));
    } else if constant_rgx.is_match(token_content) {
        return Token::Constant(content_copy);
    } else if identifier_rgx.is_match(token_content) {
        return Token::Identifier(content_copy);
    } else {
        match token_content {
            "(" => Some(Token::OpenParenthesis),
            ")" => Some(Token::CloseParenthesis),
            "{" => Some(Token::OpenBrace),
            "}" => Some(Token::CloseBrace),
            ";" => Some(Token::Semicolon),
            "~" => Some(Token::Tilde),
            "-" => Some(Token::Hyphen),
            _ => None,
        }
        .expect(&format!(
            "{token_content} should be one of the known lexical token types"
        ))
    }
}

pub fn lex_contents(src_contents: String) -> Vec<Token> {
    let mut tokens = Vec::new();
    let starting_whitespace_pattern = Regex::new(r"^\s+").unwrap();
    let next_token_pattern = Regex::new(r"^(\w+\b|--)").unwrap();

    let mut contents: String = src_contents.clone();

    while !contents.is_empty() {
        match starting_whitespace_pattern.find(&contents) {
            Some(mat) => {
                contents.drain(mat.range());
            }
            None => {
                // get entire token
                let mat = next_token_pattern.find(&contents);
                let rng = match mat {
                    Some(val) => val.range(),
                    // contents not empty and first char is a boundary. Get the boundary
                    None => 0..1,
                };
                // decice what to do with token
                let token: String = contents.drain(rng).collect();
                let classified_token = classify_token(&token);
                tokens.push(classified_token);
            }
        }
    }
    return tokens;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classification() {
        macro_rules! test_classification {
            ($raw_content:literal, $expected_token_type:expr) => {
                assert_eq!(classify_token($raw_content), $expected_token_type);
            };
        }

        test_classification!("int", Token::Keyword(Keyword::Int));
        test_classification!("main", Token::Identifier("main".to_string()));
        test_classification!("2", Token::Constant("2".to_string()));
        test_classification!("(", Token::OpenParenthesis);
        test_classification!(")", Token::CloseParenthesis);
        test_classification!("{", Token::OpenBrace);
        test_classification!("}", Token::CloseBrace);
        test_classification!(";", Token::Semicolon);
    }

    #[test]
    #[should_panic = "123bar should be one of the known lexical token types"]
    fn panic_for_bad_variable() {
        classify_token("123bar");
    }

    #[test]
    #[should_panic = "-- should be one of the known lexical token types"]
    fn recognize_minus_minus_above_double_negative() {
        lex_contents(
            "
                int main() {
                    return --2;
                }
            "
            .to_string(),
        );
    }

    #[test]
    fn lex_simple_program() {
        let result = lex_contents(
            "

                int main() {
                    return 2;
                }

                
            "
            .to_string(),
        );

        assert_eq!(
            result,
            Vec::from([
                Token::Keyword(Keyword::Int),
                Token::Identifier("main".to_string()),
                Token::OpenParenthesis,
                Token::CloseParenthesis,
                Token::OpenBrace,
                Token::Keyword(Keyword::Return),
                Token::Constant("2".to_string()),
                Token::Semicolon,
                Token::CloseBrace,
            ])
        )
    }

    #[test]
    fn should_lex_nested_unary_ops() {
        let result = lex_contents(
            "

                int main() {
                    return (~(-(-2)));
                }

            "
            .to_string(),
        );

        assert_eq!(
            result,
            Vec::from([
                Token::Keyword(Keyword::Int),
                Token::Identifier("main".to_string()),
                Token::OpenParenthesis,
                Token::CloseParenthesis,
                Token::OpenBrace,
                Token::Keyword(Keyword::Return),
                Token::OpenParenthesis,
                Token::Tilde,
                Token::OpenParenthesis,
                Token::Hyphen,
                Token::OpenParenthesis,
                Token::Hyphen,
                Token::Constant("2".to_string()),
                Token::CloseParenthesis,
                Token::CloseParenthesis,
                Token::CloseParenthesis,
                Token::Semicolon,
                Token::CloseBrace,
            ])
        )
    }
}
