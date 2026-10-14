# Turn PGN into a move review

## Goal

Explain how to preserve the board at each move so a user can step backward and forward.

## Check

Given a starting position and an ordered SAN sequence, choose a data structure that lets a requested ply render without replaying every move.

## Hint

The position list has one more entry than the move list.

## NOTE

The expected implementation returns SAN moves without PGN headers, move numbers, comments, or result markers, then counts a starting snapshot plus one snapshot per move.

## TODO

Insert the parser and position-count logic in `exercises/src/pgn_board.rs`, where each `TODO:` comment marks a function.

## Reference

Read [reference.md: PGN parsing and board positions](reference.md#pgn-parsing-and-board-positions).
