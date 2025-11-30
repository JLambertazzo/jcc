use clap::Parser;
use regex::Regex;
use std::fs;

mod lexer;
mod parser;

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

fn main() {
    let cli = Cli::parse();
    let input_path = cli.filepath.as_str();
    let contents = fs::read_to_string(input_path)
        .expect(&format!("Failed to read input file {:?}", input_path));
    let tokens = lexer::lex_contents(contents);
    let assembly = parser::parse(tokens);
    let output_path = Regex::new(r"\.i$")
        .unwrap()
        .replace(input_path, ".s")
        .to_string();
    fs::write(output_path, assembly).unwrap();
}
