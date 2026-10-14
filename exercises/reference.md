# Codebase Walkthrough — reference.md

This is the complete companion to the GTE Book. It maps the current Rust source, explains the request and state flows, names the exercise tests and solutions, and records the boundaries of the prototype. Use it as a file-by-file reference while doing the terminal problems.

## Question zero: project map

The first terminal exercise is `intro-0`, shown as Question 0. Its source at `exercises/src/intro_0.rs` is fully implemented, and its integration test is `exercises/tests/intro_0.rs`. Run it before changing anything:

```bash
cargo test --manifest-path exercises/Cargo.toml --test intro_0
```

The passing test names establish three facts: GTE's product mark is `GTE`, `main` can select `cli::run` or `web::serve`, and the component map names the six main application modules. The exercises are intentionally prefilled for this introduction so the learner sees a successful test before encountering a compiler error.

Question 0's `Component` struct has `path` and `purpose` fields. Its `COMPONENTS` array provides a small map. `product_mark` returns the mark, `entrypoint` models the branch from `main.rs`, and `component` searches the array. The tests check both present and missing paths. `Component` derives `PartialEq`/`Eq` so `assert_eq!` can compare records; this previews the equality trait in `rust_basics`.

## Project files and Cargo packages

The root contains the actual GTE application. The important tracked source paths are:

- `Cargo.toml` and `Cargo.lock` configure and lock the `gte` binary and its runtime dependencies.
- `src/main.rs`, `src/cli.rs`, `src/lessons.rs`, `src/api.rs`, `src/chess.rs`, and `src/web.rs` implement the program.
- `exercises/Cargo.toml` starts a separate exercise package. Its `src/` files are learner work, its `tests/` files are independent checks, and the root `solutions/` folder contains reference implementations.
- `exercises/reference.md` is this walkthrough. `docs/book/README.md`, `SUMMARY.md`, and `docs/book/src/*.md` contain the Guidebook chapters. `src/web.rs` embeds those Markdown files at compile time.
- `README.md` is the command-oriented project introduction. `docs/APP_AND_TERMINAL_PROTOTYPE.md` records the product scope and intended behavior. `.gitignore` excludes build output, local exercise progress, and reset backups.
- `target/` and `exercises/target/` are Cargo output, not source inputs.

There is also a tracked `src/main` Mach-O executable from the original workspace. The current root manifest sets `autobins = false` and explicitly declares `src/main.rs` as the `gte` binary, so Cargo does not use that extensionless executable as a program target.

The root package uses Crossterm for the TUI, `ureq` for HTTP, and `serde_json` for JSON. The exercise package has no external dependencies. Each integration test imports only one learner file with a `#[path]` attribute, which lets unfinished exercises fail without breaking the app crate.

## `src/main.rs`: choose the front door

The module declarations compile the five sibling modules into the binary. `main` collects the process arguments into `Vec<String>`, uses `any` to search for `--web`, then calls one of two functions:

- no `--web`: call `cli::run()` and open the terminal curriculum;
- with `--web`: call `web::serve()` and bind the local website.

The function runs synchronously. The chosen front door owns the process until the user quits the terminal or stops the web server. The small `entrypoint` helper in Question 0 models this decision for a test; it is not the production argument parser.

## Rust types and mode selection

`web.rs` uses a `GameMode` enum with `Elo` and `Opening` variants. This is a closed set of choices: invalid strings cannot be stored as a mode. The enum derives `Clone`, `Copy`, `PartialEq`, and `Eq`; comparisons in route handlers use `==` and `!=`.

The practice file `exercises/src/rust_basics.rs` repeats the idea with `GuessElo` and `GuessOpening`. Its tests use `assert_eq!` and `assert_ne!`, which require `PartialEq`. `Eq` is the marker trait for an equality relation that is reflexive for all values. `Component` in Question 0 also derives these traits so a test can compare records.

Across the app, `String` owns request or parsed text, while `&str` borrows text for a function call. `Option<i32>` represents a rating that may be missing; it is safer than a magic zero. `Result<ChessGame, String>` represents a successful archive lookup or a user-readable failure. These types make normal and exceptional data explicit at call sites.

## `src/lessons.rs`: curriculum data

`Lesson` stores the data displayed for one exercise: a stable `id`, track and title strings, concept explanation, task prompt, hint, multiple-choice question, three choices, correct-choice index, solution filename, and reference anchor.

`LESSONS` is the ordered fixed-size array of 24 entries. Eight codebase topics have Easy, Medium, and Hard variants. Project orientation starts with the already-complete Question 0, followed by a mode-to-module exercise and a full route lookup. The other seven topics each have three code exercises. The order affects the progress bar and next/previous navigation.

The id links several systems. `intro-0` becomes `intro_0` for the Rust test target and source path. `run_lesson_test` uses that transformed id after `cargo test --test`. The matching solution path is stored explicitly. The reference anchor in the lesson data is used by the `G` action to open the corresponding section in the book.

The lesson array is static because the curriculum is compiled into the binary. Adding a lesson requires adding its source, test target, Markdown prompt, solution, and `Lesson` record, plus updating the docs.

## Chess.com archive data

The production archive boundary is implemented in `src/api.rs`.

`USER_AGENT` identifies the prototype to the public API. The public functions return `Result<ChessGame, String>` so network failures, empty data, and invalid input become user-visible outcomes rather than panics.

### `random_public_game`

1. Call `validate_username` before building a URL.
2. Call `archive_urls` to request the player's archive index.
3. Iterate over the last four archive URLs in reverse order, so the newest months are checked first.
4. Call `get_json` for one monthly archive at a time and turn its `games` array entries into candidates with `to_chess_game`.
5. Stop after finding candidates in a month. Return an error if no eligible game exists.
6. Use a `SystemTime` nanosecond count modulo the candidate count to choose one result.

This is random selection within a selected player's recent public archive, not a global Chess.com game feed. The clock-based index is simple sampling for a prototype, not a secure random source.

### `game_from_archive_link`

This function trims the pasted URL and accepts only the Chess.com live/game path prefixes. It fetches the same player's archive index, scans up to twelve recent monthly archives, and compares each record URL with the requested URL using `same_game_id`. A matching record still has to pass `to_chess_game`; a matching URL without usable PGN produces a separate error. A game absent from those months returns guidance to try another username or a random game.

### Validation and JSON helpers

- `validate_username` trims, lowercases ASCII, limits length to 25, and permits only ASCII letters, digits, and underscores.
- `archive_urls` requests `/pub/player/{username}/games/archives`, extracts string members from `archives`, and distinguishes absent and empty archive lists.
- `get_json` sends a GET with the user-agent header, maps HTTP errors to readable text, and decodes a `serde_json::Value`.
- `to_chess_game` ignores a record with a present non-`chess` `rules` value and requires a PGN string. It reads ratings as `i64` and checks that they fit in `i32`. It extracts usernames, opening, ECO, URL, and time control. It passes those values to `ChessGame::with_metadata`, then replaces the default names/time control when archive fields are present.
- `same_game_id` compares normalized final ids; `game_id` takes the last nonempty slash-separated path component and removes a query string.
- `opening_from_eco` takes the final URL path segment and replaces hyphens with spaces as a readable fallback.

The API client is read-only. It needs no account secret and does not send a move. It does not cache responses, retry failed requests, parallelize archive fetches, or search every month.

## PGN parsing and board positions

The production game model and parser are implemented in `src/chess.rs`.

### `ChessGame` data

`ChessGame` is the shared parsed model. Its public fields contain names, optional player ratings, opening, ECO, result, time control, date, source URL, demo marker, and SAN moves. Its `positions` field is private so callers use `position_at` rather than mutating snapshots independently of the move list.

`from_pgn` delegates to `with_metadata` with no overrides. `with_metadata` parses headers, starts from the initial board, walks move tokens, strips move numbers, skips results/NAGs/en-passant marker tokens, and calls `apply_san`. Only accepted moves are appended to `moves` and only accepted positions are appended to `positions`. It alternates the side to move only after an accepted move.

Metadata precedence is explicit: a nonempty supplied override wins; otherwise the relevant PGN header is used; otherwise a default string is supplied. Ratings prefer supplied options then parse `WhiteElo`/`BlackElo`. The result defaults to `*`, time control to `Unknown`, and date to `Date unavailable`.

`demo` constructs the included Italian Game fixture with names, ratings, date, result, time control, ECO, opening, URL, and ten full moves. It marks `demo = true` for the renderer.

`position_at` clamps an oversized ply to the final stored snapshot. `ply_count` is `positions.len().saturating_sub(1)` so the count is the number of transitions from the starting position.

### Answer helpers

- `normalize_answer` splits on whitespace, rejoins with one space, and lowercases. It handles leading, trailing, and repeated spaces.
- `opening_matches` rejects an empty normalized guess and compares normalized values.
- `rating_tolerance` maps Easy to 200, Hard to 50, and all other input to Medium's 100-point default. The web layer normalizes user input before using it.
- `rating_is_close` compares absolute rating error with the selected tolerance. The boundary is inclusive.

## Elo scoring

The web route parses each selected numeric guess with `score_one`, limits it to the range 0 through 4000, handles unknown actual ratings, calculates absolute difference, and delegates the threshold comparison to `rating_is_close`. The three thresholds are 200, 100, and 50 points. `exercises/src/elo_scoring.rs` isolates that mapping and comparison so the tests can assert both exact limits and values just outside the limit.

## Opening answers

`normalize_answer` is shared between Guess the Opening and the capstone. It collapses whitespace and lowercases. `opening_matches` rejects an empty normalized guess and compares it to the expected name. The web handler separately distinguishes blank input so it can ask the user to enter a name instead of saying the answer was wrong. The `opening_answer` exercise checks case, repeated spaces, wrong answers, and empty input.

### Header and token parsing

- `pgn_headers` accepts bracketed lines, finds the first whitespace after the key, removes the closing bracket and surrounding quotes, unescapes `\\"`, and inserts the resulting pair into a `HashMap`.
- `header` returns a borrowed string from that map.
- `parse_rating` accepts only positive integer ratings.
- `pgn_move_tokens` scans non-header lines character by character. It discards brace comments, semicolon-to-end-of-line comments, and nested parenthesized variations. It returns the remaining whitespace-separated tokens.
- `strip_move_number` removes the leading run of digits and periods, covering SAN prefixes such as `1.` and `1...`.
- `is_result` recognizes `1-0`, `0-1`, `1/2-1/2`, and `*`.

### Board representation and SAN application

`starting_position` creates a row-major `[char; 64]`. Index zero is a8. White pieces are uppercase, Black pieces lowercase, and `.` represents an empty square. Rows are stored from Black's back rank down to White's back rank.

`apply_san` trims trailing `+`, `#`, `!`, and `?`. It handles castling by moving king and rook. For other SAN it detects promotion, capture, destination square, piece kind, and optional disambiguation. It scans possible origins and rejects candidates of the wrong color, piece kind, disambiguation, or movement shape. On success it moves the piece, removes an en passant pawn where indicated by a diagonal pawn move into an empty square, promotes a pawn on the last rank, and returns true.

- `disambiguation_matches` applies a source file or rank restriction from SAN.
- `piece_can_move` checks knight and king shapes, sliding pieces, and pawn forward/capture rules. Rooks, bishops, and queens delegate obstruction checks to `path_is_clear`.
- `path_is_clear` walks one square at a time from the origin toward the destination, excluding the destination, and rejects any occupied intermediate square.

This is intentionally a teaching renderer, not a full rules engine. It does not verify check, pins, castling rights, all SAN legality, or arbitrary FEN starts. Some special-move and ambiguity cases are simplified. The stored board should therefore be treated as a useful review visualization for common PGN, not authoritative game adjudication.

## Web server and rendered pages

All route handling and HTML generation are in `src/web.rs`.

### Shared state

`GameMode` distinguishes Elo from Opening and derives equality traits for route checks. `Session` owns the game, mode, difficulty, target, current ply, notice, feedback, and reveal state.

`SESSIONS` is a `OnceLock<Mutex<HashMap<String, Session>>>`; `sessions()` initializes it once. `NEXT_SESSION` is an atomic counter that supplies numeric ids. All session data is lost when the server process exits.

### Server and route dispatch

- `serve` binds only `127.0.0.1:7878`, reports the app and Guidebook URLs, and handles accepted connections serially.
- `handle_connection` applies a short read timeout, reads one request buffer, parses the first request line, rejects non-GET methods with 405, calls `route`, and writes a page, download, redirect, or 404.
- `HttpResponse` distinguishes HTML pages, Markdown downloads, redirects, and not-found responses.
- `route` separates a request target into path/query, calls `parse_query`, and dispatches known routes. Unknown `/guide/{slug}` paths are checked against the Guidebook chapter list.

### Game setup and session handlers

- `load_game` normalizes `mode`, difficulty, target, and source. For `random` it calls `api::random_public_game`. For `link` it calls `api::game_from_archive_link`. On API errors it creates a demo session and carries the error in a notice. The default source is the demo. If a loaded game has no accepted moves, it also falls back to the demo.
- `create_session` increments the id, chooses the initial ply (zero for Elo, up to eight for Opening), stores the owned session under the mutex, and redirects to `/play?sid=...`.
- `render_session` looks up the id and either renders the saved game or returns an expired-session notice. Mutex poisoning also produces a user-facing page.
- `update_move` reads the requested ply, falls back to the current ply if parsing fails, clamps the request to `max_ply`, clears old feedback, and redirects back to the session.
- `submit_elo_guess` verifies the session is in Elo mode, invokes `score_one` only for the selected side(s), joins the feedback lines, and marks ratings revealed.
- `score_one` validates an integer guess in `0..=4000`, handles missing actual data, computes absolute error, and calls `rating_is_close` with the session difficulty.
- `submit_opening_guess` verifies the Opening mode, calls `opening_matches`, distinguishes an empty answer from a wrong answer, escapes the revealed opening, and marks the answer revealed.
- `max_ply` returns the complete game length for Elo and at most eight plies for Opening.

The forms use GET query parameters to keep the prototype simple. Answers and usernames can appear in the local URL/history; this is not an account-backed production service.

### Page and board renderers

- `render_home` builds the GTE overview, GothamChess inspiration links, formal project description, mode forms, and demo links.
- `render_game` chooses the mode title and panel, creates black and white player rows, renders the selected board, move controls, move list, game metadata, and source link, then passes the body to `page`.
- `render_elo_panel` creates inputs only for the selected player(s), displays the tolerance, and includes the reveal action.
- `render_opening_panel` shows the short move sequence, answer field, and normalization hint.
- `render_player` displays a hidden marker until the selected player's answer is revealed; unavailable ratings are labeled as unavailable.
- `opening_sequence` formats up to eight moves as numbered SAN text.
- `render_move_list` creates one link for each visible ply and marks the selected ply. It reports an empty move list when no SAN was accepted.
- `render_board` builds all 64 squares, alternating light/dark classes and giving occupied squares accessible labels. `mini_board` shares the same character-to-piece mapping for a compact board.
- `chess_symbol` maps FEN-like letters to Unicode white/black piece glyphs. `piece_name` maps the type to an accessibility label.

`page` adds the HTML document shell and app `STYLE`. The home/game style uses the chess-inspired charcoal, cream, and green palette. `BOOK_STYLE` separately gives the documentation reader a light paper-like page, sticky chapter menu, readable code, and responsive layout.

### Guidebook and Markdown renderer

`BookChapter` stores a slug, display title, and source string. `BASE_BOOK_CHAPTERS` uses `include_str!` to embed the stable `docs/book/src/*.md` chapters and `exercises/reference.md` into the binary at compile time. `book_chapters` parses the generated markers in `docs/book/src/function-pages.md` and appends one Guidebook route for every production function. After editing book source, restart/rebuild the Rust server for the new text to appear.

- `book_chapters` combines the fixed chapters with the generated function pages; `book_chapter_index` finds a slug in that full catalog.
- `render_book` creates the sidebar links, active chapter, breadcrumb, rendered Markdown, and previous/next links.
- `download_book_markdown` concatenates every embedded chapter and function page into `GTE-Guidebook.md` content.
- `render_markdown` parses fenced code, headings, horizontal rules, blockquotes, lists, and paragraphs.
- `flush_markdown_paragraph`, `open_markdown_list`, and `close_markdown_list` manage grouped HTML blocks.
- `markdown_heading` accepts one to six leading `#` characters followed by a space. `heading_slug` produces fragment ids from headings.
- `inline_markdown` escapes ordinary characters and handles inline code, strong emphasis, and `[label](target)` links.
- `safe_markdown_href` allows `https://`, `http://`, `/guide...`, and `#...` targets; `reference.md#...` links are mapped to `/guide/reference#...`.

This is a small subset of Markdown rather than a full CommonMark parser. It intentionally has no raw HTML feature; rendered text is escaped before the limited tags are generated.

### HTTP and input helpers

- `write_response` writes an HTTP status, MIME type, byte count, close-connection header, and `nosniff` header, then writes the body.
- `write_download` sends the Guidebook Markdown as an attachment.
- `write_pdf` sends `output/pdf/GTE-Function-Appendix.pdf` with the PDF MIME type and an attachment filename so the Guidebook link downloads a viewer-ready file. The generated appendix includes one bookmark per production function.
- `parse_query` splits `&` pairs and uses `url_decode` for keys and values; duplicate keys resolve to the last collected value.
- `url_decode` maps plus signs to spaces and valid `%HH` escapes to bytes, then decodes UTF-8 lossily.
- `hex_value` converts one ASCII hex digit or rejects it.
- `html_escape` replaces `&`, `<`, `>`, double quote, and apostrophe.
- `normalize_difficulty` accepts Easy, Medium, Hard, and the typo `harm`; unknown input defaults to Medium.
- `normalize_target` accepts White or Black; other values default to Both.

## `src/cli.rs`: Rustlings-style terminal curriculum

### Terminal setup and writer

`run` enables raw mode, enters the alternate screen, hides the cursor, creates `UiState`, and immediately checks the first incomplete exercise. It polls keyboard events in 200 ms intervals. If the active source file's modification time changes while on the watch or solution screen, it reruns the check. On exit it restores the cursor, leaves the alternate screen, disables raw mode, and prints the Rustlings-style farewell.

`CrLfWriter<W>` wraps the terminal writer and converts lone line-feed bytes into CRLF. Raw mode disables the terminal's usual output newline translation; without this adapter each drawn line can begin at the previous line's column and drift sideways. Its `Write` implementation forwards normalized bytes and `flush` forwards to the inner writer.

### TUI state

`Screen` enumerates Home, List, Lesson, Check, Solution, TestResult, and ResetConfirm. `UiState` holds selected/current lesson indexes, check and solution offsets, hint visibility, started/completed id sets, message text, loaded solution lines, captured test output, pass status, and last observed source mtime.

`UiState::new` loads `.gte-state.txt`, then picks the first incomplete lesson. If all are complete, it opens lesson zero for review. `lesson()` returns the selected static lesson record. The `started` set drives the in-progress marker; the `completed` set drives the green check and progress count.

### Key handlers and navigation

`handle_key` dispatches to the current screen. Reset is global: `X` moves to ResetConfirm, with the currently selected list item as the target when the picker is open. `?` writes a screen-specific key guide.

- Home: `H` toggles the hint, `L` opens the list, `C` checks all targets, `R` reruns the current target, `N` advances after success, `P` checks the previous exercise, and `G` prints a Guidebook address.
- List: arrows or `N`/`P` move the selection; Enter makes that lesson current and runs its target; `L`, Escape, or Backspace returns.
- Lesson: run, hint, concept check, solution, Guidebook, list, and next/previous actions are available.
- Check: arrow keys or `1`–`3` choose an answer. Enter compares it with the stored answer index.
- Solution: arrows and PageUp/PageDown scroll; `R` can rerun the target; `N`/`P` navigate after completion.
- TestResult: the output summary can be rerun or the matching solution opened.
- ResetConfirm: `Y` restores the source, while `N`, Escape, or Backspace cancels.

`open_lesson_list` syncs the selection with the current lesson. `advance_next` refuses to skip an incomplete exercise, searches for the next incomplete id after the current one, and wraps to earlier lessons if needed. `advance_previous` selects the immediately earlier index and checks it.

### Exercise paths, progress, and reset

`source_path` converts hyphens in a lesson id to underscores and joins the filename under `exercises/src/`. `source_modified` reads its filesystem modification time.

`progress_path` locates `.gte-state.txt` in the project root. `load_progress` matches saved ids against the current static lesson array, ignoring stale entries. `save_progress` writes completed ids in lesson order. The file is local progress state and is ignored by Git.

`reset_current_exercise` asks Git for `HEAD:exercises/src/{file}.rs`. Before writing that starter back, it creates `.gte-backup/` and copies the current source there. It removes that lesson from progress and reruns the target. This is a per-exercise reset, not a delete-all command.

### Rendering and Cargo execution

`draw` clears the frame, paints the GTE wordmark, calls the active screen renderer, and flushes. `draw_home` renders the progress bar, current file, task, optional hint, and a terminal-height-limited tail of compiler/test output. `draw_list` marks not-started, in-progress, and completed lessons. `draw_lesson` shows task, path, hint, and links to reference/solution actions. `draw_check` shows the selected multiple-choice answer. `draw_solution` displays successful test names and the final test-result line before the matching code. `draw_test_result` shows batch or focused output. `draw_reset_confirm` describes the file restoration and backup.

`run_current_test` marks the id started and calls `run_lesson_test`. A successful exit inserts the id into completion, saves progress, reads the matching `solution_file`, and opens Solution. A failed exit removes completion and returns to the watch screen with compiler/test output. `run_all_tests` invokes every target in sequence, updates each id, saves the combined progress, and displays a summary.

`run_lesson_test` runs Cargo with the `exercises/Cargo.toml` manifest and the transformed test target. It combines captured standard output and standard error for display. It does not invoke a binary named after the exercise; the exercise name is an integration-test target.

If raw mode is unavailable, `plain_fallback` prints a readable list and the Guidebook URL instead of leaving the user with a blank screen.

## Exercise files, tests, and solutions

Every exercise source includes a `NOTE:` line describing expected behavior and a `TODO:` line showing where to implement it. Every task Markdown file contains its prompt, check, hint, NOTE/TODO explanation, and a `reference.md` deep link. Question 0 is fully implemented by design; its TODO asks the learner to run the check and inspect the code map. Easy variants are guided, Medium variants provide partial implementations, and Hard variants leave the complete function to the learner.

- `intro_0.rs` and target `intro_0`: checks the GTE mark, both entry points, and component map. It is the prefilled green first run.
- `rust_basics.rs`: models Guess the Elo/Opening with an enum, derives equality for test macros, and returns labels. Its test uses `assert_eq!` and `assert_ne!`.
- `archive_api.rs`: selects newest archive URLs first up to a limit and validates public usernames.
- `pgn_board.rs`: removes PGN scaffolding from SAN tokens and reports starting-plus-move position count.
- `elo_scoring.rs`: maps difficulty to a threshold and checks inclusive absolute error.
- `opening_answer.rs`: compares opening names without case or extra whitespace sensitivity.
- `server_rendering.rs`: constructs a session/ply URL and escapes user text for HTML.
- `capstone.rs`: combines ply clamping, selected side(s), rating tolerance, and opening matching on a small `GameRecord`.

There are 16 tier-specific variants in addition to Question 0 and the seven original Medium exercises: `codebase_medium.rs`, `codebase_hard.rs`, then `_easy.rs` and `_hard.rs` variants for Rust basics, archive API, PGN/board state, Elo scoring, opening answers, server rendering, and the capstone. Each has its own integration test and solution. Together these make the 24 exercise targets.

The tests are ordinary Rust `#[test]` functions. Each integration-test file imports only its corresponding learner file, and assertions define the behavior. For example, the rust-basics test cannot compile until `GameMode` derives `PartialEq`; compiler help is part of the lesson.

The matching files under `solutions/` are readable complete examples. They are loaded for viewing after a passing exercise; the runner does not silently replace the learner's source with the solution.

## Documentation files and generated book

`docs/book/src/` is the source for each Guidebook chapter. `docs/book/SUMMARY.md` is the human-readable chapter order. `docs/book/README.md` explains the local book and downloads. The actual web server chapter catalog is assembled from `BASE_BOOK_CHAPTERS` and the generated function pages in `src/web.rs`; source is embedded with `include_str!` at compile time. `render_book` turns entries into chapter pages, `/guide/download` concatenates them into Markdown, and `/guide/functions.pdf` serves the generated appendix.

The Markdown renderer supports headings, paragraphs, lists, horizontal rules, blockquotes, fenced code, inline code, strong text, and safe links. It does not implement all Markdown extensions. For new Guidebook material, keep syntax within that supported set.

## Capstone and end-to-end flow

The final exercise models a compact version of the app flow with a local `GameRecord`. Its functions clamp a ply, score the selected player(s) with the correct tolerance, and normalize the opening answer. The capstone tests compose these behaviors without a network call.

The full browser app path provides the production comparison:

1. Open `/` and follow a demo link to `/load?mode=elo&source=demo`.
2. `route` parses the request and calls `load_game`.
3. `ChessGame::demo` parses its PGN, extracts headers, applies supported moves, and stores board snapshots.
4. `create_session` stores the game and default settings, then redirects to `/play?sid=1`.
5. `render_session` finds the session; `render_game` gets the current position, player rows, move list, and Elo form.
6. Following a move link calls `update_move`, clamps the requested ply, clears feedback, and redirects to the session.
7. Submitting guesses calls `submit_elo_guess`; `score_one` validates and calculates an error for each selected rating.
8. The response redirects to `/play`; the state now has `revealed = true`, so the next render displays ratings and feedback.

Opening mode uses the same setup and board rendering, but starts at up to ply eight and submits a text answer to `submit_opening_guess`.

## Boundaries of the current prototype

- Chess.com access is public and read-only. Random mode samples one player's archives; no global random feed exists.
- Network calls are synchronous. Archive requests are serial.
- The web server handles one connection at a time and keeps sessions only in memory.
- Game forms use GET, so answers are URL query parameters. The app has no sign-in or database.
- The chess parser applies common SAN but is not a complete legality or PGN implementation.
- Focused Cargo tests execute local learner code with the current user's permissions; the terminal runner is an educational tool, not a security sandbox.

These boundaries keep the code easy to trace. For a chapter-style pass through it, begin with [Question 0](/guide/question-zero), continue through the book chapters, then return here for the complete function and file map.
