# Trace a game mode to its module

## Goal

Use a small enum and an exhaustive match to map each app mode to the Rust module that owns it.

## Check

Run the focused `codebase_medium` integration test. It checks both variants and relies on equality traits to compare enum values.

## Hint

Match each `GameMode` variant separately and return the stable module path as a string slice.

## NOTE

This is the medium version of the project orientation example. Read the NOTE in the Rust source to understand the expected behavior before changing it.

## TODO

Follow the TODO comment in `exercises/src/codebase_medium.rs`. The exercise tests are independent and can be rerun with the terminal's `R` command.

## Reference

Read [reference.md: question-zero-project-map](reference.md#question-zero-project-map) in the codebase walkthrough.
