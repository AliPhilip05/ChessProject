# Parse the requested mode

## Goal

Implement a complete parser from command-line mode names into a safe enum value.

## Check

Run `rust_basics_hard`. It checks exact names, case sensitivity, and unknown input.

## Hint

Use a `match` on `argument`; return `Some` for the two recognized names and `None` otherwise.

## NOTE

This is the hard version of the rust foundations example. Read the NOTE in the Rust source to understand the expected behavior before changing it.

## TODO

Follow the TODO comment in `exercises/src/rust_basics_hard.rs`. The exercise tests are independent and can be rerun with the terminal's `R` command.

## Reference

Read [reference.md: rust-types-and-mode-selection](reference.md#rust-types-and-mode-selection) in the codebase walkthrough.
