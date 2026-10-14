# Select the newest archives

## Goal

Use a reverse iterator and a limit to select recent archive URLs from a list ordered oldest first.

## Check

Run `archive_api_easy`. It checks newest-first ordering and the zero-limit edge case.

## Hint

The loop already visits the newest entries first. Add each visited URL to the output vector.

## NOTE

This is the easy version of the chess.com data example. Read the NOTE in the Rust source to understand the expected behavior before changing it.

## TODO

Follow the TODO comment in `exercises/src/archive_api_easy.rs`. The exercise tests are independent and can be rerun with the terminal's `R` command.

## Reference

Read [reference.md: chesscom-archive-data](reference.md#chesscom-archive-data) in the codebase walkthrough.
