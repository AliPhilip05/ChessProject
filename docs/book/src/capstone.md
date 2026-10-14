# Capstone: Rebuild a Small Guessing Flow

The capstone asks you to connect the ideas from the previous exercises in `exercises/src/capstone.rs`. Its starter defines a `GameRecord` with a move list, both ratings, and an opening name. The supplied example in `exercises/capstone.md` is deterministic and does not need a Chess.com request.

## Build the flow in layers

1. Implement `clamp_ply` so a requested position cannot exceed the move count. The starting position is zero; the snapshot count is one larger than the ply count.
2. Implement the difficulty tolerance: Easy ±200, Medium ±100, Hard ±50.
3. Score only the requested player, or require both submitted guesses when the target is `both`. A missing guess is not a correct guess.
4. Normalize the opening guess and expected value by trimming, collapsing whitespace, and ignoring capitalization.
5. Run the `capstone` exercise target. Use its failing assertion to identify which behavior is still missing.

## Connect it to the app

The capstone is intentionally a small pure-Rust model, not a second copy of the application. Once the functions work, trace their production counterparts:

- `api.rs` turns a public record into a `ChessGame`;
- `chess.rs` parses the game and owns the shared matching and scoring rules;
- `web.rs` stores the selected mode and answers in a session;
- `render_game` updates the board and feedback after each GET route.

To build a larger version from scratch, add an archive loader, a sequence of board snapshots, a move index, a selected side, an answer form, and a result view. Keep those pieces separate so each function can be tested.

## Read the solution and full reference

When the capstone tests pass, GTE opens `solutions/capstone.rs`. Compare the selected-side branch and tolerance handling against the exercise test instead of copying the code without reading it. The app's end-to-end lifecycle is documented at [Capstone and End-to-End Flow](/guide/reference#capstone-and-end-to-end-flow).
