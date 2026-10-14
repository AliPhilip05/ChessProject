# ChessProject App and Terminal Learning Prototype

## Purpose

Build a Rust prototype with two connected experiences:

1. A browser-based chess practice app with **Guess the Elo** and **Guess the Opening** modes.
2. A Rust terminal learning UI that teaches the app's components through Rustlings-style exercises, hints, checks, solutions, and a from-scratch capstone.

The existing project is a Rust/Cargo project. Product logic, Chess.com API access, page rendering, exercise checks, and terminal interaction stay in Rust. The browser experience is rendered as HTML/CSS by the Rust program and uses ordinary links and forms for interaction; it does not require a JavaScript application.

## Product overview

### Inspiration

The Guess the Elo concept is inspired by [GothamChess's Guess the Elo series on YouTube](https://www.youtube.com/results?search_query=GothamChess+Guess+the+Elo). Viewers watch a game without seeing the players' ratings, assess the play, and estimate each player's skill.

### Formal description

ChessProject is a chess-game study and programming-learning prototype. Its browser app loads public Chess.com game records, hides the players' ratings while users review moves, and scores rating or opening guesses. Its terminal companion presents small, ordered Rust exercises that explain how the app is built. Learners can inspect hints and reference solutions, complete focused knowledge checks, and finish with a capstone that asks them to assemble a small version of the project from a blank starting point.

The design uses a Chess.com-inspired green, cream, and charcoal palette while keeping original product naming, layout, and assets. It does not use Chess.com logos or proprietary piece artwork.

The app and terminal UI use **GTE** as the product mark.

## App requirements

### Overview and navigation

- The home page includes a concise overview, the GothamChess series link, and the formal product description.
- Primary navigation opens **Guess the Elo**, **Guess the Opening**, and the **Guidebook**.
- The visual direction uses deep charcoal page chrome, warm cream light squares, muted green dark squares, and bright green primary actions. The board uses Unicode chess symbols or original CSS-rendered pieces.

### Shared game board

- Render the current position and the move list from a game's PGN.
- Provide first, previous, next, and last move controls, plus selectable move-list entries.
- Show player names, colors, result, time control, and game date when present.
- Keep each mode's answer controls available beside or below the board at useful breakpoints.

### Guess the Elo

- Start from either a pasted Chess.com game URL or a randomly selected game in a public player's archive.
- Accept a Chess.com username for archive lookup. The PubAPI is player/archive based and does not expose a single global random-game endpoint, so random selection is from the selected player's available public archive games.
- Resolve a pasted game URL against the selected player's fetched archive. Explain when the game is not in that archive and offer the random/archive path as a fallback.
- Hide both actual ratings until the user submits a guess.
- Offer difficulty tolerances:
  - Easy: within ±200 rating points.
  - Medium: within ±100 points.
  - Hard: within ±50 points. (The brief's “harm” is interpreted as “hard.”)
- Offer a target selector for **both players**, **White only**, or **Black only**.
- Accept numeric rating guesses. Return each guessed and actual rating, the absolute error, and whether it was within the selected tolerance.

### Guess the Opening

- Load a public archive game or a built-in demo game and show its opening position after a short sequence of moves.
- Accept a free-text opening name. Compare case-insensitively after trimming whitespace and normalizing repeated spaces.
- Reveal the expected opening and provide a retry or next-game action.
- Use PGN `Opening`/`ECO` metadata when present, with a small local opening-name map for demo games.

### Chess.com data integration

- Use the public, read-only Chess.com Published-Data API.
- Fetch available monthly archives from `/pub/player/{username}/games/archives`; fetch a monthly archive from its returned URL and choose among eligible games. Make requests serially and handle missing games, invalid usernames/URLs, and HTTP/API errors with readable page feedback.
- Parse PGN headers, SAN move tokens, player ratings, opening metadata, game URL, date, result, and time control.
- Include a built-in sample game so both modes can be explored if the API is unavailable. Clearly label fixture content as demo data.
- Never submit moves to Chess.com; the PubAPI is read-only.

## Terminal learning UI

### Interaction

- `cargo run` launches a keyboard-driven terminal learning interface.
- The terminal opens with the GTE logo on Question 0, runs that prefilled orientation exercise, and shows the passing test output. `r` runs the selected exercise's focused Rust tests; `n` advances after a pass; `p` moves to the previous exercise; `h` shows a hint; `s` opens its reference solution; `c` opens its concept check; `g` prints the matching Guidebook reference URL; `l` opens the exercise list; `x` confirms restoring the current exercise from Git; `q` exits.
- Include a lesson picker so learners can jump directly to a chosen app component.
- Display a concise status for each lesson: not started, in progress, or complete.
- Keep the TUI accessible in plain terminals with a readable text fallback if raw terminal mode cannot start.

### Curriculum and lesson assets

- Store lesson prompts under `exercises/` and worked reference examples under `solutions/`.
- Keep learner-editable Rust stubs in `exercises/src/` and focused Rust integration tests in `exercises/tests/`.
- Organize lessons around Rust fundamentals used in the project:
  0. A prefilled project map and passing test run that introduces the app entry points and main modules.
  1. Rust types, enums, and the mode menu.
  2. Chess.com archive records and error handling.
  3. PGN parsing and move-by-move board state.
  4. Elo scoring and tolerance boundaries.
  5. Case-insensitive opening answers.
  6. Server-rendered page/UI flow and the Guidebook.
  7. Capstone: implement a compact playable guessing flow from scratch.
- Each exercise source function includes a `NOTE:` comment describing its expected behavior and a `TODO:` comment marking where code belongs. Each lesson also has a hint, a concrete check, a focused test target, and a matching reference solution.
- `R` runs the selected target with Cargo (`T` is also accepted). A failed target displays compiler/test output; passing tests mark the lesson complete and automatically load its matching file from `solutions/`. `S` can open that solution at any time. `C` opens a short project-understanding check. The Rust basics test uses `assert_eq!` to exercise the compiler feedback for a missing `PartialEq` derive.

### Guidebook

- Provide a browser-readable, chapter-based Guidebook from the Rust web server at `/guide`, styled as a programming book with a chapter sidebar, readable code blocks, and previous/next navigation.
- Include a Rust entry point, app architecture, Chess.com archive/API flow, PGN and board-state flow, both game-mode rules, TUI keyboard controls, exercise/hint/solution workflow, and capstone instructions. The complete file-by-file walkthrough is `exercises/reference.md` and the end of every exercise links to its relevant section.
- Link to the Guidebook from the app navigation and print its local URL when requested from the TUI.

## Implementation plan

- Keep the existing Cargo application as the single Rust project.
- Split code into small modules for chess/PGN state, API/archive access, game modes and scoring, HTML rendering/web routing, and terminal lessons.
- Keep the learner exercises in a dependency-light Cargo package under `exercises/`; one integration-test target checks each lesson independently.
- `cargo run` opens the terminal learning UI; `cargo run -- --web` starts the local browser app and Guidebook server.
- Keep browser rendering server-side. Use Rust-generated HTML and CSS assets with standard GET forms/links for game loading, move navigation, answers, and guide navigation.
- Store the selected game's PGN and lesson progress in the running Rust process for the prototype. Use a built-in game fixture for deterministic fallback behavior.
- Add lesson assets and a `solutions/` directory without replacing the existing Rust/Cargo project structure.

## Prototype boundaries

- The PubAPI is public and read-only. API freshness can lag and parallel requests can be rate-limited; the prototype uses serial requests and a demo fallback.
- Random game selection samples games in a selected player's archive, not all Chess.com games.
- The prototype does not authenticate Chess.com accounts or send moves to Chess.com.
- The terminal runs the selected exercise test as a local Cargo process. It is an instructional project tool, not a sandbox for untrusted code.
- The capstone is an instructional implementation challenge with focused tests and a reference solution, not a hosted grading service.

## References

- [Chess.com Published-Data API documentation](https://www.chess.com/news/view/published-data-api)
- [Chess.com example archive endpoint](https://api.chess.com/pub/player/erik/games/archives)
- [GothamChess Guess the Elo YouTube search](https://www.youtube.com/results?search_query=GothamChess+Guess+the+Elo)
