# PGN and Board State

Chess.com returns a PGN string for an archived game. `src/chess/game.rs` coordinates that conversion into metadata, accepted SAN moves, and a position snapshot after every accepted half-move. The web layer selects one of those snapshots instead of replaying from the beginning on every page request.

## `ChessGame` is the shared model

`ChessGame` holds player names; optional ratings; opening, ECO, result, time control, date, and URL; a `demo` flag; the SAN move list; and private board positions. This means both web modes can render the same game but interpret its metadata differently.

`ChessGame::from_pgn` is the simple constructor. `ChessGame::with_metadata` accepts values from an archive record and uses them in preference to nonempty PGN metadata. It falls back to PGN headers, then readable defaults such as `White`, `Unknown`, or `Unclassified opening`. `ChessGame::demo` constructs a local Italian Game with ratings and ten moves.

The player ratings are `Option<i32>` because API records may omit them. `moves` is owned as `Vec<String>`, and `positions` stores a starting board plus one board after each accepted move.

## Read PGN headers separately from moves

PGN has a header section, then a move section. `pgn_headers` accepts lines shaped like `[WhiteElo "1180"]`, splits the key from the quoted value, and unescapes quote characters. The private `header` helper returns a borrowed value; `parse_rating` only accepts a positive integer.

The move parser makes a clean token stream in two stages:

1. `pgn_move_tokens` skips header lines, brace comments, semicolon comments, and nested parenthesized variations. It returns whitespace-separated tokens from the remaining move text.
2. `strip_move_number` removes prefixes such as `1.` and `12...`. `with_metadata` filters game results, NAG tokens (`$5`), and the standalone en-passant marker before applying a move.

This keeps the parser understandable, but PGN has more cases than this small implementation handles. The parser is deliberately not a complete PGN standards library.

For example, a PGN fragment such as:

```text
[White "QuietKnight"]
[WhiteElo "1180"]
1. e4 {center control} e5 2. Nf3 (2. Bc4) Nc6 1/2-1/2
```

produces headers for the player and rating, then a main-line token sequence `1.`, `e4`, `e5`, `2.`, `Nf3`, `Nc6`, and the result. The brace comment and parenthesized variation are removed. `with_metadata` strips the move-number tokens and result, then attempts the four moves in order. The alternate `Bc4` line never becomes part of the board history.

## Board coordinates

The position is a 64-element array. Index zero is the top-left square, files increase across a row, and ranks move down the rows. White pieces are uppercase (`'P'`, `'K'`); Black pieces are lowercase (`'p'`, `'k'`); `'.'` is empty.

```text
index = row * 8 + column
a8 -> 0      h8 -> 7
a1 -> 56     h1 -> 63
```

`starting_position` fills the first and eighth ranks with the back-rank pieces, puts pawns beside them, and leaves the middle four ranks empty.

## Apply a supported SAN move

`apply_san` removes trailing check, mate, and annotation marks. Castling is handled as a special case by moving the king and rook. Other SAN tokens are split into destination file/rank, piece kind, optional disambiguation, capture marker, and promotion.

The function scans the current board for a piece of the required color and kind. `disambiguation_matches` restricts that scan when SAN includes a source file or rank. `piece_can_move` checks the movement shape, side-to-move direction, capture occupancy, and pawn double move. Sliding pieces also call `path_is_clear`. When an origin is found, the board is mutated, including en passant capture and promotion handling, and the function returns `true`.

If no matching piece can move to the destination, `apply_san` returns `false`; `with_metadata` does not append a move or snapshot and does not change the side to move. This is a lightweight reconstruction routine, not a legality adjudicator. It does not fully validate check, pins, castling rights, every ambiguous move, all promotion variants, or arbitrary starting FEN positions.

The origin search is intentionally explicit. The SAN destination gives a file and rank; the prefix says which piece is moving and may constrain its origin. The parser scans all 64 squares, filters by color and kind, then asks `disambiguation_matches` and `piece_can_move`. For a rook, bishop, or queen, `path_is_clear` checks every square between origin and destination. Once it finds a candidate, the move mutates the one board array. A capture replaces the destination; an en-passant pawn capture additionally clears the adjacent pawn square; promotion changes the arriving pawn to the requested piece (or a queen if no piece was specified).

The parser returns a boolean rather than a board: it mutates the board only on success and lets the caller decide whether this token belongs in the public `moves` vector. The caller then appends the exact SAN string and a copy of the post-move board together, preserving the invariant `positions.len() == moves.len() + 1`.

## Snapshots make navigation simple

`with_metadata` begins with `positions = vec![board]`. After each accepted move, it appends the updated board and the SAN token. Thus the initial position is ply zero and the number of snapshots is `ply_count() + 1`.

```rust
let game = ChessGame::demo();
let starting_board = game.position_at(0);
let board_after_move = game.position_at(1);
assert_eq!(starting_board.len(), 64);
assert_eq!(board_after_move.len(), 64);
```

`position_at` clamps an out-of-range request to the last snapshot. `ply_count` uses saturating subtraction so an empty position list cannot underflow. The web route also clamps requested plies to the current mode's maximum.

The `pgn_board` exercise reduces this path to two checks: filtering a sample move string and returning `move_count + 1` positions. Continue to [The Two Game Modes](/guide/game-modes) to see how the board is used.

The board is `Copy` because `[char; 64]` is a fixed-size array of copyable values. `positions.push(board)` therefore saves a snapshot without transferring away the mutable working board. A game with 20 accepted half-moves has 21 snapshots: index 0 is the initial position, index 1 is after White's first move, and index 20 is after Black's tenth move. A half-move is also called a ply; it is one player's turn, not a complete White-and-Black move pair.
