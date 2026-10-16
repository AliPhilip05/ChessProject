# ChessProject

GTE (ChessProject) is a Rust prototype with a browser chess app and a Rustlings-style terminal path for learning how its components work. Product requirements and implementation notes are in [the prototype spec](docs/APP_AND_TERMINAL_PROTOTYPE.md). The chapter-based Guidebook is in [docs/book](docs/book/README.md), with a full [codebase walkthrough](exercises/reference.md).

## Run the terminal learning UI

```bash
cargo install --path . --bin gte
gte
```

The installed `gte` command opens directly into the terminal UI. It starts with the GTE logo and Question 0, a completed codebase orientation exercise. The first check runs automatically so you can read a passing result before editing anything. Later exercises use a Rustlings-style progress bar and show the current source file. `N`/`P` move between exercises; `L` opens the exercise list; and `R` runs the focused Cargo test. Each exercise includes NOTE and TODO comments, a hint, a concept check, a `reference.md` link, and a reference solution. When a check passes, the TUI marks it complete and loads its matching solution automatically.

- `H` shows a hint.
- `R` runs the lesson's focused Cargo tests (`T` remains an alias); passing tests open the matching solution automatically.
- `C` checks all exercise targets from the home screen or opens a short concept check from inside an exercise.
- `X` asks before restoring the current exercise source from Git HEAD; it saves a backup in `.gte-backup/`.
- `S` opens the reference solution at any time.
- `G` prints the browser Guidebook address.
- `L` opens the exercise list; `Esc` returns to the previous screen; `Q` exits.

Learner code is under `exercises/src/`; tests are under `exercises/tests/`; reference solutions are under `solutions/`. The 24 questions cover eight codebase topics at Easy, Medium, and Hard levels. Question 0 is the prefilled Easy orientation, and its passing test output maps the app's entry points and six main subsystems. Easy lessons guide learners through small changes, Medium lessons ask for a specific behavior in partial code, and Hard lessons leave a complete function to implement. Every source has `NOTE:` and `TODO:` comments, each exercise guide ends in a link to `exercises/reference.md`, and passing checks load the matching solution automatically. The Rust basics Medium exercise uses `assert_eq!` to demonstrate the compiler error caused by a missing `PartialEq` derive.

## Run the browser app and Guidebook

```bash
cargo run -- --web
```

Open [http://127.0.0.1:7878](http://127.0.0.1:7878) for the app or [http://127.0.0.1:7878/guide](http://127.0.0.1:7878/guide) for the Guidebook. The Guidebook header links the [Function Appendix PDF](http://127.0.0.1:7878/guide/functions.pdf), a bookmarkable function-by-function reference, and [Download Markdown](http://127.0.0.1:7878/guide/download) for the complete book source. The app has two modes:

- **Guess the Elo:** load a Chess.com game link or sample a public player's recent archive, review the board move by move, and guess White, Black, or both at Easy (±200), Medium (±100), or Hard (±50) difficulty.
- **Guess the Opening:** inspect the first few moves and submit an opening name. Capitalization and repeated whitespace do not affect matching.

The browser UI and game logic are rendered by Rust. It uses Chess.com's read-only [Published-Data API](https://www.chess.com/news/view/published-data-api). Random selection samples one player's public archive because the API does not expose a global random-game feed. If an API request fails, the built-in demo game remains available.

## Project layout

```text
src/main.rs           Selects the terminal or browser entry point
src/api.rs + api/     Public archive facade, HTTP client, URLs, record conversion
src/chess.rs + chess/ Shared game facade, PGN, SAN, snapshots, answer rules
src/cli.rs + cli/     Terminal loop, state, input, rendering, progress, checks
src/lessons/          Lesson model, ordered catalog, one data file per topic
src/web.rs + web/     Browser facade, HTTP, routing, sessions, handlers, renderers, book
assets/               Embedded HTML templates and app/book stylesheets
docs/                 Product spec and programming-book source
exercises/src/        Learner-editable Rust exercises
exercises/tests/      Focused tests for each lesson
exercises/reference.md Codebase walkthrough linked at the end of every exercise
solutions/            Reference implementations
tools/                Recursive function-page and PDF appendix generators
output/pdf/           Bookmarkable function appendix
```

Each subsystem exposes a small public facade and keeps its helpers in focused child files. The [architecture chapter](docs/book/src/architecture.md) maps every responsibility. Regenerate the function pages and PDF after moving production functions so the book retains current source paths.

The built-in sample game supports both app modes without network access. The browser server listens on `127.0.0.1:7878`.

## Host the browser app

The `render.yaml` Blueprint runs the Rust server as a Render web service at its
own `onrender.com` address. Connect this repository to Render and create a
Blueprint from `render.yaml`. Render sets `PORT`, and the Blueprint sets
`BIND_ADDRESS=0.0.0.0` so the service accepts external traffic. Local runs keep
using `127.0.0.1:7878`.

The free service sleeps when idle, so the first request after a pause can take
longer. Game sessions are stored in memory and reset whenever the service
restarts. A custom domain can be added through Render's domain settings and
the domain's DNS provider.
