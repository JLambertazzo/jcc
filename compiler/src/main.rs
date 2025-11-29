use clap::Parser;
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
    let contents =
        fs::read_to_string(cli.filepath).expect("Should have been able to read the file");
    let tokens = lexer::lex_contents(&contents);
    let assembly = parser::parse(tokens);
    fs::write("test.s", assembly).unwrap();
}
