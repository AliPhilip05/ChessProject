# Recognize a move token

## Goal

Separate ordinary SAN tokens from move numbers, game results, and annotation tokens.

## Check

Run `pgn_board_easy`. The examples include pawn moves, castling, numbering, results, and NAG markers.

## Hint

A move is not blank, a move number, a result marker, or a token beginning with `$`.

## NOTE

This is the easy version of the pgn and board state example. Read the NOTE in the Rust source to understand the expected behavior before changing it.

## TODO

Follow the TODO comment in `exercises/src/pgn_board_easy.rs`. The exercise tests are independent and can be rerun with the terminal's `R` command.

## Reference

Read [reference.md: pgn-parsing-and-board-positions](reference.md#pgn-parsing-and-board-positions) in the codebase walkthrough.
