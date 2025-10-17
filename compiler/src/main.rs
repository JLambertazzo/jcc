use clap::Parser;
use regex::Regex;
use std::fs;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    lex: bool,
    #[arg(short, long)]
    parse: bool,
    #[arg(short, long)]
    codegen: bool,
    filepath: String,
}

enum TokenType {
    Identifier,
    Constant,
    Keyword,
    OpenParenthesis,
    CloseParenthesis,
    OpenBrace,
    CloseBrace,
    Semicolon,
}

struct Token {
    token_type: TokenType,
    content: String,
}

fn classify_token(token_content: &String) -> Token {
    /*
     * Given regex patterns for each token type.
     * Any other value you encounter at this point should be handled by exact match
     */

    let identifier_rgx = Regex::new(r"^[a-zA-Z_]\w*\b$").unwrap();
    let constant_rgx = Regex::new(r"^[0-9]+\b$").unwrap();
    let keyword_rgx = Regex::new(r"^(int|void|return)$").unwrap();

    let content_copy: String = token_content.into();

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
        let found_type: Option<TokenType> = match &*token_content.as_str() {
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

fn lex_contents(src_contents: &String) -> Vec<Token> {
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

fn main() {
    let cli = Cli::parse();
    let contents =
        fs::read_to_string(cli.filepath).expect("Should have been able to read the file");
    lex_contents(&contents);
}
