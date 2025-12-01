# JCC

Julien's C Compiler

Implementation of the C compiler outlined in [Writing a C Compiler](https://nostarch.com/writing-c-compiler) by Nora Sandler

## Usage

```bash
./jcc [-S][-h | --help][--lex | --parser | --codegen] path/to/code.c
```

## Project Structure

* `./src/ast` defines the AST structures for C and assembly
* `./src/ingestion` turns preprocessed C into a C AST
* `./src/processing` turns a C AST into an assembly AST
* `./src/emission` transforms an assembly AST into assembly code
