# Project Architecture

GTE is one binary with two entry points. The default command enters the terminal curriculum; `--web` starts the browser app and this book. `src/main.rs` chooses between `cli::run()` and `web::serve()`.

Each subsystem has a small public facade. Its private child modules own a specific job, so request parsing, chess rules, terminal rendering, and lesson data can be read independently. A `mod` declaration adds a child module; `pub use` exposes the few names that callers need without making every helper public.

## Follow the process branches

```text
src/main.rs
├── default launch
│   └── src/cli.rs ── src/cli/ ── src/lessons/
│                                  └── exercises/ and solutions/
└── --web
    └── src/web.rs ── src/web/handlers/ and render/
                       ├── src/api.rs ── src/api/ ── Chess.com
                       ├── src/chess.rs ── src/chess/
                       └── src/web/book/ ── docs/book/ and reference.md
```

## Network and chess modules

- `src/api.rs` exposes the two archive-loading operations. `api/archives.rs` orchestrates archive selection and game-link lookup; `client.rs` owns HTTP and JSON requests; `mapping.rs` converts API records; `urls.rs` validates usernames and compares game URLs.
- `src/chess.rs` exposes the shared game type and answer rules. `chess/game.rs` owns `ChessGame` and snapshots; `pgn.rs` extracts headers and SAN tokens; `board.rs` creates the starting position; `san.rs` applies supported moves; `answers.rs` normalizes opening guesses and scores rating errors.

The dependency direction is API conversion toward the chess model. The chess modules do not fetch network data or build browser HTML.

## Browser modules and assets

- `src/web.rs` is the browser facade. `http.rs` owns TCP and response bytes, `router.rs` chooses a handler, and `request.rs` decodes and normalizes request fields.
- `src/web/session.rs` owns `GameMode`, `Session`, the process-wide store, and the visible-ply rule.
- `src/web/handlers/setup.rs` loads games and creates sessions. `navigation.rs` renders a saved session or updates its ply. `guesses.rs` validates answers and updates feedback.
- `src/web/render/` contains separate home, game, answer-panel, board, and shared HTML helpers. Renderers borrow data and produce strings; handlers decide how a request changes state.
- `src/web/book/` separates chapter data, reader layout, block Markdown, and inline Markdown. It reuses HTML escaping from the shared renderer.
- `assets/templates/` stores fixed HTML templates. `assets/styles/` stores the app and book stylesheets. Rust embeds these assets when compiling the binary.

## Terminal modules and curriculum

- `src/cli.rs` owns terminal setup, the polling loop, source watching, and cleanup.
- `src/cli/state.rs` defines screens and current lesson state; `writer.rs` adapts newlines for raw terminals; `progress.rs` reads and writes completion ids.
- `src/cli/source.rs` resolves exercise files, `reset.rs` restores a backed-up starter, `runner.rs` executes Cargo checks, and `navigation.rs` selects lessons.
- `src/cli/input/` handles keyboard events by screen. `src/cli/render/` draws each screen in a separate file.
- `src/lessons/mod.rs` exposes `Lesson` and `LESSONS`. `model.rs` defines a lesson record, `catalog.rs` assembles the ordered course, and eight topic files each own the Easy, Medium, and Hard records for one concept.

The terminal consumes the catalog through `lessons::{Lesson, LESSONS}`. It does not need to know which topic file defines a particular record.

## Two Cargo packages

The root `Cargo.toml` builds the app binary named `gte`. Its runtime dependencies are Crossterm for the terminal, `ureq` for HTTP, and `serde_json` for JSON.

`exercises/Cargo.toml` is a separate package. Each test in `exercises/tests/` imports one learner file with `#[path = "../src/...rs"]`. An unfinished practice function therefore fails its focused test without making the browser app fail to compile. Exercise tests use local data and do not make network requests. Reference implementations live under `solutions/`.

Question 0 starts with complete code and two passing checks. Its compact component map introduces subsystem names; this chapter shows their current physical module layout.

## Data flow through the browser app

1. The home form sends settings and a game source to `/load`.
2. `web::router` dispatches to `web::handlers::setup::load_game`.
3. `api::archives` obtains a public record; `api::mapping` builds the shared `ChessGame`.
4. `chess::game` combines metadata, PGN tokens, and a saved board after each accepted ply.
5. The setup handler stores a `Session` and redirects to `/play?sid=...`.
6. Navigation selects a snapshot. Guess handlers update feedback. Render modules turn the saved state into HTML.

## Browser sessions and lifetimes

`web::session` stores rounds in a process-local `Mutex<HashMap<String, Session>>`. The map is initialized once, and an atomic counter creates ids. Renderers borrow a session; handlers that change its ply or feedback take a mutable borrow under the mutex.

Sessions are not persisted across server restarts. The process owns each loaded game and its settings until shutdown.

Continue with [Chess.com Public Data](/guide/public-data), or use [the complete walkthrough](/guide/reference#project-files-and-cargo-packages) and the generated function pages to inspect an individual function at its current source path.
