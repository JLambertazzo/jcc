use regex::Regex;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Identifier(String),
    Constant(String),
    Keyword(String),
    OpenParenthesis,
    CloseParenthesis,
    OpenBrace,
    CloseBrace,
    Semicolon,
    Tilde,
    Hyphen,
    Plus,
    Star,
    Slash,
    Modulo,
    Ampersand,
    Pipe,  // |
    Caret, // ^
    OpenAngleBracket,
    CloseAngleBracket,
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
    Plus,
    Star,
    Slash,
    Modulo,
    Ampersand,
    Pipe,  // |
    Caret, // ^
    OpenAngleBracket,
    CloseAngleBracket,
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
        Token::Plus => TokenKind::Plus,
        Token::Star => TokenKind::Star,
        Token::Slash => TokenKind::Slash,
        Token::Modulo => TokenKind::Modulo,
        Token::Ampersand => TokenKind::Ampersand,
        Token::Pipe => TokenKind::Pipe,
        Token::Caret => TokenKind::Caret,
        Token::OpenAngleBracket => TokenKind::OpenAngleBracket,
        Token::CloseAngleBracket => TokenKind::CloseAngleBracket,
    }
}

pub struct LanguageSpec {
    pub keyword_rgx: Regex,
    pub constant_rgx: Regex,
    pub identifier_rgx: Regex,
}

fn classify_token(token_content: &str, spec: &LanguageSpec) -> Token {
    /*
     * Given regex patterns for each token type.
     * Any other value you encounter at this point should be handled by exact match
     */

    let content_copy: String = token_content.to_string();

    if spec.keyword_rgx.is_match(token_content) {
        return Token::Keyword(content_copy);
    } else if spec.constant_rgx.is_match(token_content) {
        return Token::Constant(content_copy);
    } else if spec.identifier_rgx.is_match(token_content) {
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
            "+" => Some(Token::Plus),
            "*" => Some(Token::Star),
            "/" => Some(Token::Slash),
            "%" => Some(Token::Modulo),
            "&" => Some(Token::Ampersand),
            "|" => Some(Token::Pipe),
            "^" => Some(Token::Caret),
            "<" => Some(Token::OpenAngleBracket),
            ">" => Some(Token::CloseAngleBracket),
            _ => None,
        }
        .expect(&format!(
            "{token_content} should be one of the known lexical token types"
        ))
    }
}

pub fn lex_contents(src_contents: String, spec: &LanguageSpec) -> Vec<Token> {
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
                let classified_token = classify_token(&token, spec);
                tokens.push(classified_token);
            }
        }
    }
    return tokens;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_c_spec() -> LanguageSpec {
        LanguageSpec {
            identifier_rgx: Regex::new(r"^[a-zA-Z_]\w*\b$").unwrap(),
            constant_rgx: Regex::new(r"^[0-9]+\b$").unwrap(),
            keyword_rgx: Regex::new(r"^(int|return|void)$").unwrap(),
        }
    }

    #[test]
    fn test_classification() {
        macro_rules! test_classification {
            ($raw_content:literal, $expected_token_type:expr) => {
                assert_eq!(
                    classify_token($raw_content, &get_c_spec()),
                    $expected_token_type
                );
            };
        }

        test_classification!("int", Token::Keyword(String::from("int")));
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
        classify_token("123bar", &get_c_spec());
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
            &get_c_spec(),
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
            &get_c_spec(),
        );

        assert_eq!(
            result,
            Vec::from([
                Token::Keyword(String::from("int")),
                Token::Identifier("main".to_string()),
                Token::OpenParenthesis,
                Token::CloseParenthesis,
                Token::OpenBrace,
                Token::Keyword(String::from("return")),
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
            &get_c_spec(),
        );

        assert_eq!(
            result,
            Vec::from([
                Token::Keyword(String::from("int")),
                Token::Identifier("main".to_string()),
                Token::OpenParenthesis,
                Token::CloseParenthesis,
                Token::OpenBrace,
                Token::Keyword(String::from("return")),
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

    #[test]
    fn should_lex_multiple_binary_ops() {
        let result = lex_contents(
            "

                int main() {
                    return (1 + 2) * (4 - 3) / (3 % 2);
                }

            "
            .to_string(),
            &get_c_spec(),
        );

        assert_eq!(
            result,
            Vec::from([
                Token::Keyword(String::from("int")),
                Token::Identifier("main".to_string()),
                Token::OpenParenthesis,
                Token::CloseParenthesis,
                Token::OpenBrace,
                Token::Keyword(String::from("return")),
                Token::OpenParenthesis,
                Token::Constant(String::from("1")),
                Token::Plus,
                Token::Constant(String::from("2")),
                Token::CloseParenthesis,
                Token::Star,
                Token::OpenParenthesis,
                Token::Constant(String::from("4")),
                Token::Hyphen,
                Token::Constant(String::from("3")),
                Token::CloseParenthesis,
                Token::Slash,
                Token::OpenParenthesis,
                Token::Constant(String::from("3")),
                Token::Modulo,
                Token::Constant(String::from("2")),
                Token::CloseParenthesis,
                Token::Semicolon,
                Token::CloseBrace,
            ])
        )
    }
}
