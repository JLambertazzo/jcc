# JCC

Julien's C Compiler

Implementation of the C compiler outlined in [Writing a C Compiler](https://nostarch.com/writing-c-compiler) by Nora Sandler

## Usage

```bash
./jcc [-S][-h | --help][--lex | --parser | --codegen] path/to/code.c
```

## Project Structure

* `./src/ast` defines the AST structures for C
* `./src/ingestion` turns preprocessed C into a C AST
* `./src/emission` writes C to ASM output
