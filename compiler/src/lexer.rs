use regex::Regex;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Identifier,
    Constant,
    Keyword,
    OpenParenthesis,
    CloseParenthesis,
    OpenBrace,
    CloseBrace,
    Semicolon,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub content: String,
}

fn classify_token(token_content: &str) -> Token {
    /*
     * Given regex patterns for each token type.
     * Any other value you encounter at this point should be handled by exact match
     */

    let identifier_rgx = Regex::new(r"^[a-zA-Z_]\w*\b$").unwrap();
    let constant_rgx = Regex::new(r"^[0-9]+\b$").unwrap();
    let keyword_rgx = Regex::new(r"^(int|void|return)$").unwrap();

    let content_copy: String = token_content.to_string();

    if keyword_rgx.is_match(token_content) {
        return Token {
            token_type: TokenType::Keyword,
            content: content_copy,
        };
    } else if constant_rgx.is_match(token_content) {
        return Token {
            token_type: TokenType::Constant,
            content: content_copy,
        };
    } else if identifier_rgx.is_match(token_content) {
        return Token {
            token_type: TokenType::Identifier,
            content: content_copy,
        };
    } else {
        let found_type: Option<TokenType> = match token_content {
            "(" => Some(TokenType::OpenParenthesis),
            ")" => Some(TokenType::CloseParenthesis),
            "{" => Some(TokenType::OpenBrace),
            "}" => Some(TokenType::CloseBrace),
            ";" => Some(TokenType::Semicolon),
            _ => None,
        };
        return Token {
            token_type: found_type.unwrap(),
            content: content_copy,
        };
    }
}

pub fn lex_contents(src_contents: String) -> Vec<Token> {
    let mut tokens = Vec::new();
    let starting_whitespace_pattern = Regex::new(r"^\s+").unwrap();
    let next_token_pattern = Regex::new(r"^\w*\b").unwrap();

    let mut contents: String = src_contents.clone();

    while !contents.is_empty() {
        if starting_whitespace_pattern.is_match(&contents) {
            // trim starting whitespace
            let mat = starting_whitespace_pattern.find(&contents).unwrap();
            contents.drain(mat.range());
        } else {
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
    return tokens;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classification() {
        macro_rules! test_classification {
            ($raw_content:literal, $expected_token_type:expr) => {
                assert_eq!(
                    classify_token($raw_content),
                    Token {
                        token_type: $expected_token_type,
                        content: $raw_content.to_string()
                    }
                );
            };
        }

        test_classification!("int", TokenType::Keyword);
        test_classification!("main", TokenType::Identifier);
        test_classification!("2", TokenType::Constant);
        test_classification!("(", TokenType::OpenParenthesis);
        test_classification!(")", TokenType::CloseParenthesis);
        test_classification!("{", TokenType::OpenBrace);
        test_classification!("}", TokenType::CloseBrace);
        test_classification!(";", TokenType::Semicolon);
    }

    #[test]
    #[should_panic]
    fn panic_for_bad_variable() {
        classify_token("123bar");
    }

    #[test]
    fn lex_simple_program() {
        macro_rules! tok {
            ($raw_content:literal, $token_type:expr) => {
                Token {
                    token_type: $token_type,
                    content: $raw_content.to_string(),
                }
            };
        }
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
                tok!("int", TokenType::Keyword),
                tok!("main", TokenType::Identifier),
                tok!("(", TokenType::OpenParenthesis),
                tok!(")", TokenType::CloseParenthesis),
                tok!("{", TokenType::OpenBrace),
                tok!("return", TokenType::Keyword),
                tok!("2", TokenType::Constant),
                tok!(";", TokenType::Semicolon),
                tok!("}", TokenType::CloseBrace),
            ])
        )
    }
}
