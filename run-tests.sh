LAST_COMPLETED_CHAPTER=3
CHAPTER_IN_PROGRESS=4

./writing-a-c-compiler-tests/test_compiler ./jcc --chapter $LAST_COMPLETED_CHAPTER --bitwise || { echo "Previous chapters' tests failed."; exit 1; }
./writing-a-c-compiler-tests/test_compiler ./jcc --chapter $CHAPTER_IN_PROGRESS --bitwise --stage tacky
