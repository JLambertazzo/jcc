use clap::Parser;
use regex::Regex;
use std::fs;

mod ast;
mod ingestion;
mod processing;

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
    let c_program = ingestion::process_program(contents);
    let asm_output = processing::to_asm::program_as_asm(c_program);
    let output_path = Regex::new(r"\.i$")
        .unwrap()
        .replace(input_path, ".s")
        .to_string();
    fs::write(output_path, asm_output).unwrap();
}
