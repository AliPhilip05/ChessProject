# PGN and Board State

Chess.com game records include PGN, a text format that stores headers and moves. The app turns that text into the names and ratings shown in the interface, a move list, and a sequence of board snapshots.

## PGN headers and moves

A header such as `[WhiteElo "1180"]` identifies metadata. The move text uses Standard Algebraic Notation (SAN), for example `1. e4 e5 2. Nf3 Nc6`.

`chess.rs` removes headers, comments, variation branches, move numbers, and result markers before applying the supported SAN tokens. It starts with the standard chess position and records a board after each accepted ply.

## Position indexing

Position zero is the starting board. After White's first move the board is at ply one; after Black replies it is at ply two. With *n* accepted moves, the snapshot list has *n + 1* positions. The web controls clamp navigation so a requested ply cannot go below zero or beyond the final visible ply.

## A learning renderer, not a full chess engine

The prototype handles common piece moves, captures, pawn moves, castling, and promotion. It does not fully validate check, pins, every legality condition, or arbitrary starting FEN positions. It is intended to illustrate parsing and state snapshots, not to adjudicate chess games.

Continue with [The Two Game Modes](/guide/game-modes), which use these positions.
