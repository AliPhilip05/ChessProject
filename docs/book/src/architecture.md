# Project Architecture

GTE is intentionally one binary with two front doors. The default command enters the terminal curriculum; `--web` starts the browser app and the book. The two front doors share the same Rust project but have separate responsibilities.

## Follow the process branches

```text
src/main.rs
├── default launch
│   └── src/cli.rs ── src/lessons.rs ── exercises/ and solutions/
└── --web
    └── src/web.rs ── src/api.rs ── Chess.com Published-Data API
                 └── src/chess.rs ── metadata, moves, positions, answers
```

`src/main.rs` reads command-line arguments. With `--web`, it calls `web::serve()`; otherwise it calls `cli::run()`. The `mod` declarations make the five source modules available to the binary.

## What each module owns

- `src/cli.rs` owns terminal mode, keyboard events, the current exercise screen, progress, file watching, Cargo child processes, and reset confirmation.
- `src/lessons.rs` is the curriculum data: ids, track names, task text, hints, concept questions, solution filenames, and reference anchors.
- `src/api.rs` owns Chess.com HTTP requests, JSON decoding, username and game-link validation, and conversion of a game record.
- `src/chess.rs` owns the game model, PGN parsing, supported SAN application, board snapshots, rating tolerance, and opening-answer normalization.
- `src/web.rs` owns HTTP request routing, in-memory sessions, answer handlers, HTML generation, CSS, and the Guidebook Markdown renderer.
- `exercises/src/` contains small learner-editable implementations; `exercises/tests/` contains the checks; `solutions/` contains completed examples.

## Two Cargo packages and why they are separate

The root `Cargo.toml` builds the app binary named `gte`. Its runtime dependencies are Crossterm for terminal behavior, `ureq` for HTTP, and `serde_json` for JSON.

`exercises/Cargo.toml` is a separate package. Each test in `exercises/tests/` imports exactly one source file with a `#[path = "../src/...rs"]` attribute. That makes a learner's incomplete `todo!()` function fail its focused test without making the root web app fail to compile. The exercise tests use local inputs; they do not make network requests.

Question 0 has a complete component map and two passing checks. Start there to see the names of the two entry points before tracing a single game through the app. The more detailed path is in [Rust Entry Points and Types](/guide/rust-entry-point).

## Data flow through the browser app

1. The home form sends a mode, difficulty, player target, username, and optional game link to `/load`.
2. `web::load_game` chooses the demo fixture or calls an API loader.
3. `api.rs` converts a public record and its PGN into a `ChessGame`.
4. `chess.rs` creates the move list and a board snapshot for each accepted ply.
5. `web.rs` stores the game and settings in a `Session` and redirects to `/play?sid=...`.
6. Move links select a snapshot. Guess forms update feedback and reveal the answer.

## Browser sessions and lifetimes

`web.rs` stores active rounds in a process-local `Mutex<HashMap<String, Session>>`. The map is initialized once, and an atomic counter creates session ids. A route that only renders borrows a session; a route that changes the selected ply or feedback takes a mutable borrow under the mutex.

There is no account model, durable database, or persistence across server restarts. This keeps the prototype's state path easy to read and gives the capstone a bounded data model.

Continue with [Chess.com Public Data](/guide/public-data) to inspect the API boundary, or go directly to [the complete walkthrough](/guide/reference#project-at-a-glance).
