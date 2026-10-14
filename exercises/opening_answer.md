# Normalize opening guesses

## Goal

Compare opening names without case sensitivity or accidental leading/trailing spaces.

## Check

Compare `  ITALIAN GAME ` against `Italian Game`.

## Hint

Trim both strings and lowercase them before comparing.

## NOTE

The expected implementation matches equivalent opening names regardless of capitalization or repeated whitespace.

## TODO

Insert the normalization and comparison in `exercises/src/opening_answer.rs`, beneath its `TODO:` comment.

## Reference

Read [reference.md: Opening answers](reference.md#opening-answers).
