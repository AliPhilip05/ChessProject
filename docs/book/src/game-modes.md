# The Two Game Modes

The app loads one `ChessGame`, then associates it with a mode, selected difficulty, target player, current ply, and feedback state.

## Guess the Elo

The board starts at ply zero and hides both ratings. Choose a public game link or a game from a player's archive. Choose White, Black, or both; choose Easy (±200), Medium (±100), or Hard (±50); then submit a numeric guess. The page reveals the available recorded ratings, the absolute error, and whether each requested guess landed within the chosen tolerance.

If only one side is selected, the other player's input and score are omitted. When a game record lacks a rating, the app says so instead of treating it as zero.

## Guess the Opening

The board starts after up to eight plies. Enter the opening name from the game metadata. `normalize_answer` trims whitespace, collapses repeated whitespace, and lowercases the text before comparison, so capitalization does not matter.

## Demo data

Both modes can use the built-in Italian Game fixture. It has names, ratings, an opening, a result, and enough moves to explore the board without contacting Chess.com.

Continue to [Terminal Lessons](/guide/terminal-learning) to learn how the Rust code is practiced.
