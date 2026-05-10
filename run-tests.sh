LAST_COMPLETED_CHAPTER=4
CHAPTER_IN_PROGRESS=5
STAGE_IN_PROGRESS="parse"

echo "Running full test suite from chapter $LAST_COMPLETED_CHAPTER..."
./writing-a-c-compiler-tests/test_compiler ./jcc --chapter $LAST_COMPLETED_CHAPTER --bitwise || { echo "Previous chapters' tests failed."; exit 1; }
if [ $LAST_COMPLETED_CHAPTER -ne $CHAPTER_IN_PROGRESS ]; then
  echo "Running test suite for chapter in progress $CHAPTER_IN_PROGRESS up to stage $STAGE_IN_PROGRESS..."
  ./writing-a-c-compiler-tests/test_compiler ./jcc --chapter $CHAPTER_IN_PROGRESS --bitwise --stage $STAGE_IN_PROGRESS
fi
