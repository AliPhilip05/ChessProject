# Match an accepted opening alias

## Goal

Implement a complete case- and whitespace-insensitive matcher for a canonical opening name and its aliases.

## Check

Run `opening_answer_hard`. It checks aliases, normalization, empty guesses, and a wrong answer.

## Hint

Normalize the guess once, reject it if blank, and compare it with each normalized accepted name.

## NOTE

This is the hard version of the guess the opening example. Read the NOTE in the Rust source to understand the expected behavior before changing it.

## TODO

Follow the TODO comment in `exercises/src/opening_answer_hard.rs`. The exercise tests are independent and can be rerun with the terminal's `R` command.

## Reference

Read [reference.md: opening-answers](reference.md#opening-answers) in the codebase walkthrough.
