# Codebase Walkthrough — reference.md

This is the complete tour of the current GTE prototype. The chapters in the browser Guidebook introduce each idea in order; this reference brings the files and request paths together so an exercise can point directly to the code it teaches.

## Project at a glance

GTE is a Rust application with two ways to use it: a browser chess practice app and an interactive terminal curriculum about the app. The browser supports Guess the Elo and Guess the Opening. The terminal curriculum uses Rust stubs, focused tests, hints, checks, and reference implementations.

The project root is one Cargo binary. A second Cargo package under `exercises/` contains learner functions and tests. The app's web server, UI state, API data, and chess model are all written in Rust. The browser pages are generated as HTML and CSS by `src/web.rs`.

The root `Cargo.toml` declares Crossterm for the terminal, `ureq` for HTTP, and `serde_json` for JSON parsing. `Cargo.lock` records the selected dependency versions. The exercise crate has no external dependencies, so its checks stay small and do not need API access.

The root `README.md` is the quick-start page. `docs/APP_AND_TERMINAL_PROTOTYPE.md` records the product requirements and implementation plan. `docs/book/README.md` and `docs/book/SUMMARY.md` describe the Guidebook chapters, whose Markdown sources live in `docs/book/src/`. `target/` is generated build output and is not source code.

## Rust types and mode selection

The executable starts in `src/main.rs`. It collects process arguments, checks for `--web`, and calls either `web::serve()` or `cli::run()`. Rust module declarations make `api`, `chess`, `cli`, `lessons`, and `web` available to the binary.

The browser's internal `GameMode` enum in `web.rs` distinguishes Elo rounds from opening rounds. The practice file `exercises/src/rust_basics.rs` makes the same kind of choice explicit with `GameMode::GuessElo` and `GameMode::GuessOpening`. An enum is a better fit than an unrestricted string when the valid choices are known. The exercise derives `PartialEq` and `Eq`; `assert_eq!` uses `PartialEq` to compare variants and produces a compiler error when that trait is missing.

## Chess.com archive data

`src/api.rs` owns HTTP and JSON details. `random_public_game` validates the username, loads the player's archive index, fetches recent monthly archives serially, filters records to standard chess, and selects a candidate. `game_from_archive_link` checks the recent archive entries for the supplied public game URL. Both functions return a `ChessGame` or a readable `String` error.

`validate_username` allows letters, digits, and underscores, with a maximum length of 25. `archive_urls` calls the public archive-index endpoint and extracts the returned month URLs. `get_json` uses `ureq` and `serde_json`. `to_chess_game` reads a game record's PGN, ratings, player names, opening metadata, URL, and time control. Optional values remain optional.

`same_game_id` and `game_id` compare the final game-id path segment for URL variants. `opening_from_eco` turns an opening URL slug into a readable fallback name. Random mode checks a small set of newest archive months and stops when it has candidates; link lookup scans a wider set of recent months to find a specific record.

The Published-Data API is read-only and organized by player archive. The app cannot ask it for one random game from every Chess.com player. Instead, it selects from the chosen player's archive. A link lookup also needs a username whose archive contains that game. Candidate selection uses a clock-derived index for a lightweight prototype. If API access fails, the `/load` route creates a session with `ChessGame::demo()` and shows the reason in a notice.

The smaller exercise `archive_api.rs` practices selecting newest-first URLs and checking usernames. Its test target uses local strings; it does not make an HTTP call.

## PGN parsing and board positions

`src/chess.rs` defines `ChessGame`, which stores:

- player names and optional ratings;
- opening name and ECO code;
- result, time control, date, and source URL;
- the accepted SAN move list;
- an array snapshot for the initial board and every accepted ply;
- whether the game is the built-in demo.

`ChessGame::with_metadata` combines API fields with PGN headers. `pgn_headers` reads bracketed metadata such as `White`, `WhiteElo`, `Opening`, and `Result`. `pgn_move_tokens` removes header lines, brace comments, semicolon comments, and parenthesized variations. `strip_move_number` handles prefixes such as `1.` and `1...`.

`starting_position` builds the initial 64-square array. `apply_san` updates a board for common SAN moves. It locates a plausible origin square, applies captures and promotions, and handles castling. `disambiguation_matches`, `piece_can_move`, and `path_is_clear` implement origin filtering, movement shapes, and obstruction checks. Header and rating helpers read metadata, and result-token checks prevent a result from becoming a move. This is not a complete chess legality engine; check, pins, all special cases, and arbitrary FEN start positions are outside this prototype.

`position_at(ply)` returns a saved position, clamped to the stored range. The web renderer uses it to show a selected move without replaying the game. The exercise `pgn_board.rs` focuses on filtering PGN text into SAN tokens and counting position snapshots.

The board array is row-major: index zero is the upper-left square, files move across each row, and ranks move downward. White pieces use uppercase letters; Black pieces use lowercase letters; `.` means an empty square. `chess.rs` also maps piece letters to Unicode symbols in `web.rs` and has small helpers to recognize results, parse positive ratings, and find a header value. Its public lesson helpers normalize opening strings and calculate rating tolerances.

## Elo scoring

The shared game helpers in `chess.rs` include `rating_tolerance` and `rating_is_close`.

- Easy accepts an absolute rating error of at most 200.
- Medium accepts at most 100.
- Hard accepts at most 50.
- Opening matching trims, lowercases, and collapses whitespace before comparing.

The browser validates submitted ratings as numbers from 0 through 4000 before calculating the error. It scores only the chosen side or sides, then reveals the stored ratings. If a game has no rating for a player, the page reports that instead of fabricating a value.

`exercises/src/elo_scoring.rs` isolates the tolerance lookup and comparison into small functions. Its tests document the inclusive boundary behavior.

## Opening answers

`normalize_answer` trims, lowercases, and collapses whitespace. `opening_matches` compares the normalized guess with the expected opening. The browser uses the same helper and reveals the expected opening after the answer is submitted.

`exercises/src/opening_answer.rs` practices this comparison. The capstone combines rating scoring and opening normalization in one `GameRecord` flow.

## Web server and rendered pages

`src/web.rs` uses `TcpListener` to serve on `127.0.0.1:7878`. `handle_connection` reads a basic HTTP GET request and sends a page, redirect, or not-found response. `write_response` sets the response content type, byte length, connection behavior, and `nosniff` header. `route` dispatches the paths:

- `/` renders the overview, mode forms, GothamChess series link, and demo entry points.
- `/guide` and `/guide/...` render the book chapters.
- `/load` loads a demo, random archive game, or pasted archive game.
- `/play?sid=...` renders the saved game and current mode.
- `/move?sid=...&ply=...` changes the displayed board snapshot.
- `/guess-elo` scores the requested rating guesses and reveals ratings.
- `/guess-opening` compares the opening answer and reveals feedback.

Each browser game is a `Session` stored in a process-local `Mutex<HashMap<...>>`. A session contains the game, mode, difficulty, side selection, current ply, notice, and feedback. `NEXT_SESSION` generates ids. Restarting the program clears this state; the prototype has no database or account system.

`load_game` chooses demo, random-archive, or pasted-link loading; `create_session` stores the resulting game and initializes opening mode at up to eight plies. `update_move` clamps a requested ply. `submit_elo_guess` and `score_one` validate and score numeric estimates, while `submit_opening_guess` calls the opening matcher. Difficulty and target normalizers constrain query values to supported choices.

`render_home` makes the overview and entry forms. `render_game` combines player rows, the board, move controls, move list, game metadata, and the appropriate answer panel. `render_board` and `mini_board` turn board characters into Unicode chess pieces inside colored squares. `render_move_list` creates links for each visible ply. The Elo panel hides ratings until reveal and includes only the selected guess fields. The opening panel asks for a text answer after up to eight plies.

The server accepts GET forms for a small dependency-free prototype. `parse_query` and `url_decode` decode their parameters, with `hex_value` converting percent-encoded bytes. `html_escape` protects names, notices, and other text before it enters HTML. `page` assembles the app's document shell. The app styles use a Chess.com-inspired green, cream, and charcoal palette; the documentation reader uses a separate light, book-like layout.

`BOOK_CHAPTERS` embeds the book's Markdown chapters at compile time. `render_book` builds the chapter sidebar and previous/next links. The small Markdown renderer handles headings, paragraphs, lists, fenced code, inline code, emphasis, and safe links. `BOOK_STYLE` supplies the book reading layout; the app's `STYLE` supplies its dark chess palette.

The query helpers support standard form encoding: `+` becomes a space and `%HH` becomes a byte. `normalize_difficulty` accepts Easy, Medium, and Hard (also treating the brief's `harm` spelling as Hard); `normalize_target` accepts White, Black, and Both. `max_ply` limits opening mode to the first eight plies. Metadata values and move text are escaped before being inserted into HTML. Session ids are generated numeric values, so they can safely travel in the move and answer URLs.

The exercise `server_rendering.rs` practices building a move URL and escaping HTML. It does not run the server.

## Terminal learning UI

`src/cli.rs` uses Crossterm for raw-mode keyboard input, an alternate screen, colors, and cursor control. `UiState` stores the current screen, selected lesson, hint visibility, started and completed sets, answer selection, solution text, and Cargo output.

The lesson list is defined in `src/lessons.rs`. Each `Lesson` contains an id, track name, title, concept, prompt, hint, short multiple-choice question, expected choice, and solution path. The status display distinguishes not started, in progress, and complete.

Key handling switches between home, lesson, concept-check, solution, and test-result screens. The lesson picker status uses an in-memory set: entering a lesson marks it in progress, and successful tests mark it complete. Pressing `T` runs `cargo test --manifest-path exercises/Cargo.toml --test <lesson-id>` as a child process. The screen shows stdout and stderr when the target fails. When it passes, the lesson is marked complete and its matching `solutions/` file loads automatically in the solution screen. Pressing `S` can also open that file at any time. If Crossterm cannot enable raw mode, the program prints a plain lesson list instead.

The Guidebook's terminal lesson chapter uses `H`, `C`, `T`, `S`, `G`, and `Esc` in the same way as the code.

## Practice files, tests, and solutions

The `exercises/` directory has its own `Cargo.toml` and contains:

- `src/*.rs`: learner-editable stubs. Every requested implementation has a `NOTE:` comment describing the expected behavior and a `TODO:` comment pointing to the insertion point.
- `tests/*.rs`: one focused integration-test target per lesson. Each test imports one exercise source file directly.
- `*.md`: the task, check, hint, NOTE, TODO, and a link to a section in this `reference.md`.
- `reference.md`: the full walkthrough you're reading now.

The focused exercise source and matching test targets are:

- `rust_basics.rs`: compare `GameMode` variants with `PartialEq`, then return their labels. Its test intentionally fails to compile until the equality trait is derived.
- `archive_api.rs`: select newest-first archive URLs and reject empty or unsafe usernames.
- `pgn_board.rs`: strip PGN scaffolding from SAN moves and count board snapshots.
- `elo_scoring.rs`: map difficulties to thresholds and test inclusive rating boundaries.
- `opening_answer.rs`: compare opening names after whitespace and case normalization.
- `server_rendering.rs`: compose move URLs and HTML-escape user text.
- `capstone.rs`: combine ply bounds, selected-side scoring, and opening matching.

The corresponding tests are in `exercises/tests/` with the same filenames. The app's terminal test runner turns hyphens in lesson ids into underscores to locate those Cargo test targets. Each source stub and each problem markdown file has NOTE/TODO guidance; each problem markdown ends with a `reference.md` section link.

The root `solutions/*.rs` directory contains complete exercise implementations. A typical cycle is: open the lesson, read the prompt and hint, edit its `exercises/src/` file, press `T` or run the focused Cargo command, then use `S` to compare against a solution if needed.

Focused tests are instructional, not a security sandbox. The terminal runs Cargo against local learner code with the current user's permissions.

## Capstone and end-to-end flow

The capstone begins with a `GameRecord` and asks you to connect move bounds, side selection, rating tolerances, and opening normalization. The sample data lets you complete that flow without API access. The root `solutions/capstone.rs` demonstrates the tested functions.

In the full app, the end-to-end flow is:

1. The landing page collects a mode and source.
2. `api.rs` loads a public archive game, or `ChessGame::demo()` supplies a fixture.
3. `chess.rs` builds metadata, moves, and board snapshots.
4. `web.rs` stores a session and renders its current position.
5. Move links change the ply; a guess form updates feedback and reveals the answer.

The app and the capstone share concepts, while the capstone remains a small learning exercise rather than a second full copy of the production app.

## Constraints and next steps

This prototype has no sign-in, persistent storage, global game feed, full chess engine, or real-time game connection. It relies on public archive freshness and network availability. The web server serves one connection at a time. Useful next steps include a proper chess rules library, better game-link lookup, persistent lesson completion, and more practice cases for PGN variations and unusual metadata.

The book reader itself is Rust-generated. It includes Markdown chapter sources with `include_str!`, converts the supported Markdown features to escaped HTML in `web.rs`, and wraps them in a responsive reading layout. This keeps the documentation navigable in the browser without adding a JavaScript framework or another runtime dependency.
