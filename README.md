# ChessProject

GTE (ChessProject) is a Rust prototype with a browser chess app and a Rustlings-style terminal path for learning how its components work. Product requirements and implementation notes are in [the prototype spec](docs/APP_AND_TERMINAL_PROTOTYPE.md). The chapter-based Guidebook is in [docs/book](docs/book/README.md), with a full [codebase walkthrough](exercises/reference.md).

## Run the terminal learning UI

```bash
cargo run
```

Use ↑/↓ to select a lesson and Enter to open it. Each lesson includes NOTE and TODO comments in its editable Rust exercise, a hint, a focused Rust test target, and a reference solution. When a test passes, the TUI marks the lesson complete and loads its matching solution automatically.

- `H` shows a hint.
- `T` runs the lesson's focused Cargo tests; passing tests open the matching solution automatically.
- `C` opens a short concept check.
- `S` opens the reference solution at any time.
- `G` prints the browser Guidebook address.
- `Esc` returns to the lesson list; `Q` exits.

Learner code is under `exercises/src/`; tests are under `exercises/tests/`; reference solutions are under `solutions/`. Each exercise markdown file ends with a link to the relevant section in `exercises/reference.md`. The Rust basics exercise uses `assert_eq!` to demonstrate the compiler error caused by a missing `PartialEq` derive.

## Run the browser app and Guidebook

```bash
cargo run -- --web
```

Open [http://127.0.0.1:7878](http://127.0.0.1:7878) for the app or [http://127.0.0.1:7878/guide](http://127.0.0.1:7878/guide) for the Guidebook. The app has two modes:

- **Guess the Elo:** load a Chess.com game link or sample a public player's recent archive, review the board move by move, and guess White, Black, or both at Easy (±200), Medium (±100), or Hard (±50) difficulty.
- **Guess the Opening:** inspect the first few moves and submit an opening name. Capitalization and repeated whitespace do not affect matching.

The browser UI and game logic are rendered by Rust. It uses Chess.com's read-only [Published-Data API](https://www.chess.com/news/view/published-data-api). Random selection samples one player's public archive because the API does not expose a global random-game feed. If an API request fails, the built-in demo game remains available.

## Project layout

```text
src/                 Rust app, Chess.com API, board parser, TUI, web renderer
docs/                Product spec and book-style Guidebook source
exercises/src/        Learner-editable Rust exercises
exercises/tests/      Focused tests for each lesson
exercises/reference.md Codebase walkthrough linked at the end of every exercise
solutions/            Reference implementations
```

The built-in sample game supports both app modes without network access. The browser server listens on `127.0.0.1:7878`.
