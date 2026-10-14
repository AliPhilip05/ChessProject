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

The exercise's `Component` struct pairs a repository path with a short purpose. The `COMPONENTS` array describes the six modules that form the main application:

- `src/main.rs` selects the terminal or web entry point.
- `src/cli.rs` draws the terminal interface, tracks progress, and runs exercise checks.
- `src/lessons.rs` supplies the titles, prompts, hints, answers, solution paths, and reference anchors.
- `src/api.rs` fetches public Chess.com archive data and converts a game record.
- `src/chess.rs` parses game metadata and moves, saves board positions, and implements answer checks.
- `src/web.rs` serves the browser app and this book.

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
