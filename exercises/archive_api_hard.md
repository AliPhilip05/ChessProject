# Find the newest usable month

## Goal

Write a complete function that finds the newest archive month containing usable games.

## Check

Run `archive_api_hard`. It checks empty recent months, a valid month, and the no-games case.

## Hint

Scan the month slices in reverse order and return the first slice that is not empty.

## NOTE

This is the hard version of the chess.com data example. Read the NOTE in the Rust source to understand the expected behavior before changing it.

## TODO

Follow the TODO comment in `exercises/src/archive_api_hard.rs`. The exercise tests are independent and can be rerun with the terminal's `R` command.

## Reference

Read [reference.md: chesscom-archive-data](reference.md#chesscom-archive-data) in the codebase walkthrough.
