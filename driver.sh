#!/bin/bash

# Parse cli options
OUTPUT_ASSEMBLY=0

vars=$(getopt -o S -l "lex,parse,codegen" -- "$@")
for var in $vars
do
  case "$var" in
    "-S")
      OUTPUT_ASSEMBLY=1
      ;;
    "--") # marks end of opts. break from loop
      break
      ;;
    *)
      echo Valid option $var is not yet handled
      ;;
  esac
done

# Driver implementation for "Writing a C Compiler by Nora Sandler".
# Must accept as single argument - a path to a c source file in $1

INPUT_FILE=$1
BASE_PATH=$(echo $INPUT_FILE | sed -E 's/\.\w*$//')

# Preprocess input file
PREPROC_PATH="${BASE_PATH}.i"
gcc -E -P $INPUT_FILE -o $PREPROC_PATH

# Compile the preprocessed source code - stubbed out for now
ASSEMBLY_PATH="${BASE_PATH}.s"
echo Please implement compiler
rm $PREPROC_PATH

# If -S provided, stop after assembly output
if [ $OUTPUT_ASSEMBLY -eq 1 ]; then
  echo compiled assembly to $ASSEMBLY_PATH
  exit 0
fi

# Assemble and link assembly file
# BASE_PATH is the same as the path where the linked binary should live
gcc $ASSEMBLY_PATH -o $BASE_PATH
rm $ASSEMBLY_PATH

echo compiled executable to $BASE_PATH
exit 0

