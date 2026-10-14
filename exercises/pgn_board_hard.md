# Navigate a saved position

## Goal

Implement a complete bounded move-selection function over saved board snapshots.

## Check

Run `pgn_board_hard`. It verifies the starting position, a middle ply, the final ply, and requests beyond the game.

## Hint

The snapshots include index zero for the initial board. Clamp the requested ply to the last valid index before indexing.

## NOTE

This is the hard version of the pgn and board state example. Read the NOTE in the Rust source to understand the expected behavior before changing it.

## TODO

Follow the TODO comment in `exercises/src/pgn_board_hard.rs`. The exercise tests are independent and can be rerun with the terminal's `R` command.

## Reference

Read [reference.md: pgn-parsing-and-board-positions](reference.md#pgn-parsing-and-board-positions) in the codebase walkthrough.
