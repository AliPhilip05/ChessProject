# Normalize an opening name

## Goal

Use whitespace-aware splitting to make an opening name insensitive to spacing and capitalization.

## Check

Run `opening_answer_easy`. It checks normalized case, repeated spaces, and empty input.

## Hint

The word joining line is already done. Convert the joined string to ASCII lowercase before returning it.

## NOTE

This is the easy version of the guess the opening example. Read the NOTE in the Rust source to understand the expected behavior before changing it.

## TODO

Follow the TODO comment in `exercises/src/opening_answer_easy.rs`. The exercise tests are independent and can be rerun with the terminal's `R` command.

## Reference

Read [reference.md: opening-answers](reference.md#opening-answers) in the codebase walkthrough.
