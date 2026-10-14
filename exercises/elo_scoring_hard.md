# Evaluate a possibly missing guess

## Goal

Write a complete scoring function that handles missing ratings and applies an inclusive tolerance.

## Check

Run `elo_scoring_hard`. It checks both missing-value cases, an exact boundary, and an out-of-range guess.

## Hint

Use `?` on the two optional ratings, reject a negative tolerance, then compare the absolute error.

## NOTE

This is the hard version of the guess the elo example. Read the NOTE in the Rust source to understand the expected behavior before changing it.

## TODO

Follow the TODO comment in `exercises/src/elo_scoring_hard.rs`. The exercise tests are independent and can be rerun with the terminal's `R` command.

## Reference

Read [reference.md: elo-scoring](reference.md#elo-scoring) in the codebase walkthrough.
