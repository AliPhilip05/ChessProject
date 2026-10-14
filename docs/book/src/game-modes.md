# The Two Game Modes

Both modes use the same `ChessGame`, session store, board, move controls, and game metadata. A `GameMode` value selects which answer panel appears and which position limit applies.

## Session state

In `src/web.rs`, a `Session` contains:

- the parsed `ChessGame`;
- `mode`, `difficulty`, and target side;
- the current `ply`;
- a note explaining demo or fallback data;
- answer `feedback` and a `revealed` flag.

`create_session` generates an id and stores the session. Elo rounds start at ply zero. Opening rounds start after up to eight plies. Both modes can use the local demo or archive game. The visible maximum is determined by `max_ply`.

## Guess the Elo

The Elo panel hides the real ratings until an answer is submitted. The form sends one numeric input for each selected side: both, White only, or Black only. `render_elo_panel` omits the unselected side's input so a learner doesn't enter a guess that will be ignored.

`submit_elo_guess` checks that the session is an Elo round. `score_one` trims and parses the input, accepts ratings from 0 through 4000, handles a missing actual rating, computes the absolute error, and checks it against `rating_tolerance`:

```rust
let tolerance = match difficulty.to_ascii_lowercase().as_str() {
    "easy" => 200,
    "hard" => 50,
    _ => 100, // Medium is the default.
};
let correct = (guess - actual).abs() <= tolerance;
```

The boundary is inclusive. A Medium guess 100 points above or below the actual rating succeeds; 101 points does not. Easy is ±200. Hard is ±50. `rating_is_close` owns this rule so the browser and focused exercise can share the same behavior.

After submission, the session stores one feedback line per selected player and sets `revealed`. `render_player` then shows each actual rating or reports that the record did not contain one. A missing rating is never treated as a perfect score or zero.

## Guess the Opening

Opening mode asks for the name associated with the loaded game's metadata. It shows the move sequence from up to eight plies and starts with the board at the same point in the game. The learner can still use the visible move controls within that limit.

`normalize_answer` collapses whitespace and lowercases a string. `opening_matches` rejects an empty normalized guess, then compares the normalized guess and expected name:

```rust
assert!(opening_matches("  ITALIAN   GAME ", "Italian Game"));
```

The handler reveals the expected opening for a correct or incorrect nonempty answer. An empty answer receives a prompt instead.

## One renderer, mode-specific panels

`render_game` builds shared content first: player rows, board, move controls, move list, result, time control, date, and ECO. It then chooses `render_elo_panel` or `render_opening_panel` for the aside. This makes layout and move navigation reusable while keeping the two scoring forms separate.

`render_move_list` marks the active ply. `update_move` clamps the requested value and redirects back to the same session. Elo can move through the whole game; opening mode limits navigation to the introductory sequence. Submitting a move clears old feedback.

## Exercises that isolate the rules

`elo_scoring` tests threshold mapping and inclusive absolute-error boundaries. `opening_answer` tests capitalization, repeated whitespace, incorrect answers, and empty input. The capstone exercises put both behaviors into a small sample `GameRecord` flow.

Continue to [the web server chapter](/guide/web-server) for the request and render path, or read the [complete mode implementation notes](/guide/reference#elo-scoring).
