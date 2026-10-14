# Project Architecture

The executable is intentionally small. `src/main.rs` selects one of two entry points: the terminal lesson UI or the local browser server.

## Main application files

- `src/main.rs` parses the `--web` switch and starts the chosen interface.
- `src/cli.rs` draws the terminal screens and handles keyboard input.
- `src/lessons.rs` defines lesson text, hints, concept questions, and exercise paths.
- `src/api.rs` requests public player archives and converts records to `ChessGame`.
- `src/chess.rs` parses PGN metadata and SAN moves, stores board snapshots, and implements scoring helpers.
- `src/web.rs` runs the local HTTP server, routes requests, stores sessions, and renders HTML/CSS.

The data flow is:

```text
main.rs
├── cli.rs ── lessons.rs ── exercises/ and solutions/
└── web.rs ── api.rs ── Chess.com Published-Data API
       └──── chess.rs ── PGN, positions, scoring, opening answers
```

## Two Cargo packages

The root `Cargo.toml` builds the app. `exercises/Cargo.toml` defines small practice functions and integration-test targets. Keeping those files separate lets a learner edit a stub without breaking the app's `cargo check`.

The app uses Crossterm for the terminal interface, `ureq` for HTTP requests, and `serde_json` for API responses. `Cargo.lock` records the resolved dependency versions. The exercise crate has no third-party dependencies.

Each exercise test includes one file from `exercises/src/` with a path attribute. The test checks that function's behavior; it does not make live API requests.

## Browser sessions

`web.rs` keeps each selected game and its settings in an in-memory map. A session id is carried in links and forms. Move links request a saved ply, and guess forms send the user's answer to a Rust route. The server renders a new HTML page after each action. There is no JavaScript client or database in this prototype.

## Continue reading

Read [Chess.com Public Data](/guide/public-data) for the API flow, or open the full [reference.md walkthrough](/guide/reference) for every module and exercise file.
