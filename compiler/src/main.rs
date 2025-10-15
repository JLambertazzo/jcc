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

fn main() {
    let starting_whitespace_pattern = Regex::new(r"^\s+").unwrap();
    let next_token_pattern = Regex::new(r"^.*\b").unwrap();

    let cli = Cli::parse();
    let mut contents =
        fs::read_to_string(cli.filepath).expect("Should have been able to read the file");
    while !contents.is_empty() {
        if starting_whitespace_pattern.is_match(&contents) {
            // trim starting whitespace
            let mat = starting_whitespace_pattern.find(&contents).unwrap();
            contents.drain(mat.range());
        } else {
            // get entire token
            let mat = next_token_pattern.find(&contents).unwrap();
            let token: String = contents.drain(mat.range()).collect();
            // decice what to do with token
        }
    }
    println!("Hello, world! {:?}", cli.lex);
}
