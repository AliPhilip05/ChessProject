# Finish a guessing round

## Goal

Build a small round model and implement the entire state transition that clamps navigation and reveals a normalized correct answer.

## Check

Run `capstone_hard`. The integration test checks the completed round and a wrong answer that must stay hidden.

## Hint

Clamp to `moves.len()`. Normalize both opening names by splitting whitespace, joining with one space, and lowercasing.

## NOTE

This is the hard version of the capstone example. Read the NOTE in the Rust source to understand the expected behavior before changing it.

## TODO

Follow the TODO comment in `exercises/src/capstone_hard.rs`. The exercise tests are independent and can be rerun with the terminal's `R` command.

## Reference

Read [reference.md: capstone-and-end-to-end-flow](reference.md#capstone-and-end-to-end-flow) in the codebase walkthrough.
