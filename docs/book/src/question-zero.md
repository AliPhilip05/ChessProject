# Question 0: Start with GTE

GTE begins with a small green logo, a known-good Rust implementation, and a passing test. Question 0 is deliberately prefilled: run its check and read what the tests say before you change any code.

## Run the first check

Start the terminal path from the repository root:

```bash
cargo run
```

The TUI runs the first incomplete exercise when it opens. You can also press `R` to run the current check again. The focused command for this exercise is:

```bash
cargo test --manifest-path exercises/Cargo.toml --test intro_0
```

Question 0 should report two passing tests. The first checks that GTE has two entry points. The second checks that the main files are represented in a small component map. The source in `exercises/src/intro_0.rs` is complete; there is no code to insert.

## Read the project map

The exercise's `Component` struct pairs a repository path with a short purpose. The `COMPONENTS` array introduces six subsystem labels. The starter keeps its original broad labels so that the exercise contract stays stable; the implementation is now split into focused child files:

- `src/main.rs` selects the terminal or web entry point.
- The `cli` subsystem starts in `src/cli.rs`; its `input/`, `render/`, runner, state, and progress modules each own one terminal concern.
- The `lessons` subsystem starts in `src/lessons/mod.rs`; its model, catalog, and topic files supply prompts, hints, answers, solution paths, and reference anchors.
- `src/api.rs` exposes archive loading, with HTTP, URL validation, archive orchestration, and conversion under `src/api/`.
- `src/chess.rs` exposes the game model and answer rules; `src/chess/` separates game state, PGN parsing, SAN moves, board setup, and scoring.
- The `web` subsystem starts in `src/web.rs`; its handlers, renderers, HTTP helpers, sessions, and book reader serve the browser app.

The two entry-point assertions mirror `main`:

```rust
pub fn entrypoint(web_mode: bool) -> &'static str {
    if web_mode { "web::serve" } else { "cli::run" }
}
```

`&'static str` means the function returns a string slice stored in the program, not a newly allocated `String`. `Component` derives `PartialEq` and `Eq` so tests can compare complete records. This is the same equality idea used in the next Rust-foundations exercise.

## Follow one request through the app

For the web route, the broad path is:

```text
main.rs (--web)
  -> web::serve
  -> handle_connection
  -> route
  -> load_game
  -> api::random_public_game or ChessGame::demo
  -> ChessGame parses PGN and stores board snapshots
  -> create_session
  -> render_game
```

The terminal path branches from the same `main` function:

```text
main.rs (default)
  -> cli::run
  -> UiState selects an exercise
  -> run_lesson_test invokes one exercises integration-test target
  -> a passing check loads the matching file from solutions/
```

Question 0 gives you the map. The next chapters follow each arrow in detail. Start with [Rust entry points and types](/guide/rust-entry-point), then continue to [Chess.com data](/guide/public-data), [PGN and board state](/guide/board-state), and [the web server](/guide/web-server).

## Use the output as a reading guide

Test names describe behaviors rather than implementation lines. Open the corresponding test under `exercises/tests/`, follow its `#[path]` to the starter under `exercises/src/`, and compare that small behavior with the production module under `src/`. The focused reference implementation under `solutions/` shows one way to satisfy the test.

Each exercise source has a `NOTE:` comment for the expected behavior and a `TODO:` comment marking the requested work. Question 0 is the exception: its TODO asks you to run and read the check because the code is already filled in.

For a file-by-file map, continue to [Codebase Walkthrough — reference.md](/guide/reference#question-zero-project-map).
