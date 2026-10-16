#!/usr/bin/env python3
"""Generate one Guidebook page per production Rust function, across nested modules.

The scanner masks comments and literals before recognizing Rust items. This is
not a general Rust parser: it deliberately covers the application source, while
keeping source excerpts byte-for-byte faithful to the checked-in implementation.
"""

from __future__ import annotations

import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
OUTPUT = ROOT / "docs/book/src/function-pages.md"


def source_paths() -> list[Path]:
    """Discover source modules, including functions moved into module folders."""
    paths = (path for path in (ROOT / "src").rglob("*.rs")
             if "tests" not in path.relative_to(ROOT / "src").with_suffix("").parts)
    return sorted(paths, key=lambda path: (path.name != "main.rs", path.relative_to(ROOT).as_posix()))


def module_path(path: Path) -> str:
    parts = list(path.relative_to(ROOT / "src").with_suffix("").parts)
    if parts[-1] == "mod":
        parts.pop()
    return "::".join(parts)


def mask_non_code(source: str) -> str:
    """Replace comments and literals with spaces, preserving offsets/newlines."""
    masked = list(source)
    index = 0

    def erase(start: int, end: int) -> None:
        for offset in range(start, end):
            if source[offset] != "\n":
                masked[offset] = " "

    while index < len(source):
        start = index
        if source.startswith("//", index):
            index = source.find("\n", index)
            if index < 0:
                index = len(source)
        elif source.startswith("/*", index):
            depth = 1
            index += 2
            while index < len(source) and depth:
                if source.startswith("/*", index):
                    depth += 1
                    index += 2
                elif source.startswith("*/", index):
                    depth -= 1
                    index += 2
                else:
                    index += 1
        else:
            raw = re.match(r'(?:br|cr|r)(#*)"', source[index:])
            if raw:
                delimiter = '"' + raw.group(1)
                end = source.find(delimiter, index + raw.end())
                index = len(source) if end < 0 else end + len(delimiter)
            elif source[index] == '"':
                index += 1
                while index < len(source):
                    if source[index] == "\\":
                        index += 2
                    elif source[index] == '"':
                        index += 1
                        break
                    else:
                        index += 1
            elif source[index] == "'":
                char = re.match(r"'(?:\\(?:u\{[0-9a-fA-F_]+\}|x[0-9a-fA-F]{2}|.)|[^'\\\n])'", source[index:])
                if char:
                    index += char.end()
                else:
                    index += 1
                    continue
            else:
                index += 1
                continue
        erase(start, min(index, len(source)))
    return "".join(masked)


def body_span(masked: str, start: int) -> tuple[int, int]:
    opening = masked.find("{", start)
    if opening < 0:
        raise ValueError(f"Missing function body after offset {start}")
    depth = 1
    index = opening + 1
    while index < len(masked):
        if masked[index] == "{":
            depth += 1
        elif masked[index] == "}":
            depth -= 1
            if depth == 0:
                return opening, index + 1
        index += 1
    raise ValueError(f"Unclosed function body after offset {start}")


def without_generics(value: str) -> str:
    return re.sub(r"<[^<>]*>", "", value).strip()


def impl_owner(declaration: str) -> tuple[str, str]:
    declaration = " ".join(declaration.split())
    if declaration.startswith("<"):
        depth = 0
        for index, char in enumerate(declaration):
            depth += char == "<"
            depth -= char == ">"
            if depth == 0:
                declaration = declaration[index + 1:].strip()
                break
    declaration = declaration.split(" where ", 1)[0].strip()
    if " for " in declaration:
        trait, target = declaration.split(" for ", 1)
        return f"<{target} as {trait}>", without_generics(target).rsplit("::", 1)[-1]
    return declaration, without_generics(declaration).rsplit("::", 1)[-1]


def containers(masked: str) -> list[dict[str, str | int]]:
    result = []
    pattern = re.compile(r"(?m)^[ \t]*(?:unsafe[ \t]+)?impl\b([^{}]*?)\{")
    for match in pattern.finditer(masked):
        opening, end = body_span(masked, match.start())
        owner, lookup_owner = impl_owner(match.group(1))
        result.append({"start": opening, "end": end, "owner": owner, "lookup_owner": lookup_owner})
    return result


def slug_piece(value: str) -> str:
    return re.sub(r"[^a-z0-9]+", "-", value.lower()).strip("-")


ROLE = {
    "main::main": "Selects the terminal curriculum or browser app from the process arguments.",
    "api::random_public_game": "Loads a player's archive index, checks recent monthly archives, filters usable chess games, and selects one candidate.",
    "api::game_from_archive_link": "Resolves a pasted public game URL by searching the selected player's recent archive records.",
    "api::validate_username": "Normalizes and validates a public Chess.com username before it becomes part of a request URL.",
    "api::archive_urls": "Requests and extracts the monthly archive URLs returned for a player.",
    "api::get_json": "Performs a public GET request and turns either JSON or transport errors into a Rust Result.",
    "api::to_chess_game": "Converts one archive JSON value into the app's shared chess model, preserving absent ratings as None.",
    "api::same_game_id": "Compares the stable final game id instead of requiring two URLs to have identical formatting.",
    "api::game_id": "Extracts the last nonempty URL segment and removes any query string.",
    "api::opening_from_eco": "Builds a readable opening label from the final path segment of an ECO URL.",
    "chess::ChessGame::from_pgn": "Provides the simple constructor when no API metadata overrides are available.",
    "chess::ChessGame::with_metadata": "Combines API overrides and PGN headers, applies accepted SAN moves, and stores an initial board plus each resulting snapshot.",
    "chess::ChessGame::demo": "Creates the offline Italian Game fixture used by the app when no archive request is selected or available.",
    "chess::ChessGame::position_at": "Returns a saved board snapshot while clamping an oversized ply to the final position.",
    "chess::ChessGame::ply_count": "Reports the number of board transitions without underflowing an empty snapshot list.",
    "chess::normalize_answer": "Canonicalizes answer text by collapsing whitespace and lowercasing it.",
    "chess::opening_matches": "Rejects blank guesses and compares normalized opening names.",
    "chess::rating_tolerance": "Maps Easy, Medium, and Hard difficulty names to their rating-error thresholds.",
    "chess::rating_is_close": "Applies an inclusive absolute-error comparison using the selected tolerance.",
    "chess::pgn_headers": "Reads bracketed PGN header records into a key/value map and unescapes quoted values.",
    "chess::pgn_move_tokens": "Removes header lines, comments, and variations before returning whitespace-separated move tokens.",
    "chess::header": "Returns one borrowed metadata value without copying the header string.",
    "chess::parse_rating": "Parses a positive integer rating and represents invalid or nonpositive input as None.",
    "chess::is_result": "Recognizes the four PGN result markers so they are not mistaken for moves.",
    "chess::strip_move_number": "Removes numeric and period prefixes such as 1. or 12... from a PGN token.",
    "chess::starting_position": "Creates the row-major 64-square starting board used before the first ply.",
    "chess::apply_san": "Interprets common SAN, finds a plausible origin square, mutates the board, and reports whether a move was applied.",
    "chess::disambiguation_matches": "Filters possible SAN origins by a specified source file or rank.",
    "chess::piece_can_move": "Checks movement shape, pawn direction, captures, and sliding-piece obstruction for one candidate origin.",
    "chess::path_is_clear": "Walks intermediate squares and rejects a sliding move when a piece blocks the path.",
    "cli::CrLfWriter::new": "Initializes the writer adapter that keeps track of whether the last byte was a carriage return.",
    "cli::CrLfWriter::write": "Adapts line-feed bytes for raw-mode terminals so output lines return to column one.",
    "cli::CrLfWriter::flush": "Forwards an explicit flush to the wrapped terminal writer.",
    "cli::UiState::new": "Loads saved lesson completion and selects the first exercise that still needs work.",
    "cli::UiState::lesson": "Looks up the static lesson currently selected by the UI state.",
    "cli::run": "Owns the terminal lifecycle: raw mode, first check, input polling, redraws, and cleanup.",
    "cli::handle_key": "Routes a key press according to the current screen and changes navigation or exercise state.",
    "cli::handle_home": "Dispatches the home screen's navigation, hint, focused check, all-check, and Guidebook shortcuts.",
    "cli::handle_list": "Moves the exercise picker selection, opens the selected exercise, and returns to the home screen.",
    "cli::handle_lesson": "Handles one exercise's hint, concept check, solution, Guidebook, and navigation shortcuts.",
    "cli::handle_check": "Selects and validates a concept-check answer while keeping exercise navigation available.",
    "cli::handle_solution": "Scrolls a reference solution and routes rerun, previous/next exercise, and screen navigation keys.",
    "cli::handle_test_result": "Scrolls captured diagnostics and handles reruns, solution access, and exercise navigation.",
    "cli::handle_reset_confirm": "Accepts a source reset, cancels back to the home screen, or exits the terminal UI.",
    "cli::open_lesson_list": "Copies the active lesson into the picker selection and opens the list screen.",
    "cli::advance_next": "Blocks forward navigation until the current exercise passes, then selects and runs the next incomplete lesson.",
    "cli::advance_previous": "Moves one lesson backward and immediately checks that lesson.",
    "cli::source_path": "Maps a lesson id to its learner-editable Rust file under exercises/src.",
    "cli::source_modified": "Reads the active exercise file's modification time for automatic check reruns.",
    "cli::reset_current_exercise": "Backs up the active source, restores its Git HEAD version, clears its progress, and reruns it.",
    "cli::progress_path": "Builds the path to the repository-local ignored completion file.",
    "cli::load_progress": "Reads saved ids and keeps only ids that still exist in the current lesson catalog.",
    "cli::save_progress": "Writes completed ids in lesson order to make the progress file stable and readable.",
    "cli::draw": "Clears the frame, paints the GTE terminal wordmark, and dispatches to the active screen renderer.",
    "cli::draw_home": "Shows progress, the current task, optional hint, and a terminal-height-limited slice of check output.",
    "cli::draw_reset_confirm": "Explains which source file will be restored and where its backup will be written.",
    "cli::draw_list": "Renders all exercises with status markers and the keyboard controls for selecting one.",
    "cli::draw_lesson": "Displays a lesson prompt, code location, NOTE/TODO guidance, and available actions.",
    "cli::draw_check": "Renders the concept-check choices and highlights the currently selected answer.",
    "cli::draw_solution": "Shows test names and the final test summary before the matching solution source.",
    "cli::draw_test_result": "Displays the test output for a focused or all-target run and its pass/fail label.",
    "cli::run_current_test": "Runs one lesson target, updates progress, stores diagnostics, and loads that lesson's matching solution on success.",
    "cli::run_all_tests": "Runs every registered exercise target and saves each pass/fail completion result.",
    "cli::run_lesson_test": "Invokes Cargo with one integration-test target and captures both stdout and stderr.",
    "cli::plain_fallback": "Prints a noninteractive lesson summary when terminal raw mode or screen setup is unavailable.",
    "lessons::lesson": "Constructs one static curriculum record that joins its prompt, test, solution, difficulty, and codebase reference.",
    "web::sessions": "Lazily initializes and returns the process-wide mutex-protected browser session map.",
    "web::serve": "Binds the localhost HTTP listener, prints the local URLs, and serially handles connections.",
    "web::handle_connection": "Reads one request line, permits GET, dispatches a route, and writes the matching HTTP response.",
    "web::route": "Splits path from query values and selects the correct page, game handler, download, or 404.",
    "web::load_game": "Normalizes the round settings, selects demo/archive/link data, and keeps a user-readable API fallback notice.",
    "web::create_session": "Allocates a session id, chooses the starting ply for the mode, stores state, and redirects to play.",
    "web::render_session": "Looks up a session id and renders the saved game or explains that the session is unavailable.",
    "web::update_move": "Parses and clamps the requested ply, clears stale feedback, then redirects to the session page.",
    "web::submit_elo_guess": "Scores only the requested player side or sides and marks the session's ratings revealed.",
    "web::score_one": "Validates one numeric guess, handles missing ratings, computes error, and appends tolerance feedback.",
    "web::submit_opening_guess": "Validates the active mode, normalizes the answer through chess.rs, and stores the revealed result.",
    "web::render_home": "Builds the overview, GothamChess link, formal description, challenge forms, and demo links.",
    "web::render_game": "Composes the board, player rows, move list, metadata, navigation, and mode-specific answer panel.",
    "web::render_elo_panel": "Creates the rating form with inputs only for the selected side and the selected tolerance displayed.",
    "web::render_opening_panel": "Builds the opening answer form and shows the visible short sequence of SAN moves.",
    "web::render_player": "Renders a player row while hiding its rating until the round's reveal state permits it.",
    "web::opening_sequence": "Formats up to eight stored SAN plies into conventional numbered chess notation.",
    "web::max_ply": "Applies the mode rule that opening rounds show at most eight plies while Elo can show the full game.",
    "web::render_move_list": "Turns each visible SAN token into a navigation link and marks the selected ply.",
    "web::render_board": "Converts all 64 board cells into labeled squares for the accessible full board.",
    "web::mini_board": "Renders the compact decorative board used on the home page.",
    "web::chess_symbol": "Maps internal piece letters to Unicode chess glyphs, returning an empty string for empty squares.",
    "web::piece_name": "Maps an internal piece letter to a spoken accessibility label.",
    "web::book_chapters": "Builds the complete Guidebook catalog from its stable chapter files and the generated function-page source.",
    "web::book_chapter_index": "Finds the Guidebook page index for a URL slug.",
    "web::render_book": "Builds the Guidebook sidebar, active reading page, breadcrumb, and previous/next navigation.",
    "web::download_book_markdown": "Concatenates the embedded Guidebook pages into the downloadable Markdown edition.",
    "web::render_markdown": "Converts the documented Markdown subset into HTML while tracking paragraphs, lists, and fenced code.",
    "web::flush_markdown_paragraph": "Joins a pending paragraph's wrapped source lines and emits one escaped HTML paragraph.",
    "web::open_markdown_list": "Opens an ordered or unordered list, closing a prior list if its kind changed.",
    "web::close_markdown_list": "Closes the currently open Markdown list, if there is one.",
    "web::markdown_heading": "Recognizes one through six leading Markdown heading markers followed by a space.",
    "web::heading_slug": "Builds a stable fragment id from lowercase alphanumeric heading text.",
    "web::inline_markdown": "Escapes text and recognizes only inline code, strong emphasis, and validated links.",
    "web::safe_markdown_href": "Allows a small set of safe external and local links and rewrites reference.md anchors.",
    "web::page": "Wraps a rendered app body in the shared HTML shell and app stylesheet.",
    "web::write_response": "Writes a standard HTTP response header and its body to the client socket.",
    "web::write_download": "Sends Markdown with an attachment disposition and the selected filename.",
    "web::write_pdf": "Sends the embedded function appendix bytes with an application/pdf content type and download filename.",
    "web::parse_query": "Decodes ampersand-separated key/value pairs into the request parameter map.",
    "web::url_decode": "Decodes plus signs and valid percent escapes before converting bytes into UTF-8 text.",
    "web::hex_value": "Converts one hexadecimal ASCII digit into its numeric nibble value.",
    "web::html_escape": "Replaces five HTML-sensitive characters before untrusted text enters markup.",
    "web::normalize_difficulty": "Restricts user input to easy, medium, or hard and accepts the brief's harm typo as hard.",
    "web::normalize_target": "Restricts user input to white, black, or the default both-player choice.",
}


def responsibility(module: str, name: str, lookup_owner: str) -> str:
    # Responsibility follows a function when its module is split. Trait owners
    # remain distinct in the qualified title, but share their target type's role.
    family = module.split("::", 1)[0]
    stable = f"{family}::{lookup_owner}::{name}" if lookup_owner else f"{family}::{name}"
    return ROLE.get(stable, f"Implements `{name}` in `{module}`. Read its parameters, branches, and return value alongside the neighboring functions in this focused module.")


def functions_in_source(path: Path, source: str) -> list[dict[str, str]]:
    functions = []
    module = module_path(path)
    masked = mask_non_code(source)
    owners = containers(masked)
    inline_modules = []
    for match in re.finditer(r"(?m)^[ \t]*(?:pub(?:\([^)]*\))?[ \t]+)?mod[ \t]+([A-Za-z_][A-Za-z0-9_]*)[ \t]*\{", masked):
        opening, end = body_span(masked, match.start())
        inline_modules.append((opening, end, match.group(1)))
    pattern = re.compile(r"\b(?:pub(?:[ \t]*\([^)]*\))?[ \t]+)?(?:(?:async|const|unsafe|extern)[ \t]+)*fn[ \t]+([A-Za-z_][A-Za-z0-9_]*)\b")
    for match in pattern.finditer(masked):
        enclosing_modules = [name for opening, end, name in inline_modules if opening < match.start() < end]
        if "tests" in enclosing_modules:
            continue
        function_module = "::".join([module, *enclosing_modules])
        body_start, end = body_span(masked, match.start())
        line_start = source.rfind("\n", 0, match.start()) + 1
        if source[line_start:match.start()].strip():
            line_start = match.start()
        owner = next((entry for entry in reversed(owners) if entry["start"] < match.start() < entry["end"]), None)
        owner_name = str(owner["owner"]) if owner else ""
        name = match.group(1)
        qualified = f"{function_module}::{owner_name}::{name}" if owner_name else f"{function_module}::{name}"
        functions.append({
            "module": function_module,
            "owner": owner_name,
            "name": name,
            "qualified": qualified,
            "slug": f"function-{slug_piece(qualified)}",
            "source_path": path.relative_to(ROOT).as_posix(),
            "line": str(source.count("\n", 0, line_start) + 1),
            "signature": source[line_start:body_start].strip(),
            "snippet": source[line_start:end].strip(),
            "role": responsibility(function_module, name, str(owner["lookup_owner"]) if owner else ""),
        })
    return functions


def collect_functions() -> list[dict[str, str]]:
    return [function for path in source_paths() for function in functions_in_source(path, path.read_text())]


def main() -> None:
    functions = collect_functions()
    if len(functions) < 92:
        raise SystemExit(f"Expected at least 92 functions; found {len(functions)}")
    slugs = [function["slug"] for function in functions]
    if len(slugs) != len(set(slugs)):
        duplicates = sorted({slug for slug in slugs if slugs.count(slug) > 1})
        raise SystemExit(f"Function page slugs must be unique: {duplicates}")

    pages = []
    total = len(functions)
    for number, function in enumerate(functions, start=1):
        title = f"{function['module']} · {function['owner'] + '::' if function['owner'] else ''}{function['name']}"
        signature = function["signature"]
        family = function["module"].split("::", 1)[0]
        chapter = {"main": "rust-entry-point", "api": "public-data", "chess": "board-state", "cli": "terminal-learning", "lessons": "terminal-learning", "web": "web-server"}.get(family, "architecture")
        siblings = [other for other in functions if other["module"] == function["module"] and other["qualified"] != function["qualified"]]
        related = ", ".join(f"[`{other['name']}`](/guide/{other['slug']})" for other in siblings)
        related = f"Other functions in this module: {related}." if related else "This module has one function; its supporting types and constants live alongside it."
        body = f'''<!-- GTE_FUNCTION_PAGE: {function['slug']}|{title} -->
# Function page {number:03} of {total:03}: `{function['qualified']}`

**Source:** `{function['source_path']}:{function['line']}`

## Responsibility

{function['role']}

## Signature and data contract

```rust
{signature}
```

Read the parameter types first: owned `String` values can outlive the current call; borrowed `&str` or references read existing storage; `Option<T>` represents data that may be absent; and `Result<T, E>` makes recoverable failure explicit. The return type tells the caller what it can do next. The body below shows which paths this function uses.

## Implementation, line by line

This excerpt is copied from the production source. For each `if`, `match`, or early `return`, identify what input reaches that path and what state the caller observes afterward. When a value changes, follow it through the types named in the signature.

```rust
{function['snippet']}
```

## Where it fits

The implementation lives in the focused `{function['module']}` module at the source path above. Read the [related architecture chapter](/guide/{chapter}) for its larger flow and the [complete codebase walkthrough](/guide/reference) for the module boundaries, related helpers, exercises, and known limits.

{related}

For terminal exercises that touch this behavior, follow the exercise's `reference.md` link and compare its small contract with this production implementation.
'''
        pages.append(f"{body}\n<!-- END_GTE_FUNCTION_PAGE -->")

    OUTPUT.write_text("\n\n".join(pages) + "\n")
    modules = {function["module"] for function in functions}
    print(f"Generated {len(functions)} function pages across {len(modules)} Rust modules in {OUTPUT.relative_to(ROOT)}")


if __name__ == "__main__":
    main()
