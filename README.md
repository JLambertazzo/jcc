# JCC

Julien's C Compiler

Implementation of the C compiler outlined in [Writing a C Compiler](https://nostarch.com/writing-c-compiler) by Nora Sandler

## Usage

```bash
./jcc [-S][-h | --help][--lex | --parser | --codegen] path/to/code.c
```

`--lex`, `--parser`, and `--codegen` are not yet implemented. Still need to be forwarded to rust core logic.

## Project Structure

Logic is separated by language in separate folders. Tacky folder only defines structure and will eventually expose optimization logic.

* `./src/c` processes raw text into a C AST and emits a TACKY AST
* `./src/tacky` only defines the AST, does not currently expose logic
* `./src/asm` turns TACKY AST into an ASM AST and emits code from an ASM AST

## Testing Structure

Two commands cover testing for the compiler. Both are run in CI.

```bash
cargo test # runs unit & end-to-end defined in this repository
./run-tests.sh # runs author-defined test
```

* `./tests` contains end-to-end tests running the cli against fixtures
  * will be removed in favour of `run-tests.sh`
* unit tests defined directly in the files
* `./run-tests.sh` runs the end-to-end tests defined by the book's author
  * testing script is defined in submodule `writing-a-c-compiler-tests`
