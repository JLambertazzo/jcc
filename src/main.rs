use clap::Parser;
use regex::Regex;
use std::{fs, process};

mod asm;
mod c;
mod core;
mod tacky;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    lex: bool,
    #[arg(short, long)]
    parse: bool,
    #[arg(short, long)]
    tacky: bool,
    #[arg(short, long)]
    codegen: bool,
    filepath: String,
}

fn main() {
    let cli = Cli::parse();
    let input_path = cli.filepath.as_str();
    let contents = fs::read_to_string(input_path)
        .expect(&format!("Failed to read input file {:?}", input_path));
    let c_program = c::process_program(contents, cli.lex);
    if cli.parse {
        process::exit(0);
    }
    let tacky_program = c::to_tacky::translate_program(c_program);
    if cli.tacky {
        process::exit(0);
    }
    let asm_program = asm::tacky_program_to_asm_code(tacky_program);
    if cli.codegen {
        process::exit(0);
    }
    let asm_output = asm::to_code::asm_program_to_string(asm_program);
    let output_path = Regex::new(r"\.i$")
        .unwrap()
        .replace(input_path, ".s")
        .to_string();
    fs::write(output_path, asm_output).unwrap();
}
