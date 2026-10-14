#!/usr/bin/env python3
"""Create the guided and challenge variants that complete the 24 GTE lessons."""

from pathlib import Path
import textwrap

ROOT = Path(__file__).resolve().parents[1]

EXERCISES = [
    {
        "id": "codebase-medium", "title": "Trace a game mode to its module", "topic": "Project orientation", "tier": "Medium",
        "goal": "Use a small enum and an exhaustive match to map each app mode to the Rust module that owns it.",
        "check": "Run the focused `codebase_medium` integration test. It checks both variants and relies on equality traits to compare enum values.",
        "hint": "Match each `GameMode` variant separately and return the stable module path as a string slice.",
        "reference": "question-zero-project-map",
        "source": '''// NOTE: Map each explicit game mode to the module that owns its behavior.\n// TODO: Write the match arms in `owner` and keep both variants distinct.\n#[derive(Clone, Copy, Debug, PartialEq, Eq)]\npub enum GameMode {\n    GuessElo,\n    GuessOpening,\n}\n\npub fn owner(mode: GameMode) -> &'static str {\n    todo!("map each mode to its application module")\n}\n''',
        "solution": '''#[derive(Clone, Copy, Debug, PartialEq, Eq)]\npub enum GameMode {\n    GuessElo,\n    GuessOpening,\n}\n\npub fn owner(mode: GameMode) -> &'static str {\n    match mode {\n        GameMode::GuessElo => "src/web.rs",\n        GameMode::GuessOpening => "src/web.rs",\n    }\n}\n''',
        "test": '''#[path = "../src/codebase_medium.rs"]\nmod exercise;\n\n#[test]\nfn both_modes_are_distinct_and_owned_by_the_web_layer() {\n    let elo = exercise::GameMode::GuessElo;\n    let opening = exercise::GameMode::GuessOpening;\n    assert_ne!(elo, opening);\n    assert_eq!(exercise::owner(elo), "src/web.rs");\n    assert_eq!(exercise::owner(opening), "src/web.rs");\n}\n''',
    },
    {
        "id": "codebase-hard", "title": "Route a request to its owner", "topic": "Project orientation", "tier": "Hard",
        "goal": "Implement a complete route lookup that separates a URL path from its query and identifies the responsible GTE module.",
        "check": "Run the `codebase_hard` test. The cases cover game routes, the Guidebook, query strings, and an unknown path.",
        "hint": "Split once at `?`, then match the path. A Guidebook child path and a game route both belong to `src/web.rs`.",
        "reference": "web-server-and-rendered-pages",
        "source": '''// NOTE: The Rust web module owns route selection for game and Guidebook requests.\n// TODO: Implement this whole function: remove the query string and select the owner.\npub fn route_owner(request_target: &str) -> Option<&'static str> {\n    todo!("map supported GTE request paths to their source module")\n}\n''',
        "solution": '''pub fn route_owner(request_target: &str) -> Option<&'static str> {\n    let path = request_target.split('?').next().unwrap_or(request_target);\n    match path {\n        "/" | "/new" | "/load" | "/play" | "/move" | "/guess-elo" | "/guess-opening" => {\n            Some("src/web.rs")\n        }\n        "/guide" | "/guide/download" | "/guide/functions.pdf" => Some("src/web.rs"),\n        path if path.starts_with("/guide/") => Some("src/web.rs"),\n        _ => None,\n    }\n}\n''',
        "test": '''#[path = "../src/codebase_hard.rs"]\nmod exercise;\n\n#[test]\nfn route_owner_handles_query_strings_and_guide_chapters() {\n    assert_eq!(exercise::route_owner("/play?sid=12"), Some("src/web.rs"));\n    assert_eq!(exercise::route_owner("/guide/function-web-route"), Some("src/web.rs"));\n    assert_eq!(exercise::route_owner("/unknown"), None);\n}\n''',
    },
    {
        "id": "rust-basics-easy", "title": "Label each game mode", "topic": "Rust foundations", "tier": "Easy",
        "goal": "Follow a guided enum match and return the visible label for each mode.",
        "check": "Run `rust_basics_easy`. Equality assertions identify the exact expected label for each mode.",
        "hint": "The enum is already defined. Replace each placeholder string with the matching user-facing label.",
        "reference": "rust-types-and-mode-selection",
        "source": '''// NOTE: Each mode has a fixed label that the interface can display.\n// TODO: Replace the two placeholder strings with the correct labels.\n#[derive(Clone, Copy, Debug, PartialEq, Eq)]\npub enum GameMode {\n    GuessElo,\n    GuessOpening,\n}\n\npub fn label(mode: GameMode) -> &'static str {\n    match mode {\n        GameMode::GuessElo => "???",\n        GameMode::GuessOpening => "???",\n    }\n}\n''',
        "solution": '''#[derive(Clone, Copy, Debug, PartialEq, Eq)]\npub enum GameMode {\n    GuessElo,\n    GuessOpening,\n}\n\npub fn label(mode: GameMode) -> &'static str {\n    match mode {\n        GameMode::GuessElo => "Guess the Elo",\n        GameMode::GuessOpening => "Guess the Opening",\n    }\n}\n''',
        "test": '''#[path = "../src/rust_basics_easy.rs"]\nmod exercise;\n\n#[test]\nfn labels_match_the_mode_names() {\n    assert_eq!(exercise::label(exercise::GameMode::GuessElo), "Guess the Elo");\n    assert_eq!(exercise::label(exercise::GameMode::GuessOpening), "Guess the Opening");\n}\n''',
    },
    {
        "id": "rust-basics-hard", "title": "Parse the requested mode", "topic": "Rust foundations", "tier": "Hard",
        "goal": "Implement a complete parser from command-line mode names into a safe enum value.",
        "check": "Run `rust_basics_hard`. It checks exact names, case sensitivity, and unknown input.",
        "hint": "Use a `match` on `argument`; return `Some` for the two recognized names and `None` otherwise.",
        "reference": "rust-types-and-mode-selection",
        "source": '''// NOTE: Keep external strings out of the app state by converting them to a closed enum.\n// TODO: Implement the complete parser with an exhaustive match and a safe fallback.\n#[derive(Clone, Copy, Debug, PartialEq, Eq)]\npub enum GameMode {\n    GuessElo,\n    GuessOpening,\n}\n\npub fn parse_mode(argument: &str) -> Option<GameMode> {\n    todo!("parse the supported mode names")\n}\n''',
        "solution": '''#[derive(Clone, Copy, Debug, PartialEq, Eq)]\npub enum GameMode {\n    GuessElo,\n    GuessOpening,\n}\n\npub fn parse_mode(argument: &str) -> Option<GameMode> {\n    match argument {\n        "elo" => Some(GameMode::GuessElo),\n        "opening" => Some(GameMode::GuessOpening),\n        _ => None,\n    }\n}\n''',
        "test": '''#[path = "../src/rust_basics_hard.rs"]\nmod exercise;\n\n#[test]\nfn parser_accepts_only_the_two_supported_mode_names() {\n    assert_eq!(exercise::parse_mode("elo"), Some(exercise::GameMode::GuessElo));\n    assert_eq!(exercise::parse_mode("opening"), Some(exercise::GameMode::GuessOpening));\n    assert_eq!(exercise::parse_mode("Opening"), None);\n    assert_eq!(exercise::parse_mode("other"), None);\n}\n''',
    },
    {
        "id": "archive-api-easy", "title": "Select the newest archives", "topic": "Chess.com data", "tier": "Easy",
        "goal": "Use a reverse iterator and a limit to select recent archive URLs from a list ordered oldest first.",
        "check": "Run `archive_api_easy`. It checks newest-first ordering and the zero-limit edge case.",
        "hint": "The loop already visits the newest entries first. Add each visited URL to the output vector.",
        "reference": "chesscom-archive-data",
        "source": '''// NOTE: Chess.com returns archive months oldest first; GTE reviews recent months first.\n// TODO: Add each visited URL to `selected` (the exercise is guided one line at a time).\npub fn recent_archives<'a>(urls: &[&'a str], limit: usize) -> Vec<&'a str> {\n    let mut selected = Vec::new();\n    for _archive in urls.iter().rev().take(limit) {\n        // TODO: selected.push(???);\n    }\n    selected\n}\n''',
        "solution": '''pub fn recent_archives<'a>(urls: &[&'a str], limit: usize) -> Vec<&'a str> {\n    let mut selected = Vec::new();\n    for archive in urls.iter().rev().take(limit) {\n        selected.push(*archive);\n    }\n    selected\n}\n''',
        "test": '''#[path = "../src/archive_api_easy.rs"]\nmod exercise;\n\n#[test]\nfn archives_are_limited_and_returned_newest_first() {\n    let urls = ["jan", "feb", "mar", "apr"];\n    assert_eq!(exercise::recent_archives(&urls, 2), vec!["apr", "mar"]);\n    assert_eq!(exercise::recent_archives(&urls, 0), Vec::<&str>::new());\n}\n''',
    },
    {
        "id": "archive-api-hard", "title": "Find the newest usable month", "topic": "Chess.com data", "tier": "Hard",
        "goal": "Write a complete function that finds the newest archive month containing usable games.",
        "check": "Run `archive_api_hard`. It checks empty recent months, a valid month, and the no-games case.",
        "hint": "Scan the month slices in reverse order and return the first slice that is not empty.",
        "reference": "chesscom-archive-data",
        "source": '''// NOTE: Recent archive months may be empty, so keep searching until a month has games.\n// TODO: Write the complete newest-first search and return None when every month is empty.\npub fn newest_month_with_games(archives: &[Vec<String>]) -> Option<&[String]> {\n    todo!("find the newest archive month containing at least one game")\n}\n''',
        "solution": '''pub fn newest_month_with_games(archives: &[Vec<String>]) -> Option<&[String]> {\n    archives\n        .iter()\n        .rev()\n        .find(|games| !games.is_empty())\n        .map(Vec::as_slice)\n}\n''',
        "test": '''#[path = "../src/archive_api_hard.rs"]\nmod exercise;\n\n#[test]\nfn search_skips_empty_months_and_stops_at_the_newest_usable_one() {\n    let archives = vec![vec!["old".into()], vec![], vec!["new".into()], vec![]];\n    assert_eq!(exercise::newest_month_with_games(&archives), Some(["new".to_owned()].as_slice()));\n    assert_eq!(exercise::newest_month_with_games(&[vec![], vec![]]), None);\n}\n''',
    },
    {
        "id": "pgn-board-easy", "title": "Recognize a move token", "topic": "PGN and board state", "tier": "Easy",
        "goal": "Separate ordinary SAN tokens from move numbers, game results, and annotation tokens.",
        "check": "Run `pgn_board_easy`. The examples include pawn moves, castling, numbering, results, and NAG markers.",
        "hint": "A move is not blank, a move number, a result marker, or a token beginning with `$`.",
        "reference": "pgn-parsing-and-board-positions",
        "source": '''// NOTE: PGN contains metadata tokens as well as moves; only move tokens belong on the board.\n// TODO: Exclude blank text, move numbers, results, and numeric annotation glyphs.\npub fn is_san_move(token: &str) -> bool {\n    !token.is_empty() && token.chars().all(|character| character.is_ascii_alphabetic())\n}\n''',
        "solution": '''pub fn is_san_move(token: &str) -> bool {\n    !token.is_empty()\n        && !token.starts_with('$')\n        && !matches!(token, "1-0" | "0-1" | "1/2-1/2" | "*")\n        && !token.chars().all(|character| character.is_ascii_digit() || character == '.')\n}\n''',
        "test": '''#[path = "../src/pgn_board_easy.rs"]\nmod exercise;\n\n#[test]\nfn move_recognizer_filters_pgn_scaffolding() {\n    for token in ["e4", "Nf3", "O-O", "Qxe5+"] {\n        assert!(exercise::is_san_move(token), "expected move: {token}");\n    }\n    for token in ["1.", "1...", "1-0", "*", "$1", ""] {\n        assert!(!exercise::is_san_move(token), "expected metadata: {token}");\n    }\n}\n''',
    },
    {
        "id": "pgn-board-hard", "title": "Navigate a saved position", "topic": "PGN and board state", "tier": "Hard",
        "goal": "Implement a complete bounded move-selection function over saved board snapshots.",
        "check": "Run `pgn_board_hard`. It verifies the starting position, a middle ply, the final ply, and requests beyond the game.",
        "hint": "The snapshots include index zero for the initial board. Clamp the requested ply to the last valid index before indexing.",
        "reference": "pgn-parsing-and-board-positions",
        "source": '''// NOTE: Saved positions include the initial board and one snapshot for every accepted ply.\n// TODO: Implement a safe lookup that clamps requests to the final available snapshot.\npub fn position_at(positions: &[String], requested_ply: usize) -> Option<&str> {\n    todo!("select a saved position without indexing past the snapshot list")\n}\n''',
        "solution": '''pub fn position_at(positions: &[String], requested_ply: usize) -> Option<&str> {\n    let final_ply = positions.len().saturating_sub(1);\n    positions.get(requested_ply.min(final_ply)).map(String::as_str)\n}\n''',
        "test": '''#[path = "../src/pgn_board_hard.rs"]\nmod exercise;\n\n#[test]\nfn position_lookup_includes_the_start_and_clamps_to_the_end() {\n    let positions = vec!["start".into(), "after e4".into(), "after e5".into()];\n    assert_eq!(exercise::position_at(&positions, 0), Some("start"));\n    assert_eq!(exercise::position_at(&positions, 1), Some("after e4"));\n    assert_eq!(exercise::position_at(&positions, 99), Some("after e5"));\n    assert_eq!(exercise::position_at(&[], 0), None);\n}\n''',
    },
    {
        "id": "elo-scoring-easy", "title": "Calculate rating error", "topic": "Guess the Elo", "tier": "Easy",
        "goal": "Calculate how far a guess is from the actual Elo rating using a non-negative absolute error.",
        "check": "Run `elo_scoring_easy`. It checks guesses above and below the actual rating.",
        "hint": "The subtraction order should not change the size of the error. Use the integer absolute-value method.",
        "reference": "elo-scoring",
        "source": '''// NOTE: A rating miss is an absolute distance, regardless of which rating was larger.\n// TODO: Convert the signed difference to a non-negative value before returning it.\npub fn rating_error(guess: i32, actual: i32) -> i32 {\n    let difference = guess - actual;\n    difference\n}\n''',
        "solution": '''pub fn rating_error(guess: i32, actual: i32) -> i32 {\n    (guess - actual).abs()\n}\n''',
        "test": '''#[path = "../src/elo_scoring_easy.rs"]\nmod exercise;\n\n#[test]\nfn error_is_the_same_on_either_side_of_the_actual_rating() {\n    assert_eq!(exercise::rating_error(1400, 1500), 100);\n    assert_eq!(exercise::rating_error(1600, 1500), 100);\n    assert_eq!(exercise::rating_error(1500, 1500), 0);\n}\n''',
    },
    {
        "id": "elo-scoring-hard", "title": "Evaluate a possibly missing guess", "topic": "Guess the Elo", "tier": "Hard",
        "goal": "Write a complete scoring function that handles missing ratings and applies an inclusive tolerance.",
        "check": "Run `elo_scoring_hard`. It checks both missing-value cases, an exact boundary, and an out-of-range guess.",
        "hint": "Use `?` on the two optional ratings, reject a negative tolerance, then compare the absolute error.",
        "reference": "elo-scoring",
        "source": '''// NOTE: A result is unknown when either the guess or the archived rating is missing.\n// TODO: Implement the full optional-value and inclusive-tolerance contract.\npub fn evaluate_guess(guess: Option<i32>, actual: Option<i32>, tolerance: i32) -> Option<bool> {\n    todo!("score a known rating guess and preserve missing data")\n}\n''',
        "solution": '''pub fn evaluate_guess(guess: Option<i32>, actual: Option<i32>, tolerance: i32) -> Option<bool> {\n    let guess = guess?;\n    let actual = actual?;\n    if tolerance < 0 {\n        return None;\n    }\n    Some((guess - actual).abs() <= tolerance)\n}\n''',
        "test": '''#[path = "../src/elo_scoring_hard.rs"]\nmod exercise;\n\n#[test]\nfn scoring_preserves_missing_values_and_includes_the_limit() {\n    assert_eq!(exercise::evaluate_guess(None, Some(1500), 100), None);\n    assert_eq!(exercise::evaluate_guess(Some(1400), None, 100), None);\n    assert_eq!(exercise::evaluate_guess(Some(1400), Some(1500), 100), Some(true));\n    assert_eq!(exercise::evaluate_guess(Some(1399), Some(1500), 100), Some(false));\n    assert_eq!(exercise::evaluate_guess(Some(1500), Some(1500), -1), None);\n}\n''',
    },
    {
        "id": "opening-answer-easy", "title": "Normalize an opening name", "topic": "Guess the Opening", "tier": "Easy",
        "goal": "Use whitespace-aware splitting to make an opening name insensitive to spacing and capitalization.",
        "check": "Run `opening_answer_easy`. It checks normalized case, repeated spaces, and empty input.",
        "hint": "The word joining line is already done. Convert the joined string to ASCII lowercase before returning it.",
        "reference": "opening-answers",
        "source": '''// NOTE: Opening names are compared as words, not by the user's capitalization or spacing.\n// TODO: Normalize the joined words to lowercase before returning the value.\npub fn normalize_opening(answer: &str) -> String {\n    let words: Vec<&str> = answer.split_whitespace().collect();\n    words.join(" ")\n}\n''',
        "solution": '''pub fn normalize_opening(answer: &str) -> String {\n    answer\n        .split_whitespace()\n        .collect::<Vec<_>>()\n        .join(" ")\n        .to_ascii_lowercase()\n}\n''',
        "test": '''#[path = "../src/opening_answer_easy.rs"]\nmod exercise;\n\n#[test]\nfn normalizer_collapses_spaces_and_lowercases_text() {\n    assert_eq!(exercise::normalize_opening("  ITALIAN   GAME "), "italian game");\n    assert_eq!(exercise::normalize_opening("   "), "");\n}\n''',
    },
    {
        "id": "opening-answer-hard", "title": "Match an accepted opening alias", "topic": "Guess the Opening", "tier": "Hard",
        "goal": "Implement a complete case- and whitespace-insensitive matcher for a canonical opening name and its aliases.",
        "check": "Run `opening_answer_hard`. It checks aliases, normalization, empty guesses, and a wrong answer.",
        "hint": "Normalize the guess once, reject it if blank, and compare it with each normalized accepted name.",
        "reference": "opening-answers",
        "source": '''// NOTE: Chess.com labels and human-readable opening names can use different accepted names.\n// TODO: Implement the full alias comparison and reject an empty normalized guess.\npub fn matches_opening(guess: &str, accepted_names: &[&str]) -> bool {\n    todo!("compare a guess with every accepted opening name")\n}\n''',
        "solution": '''pub fn matches_opening(guess: &str, accepted_names: &[&str]) -> bool {\n    let normalize = |value: &str| {\n        value.split_whitespace().collect::<Vec<_>>().join(" ").to_ascii_lowercase()\n    };\n    let guess = normalize(guess);\n    !guess.is_empty() && accepted_names.iter().any(|name| normalize(name) == guess)\n}\n''',
        "test": '''#[path = "../src/opening_answer_hard.rs"]\nmod exercise;\n\n#[test]\nfn aliases_match_without_case_or_spacing_sensitivity() {\n    let names = ["Italian Game", "Giuoco Piano"];\n    assert!(exercise::matches_opening("  GIUOCO   PIANO ", &names));\n    assert!(exercise::matches_opening("italian game", &names));\n    assert!(!exercise::matches_opening("   ", &names));\n    assert!(!exercise::matches_opening("Sicilian Defense", &names));\n}\n''',
    },
    {
        "id": "server-rendering-easy", "title": "Build a move-navigation URL", "topic": "Web app and Guidebook", "tier": "Easy",
        "goal": "Complete a guided URL that carries both the session id and requested ply back to the Rust server.",
        "check": "Run `server_rendering_easy`. Exact-string assertions verify both query parameters and their order.",
        "hint": "Append the `ply` query parameter to the already-built session URL using `format!`.",
        "reference": "web-server-and-rendered-pages",
        "source": '''// NOTE: The move link must identify both the active game session and the selected ply.\n// TODO: Add `&ply={ply}` to the already-built session URL.\npub fn move_url(session_id: &str, ply: usize) -> String {\n    let base = format!("/move?sid={session_id}");\n    base\n}\n''',
        "solution": '''pub fn move_url(session_id: &str, ply: usize) -> String {\n    format!("/move?sid={session_id}&ply={ply}")\n}\n''',
        "test": '''#[path = "../src/server_rendering_easy.rs"]\nmod exercise;\n\n#[test]\nfn move_url_carries_the_session_and_requested_ply() {\n    assert_eq!(exercise::move_url("42", 7), "/move?sid=42&ply=7");\n}\n''',
    },
    {
        "id": "server-rendering-hard", "title": "Render an escaped notice", "topic": "Web app and Guidebook", "tier": "Hard",
        "goal": "Write a complete notice renderer and escape untrusted title and message text before placing it in HTML.",
        "check": "Run `server_rendering_hard`. It checks all five escaped HTML characters and confirms trusted wrapper markup remains intact.",
        "hint": "Write an escape loop over characters; map `&`, `<`, `>`, double quote, and apostrophe, then format the wrapper.",
        "reference": "web-server-and-rendered-pages",
        "source": '''// NOTE: HTML wrapper text is trusted, but title and message values can be user supplied.\n// TODO: Implement this whole renderer and escape the two values before interpolation.\npub fn render_notice(title: &str, message: &str) -> String {\n    todo!("escape content and render the notice panel")\n}\n''',
        "solution": '''pub fn render_notice(title: &str, message: &str) -> String {\n    fn escape(text: &str) -> String {\n        let mut escaped = String::new();\n        for character in text.chars() {\n            match character {\n                '&' => escaped.push_str("&amp;"),\n                '<' => escaped.push_str("&lt;"),\n                '>' => escaped.push_str("&gt;"),\n                '"' => escaped.push_str("&quot;"),\n                '\\'' => escaped.push_str("&#39;"),\n                other => escaped.push(other),\n            }\n        }\n        escaped\n    }\n    format!("<aside><strong>{}</strong><p>{}</p></aside>", escape(title), escape(message))\n}\n''',
        "test": '''#[path = "../src/server_rendering_hard.rs"]\nmod exercise;\n\n#[test]\nfn renderer_escapes_untrusted_text_and_keeps_its_wrapper() {\n    assert_eq!(\n        exercise::render_notice("<alert>", "A & B > C \\\"quoted\\\" 'text'"),\n        "<aside><strong>&lt;alert&gt;</strong><p>A &amp; B &gt; C &quot;quoted&quot; &#39;text&#39;</p></aside>"\n    );\n}\n''',
    },
    {
        "id": "capstone-easy", "title": "List every reviewable ply", "topic": "Capstone", "tier": "Easy",
        "goal": "Build a guided inclusive range for a game that includes the starting position and final ply.",
        "check": "Run `capstone_easy`. It checks a game with moves and a game with no moves.",
        "hint": "The starting position is zero. Use an inclusive range so `move_count` itself is included.",
        "reference": "capstone-and-end-to-end-flow",
        "source": '''// NOTE: A game with N plies has N + 1 reviewable positions, starting at ply zero.\n// TODO: Change the exclusive range so the final ply is included.\npub fn reviewable_plies(move_count: usize) -> Vec<usize> {\n    let final_ply = move_count;\n    (0..final_ply).collect()\n}\n''',
        "solution": '''pub fn reviewable_plies(move_count: usize) -> Vec<usize> {\n    (0..=move_count).collect()\n}\n''',
        "test": '''#[path = "../src/capstone_easy.rs"]\nmod exercise;\n\n#[test]\nfn review_range_includes_start_and_final_position() {\n    assert_eq!(exercise::reviewable_plies(3), vec![0, 1, 2, 3]);\n    assert_eq!(exercise::reviewable_plies(0), vec![0]);\n}\n''',
    },
    {
        "id": "capstone-hard", "title": "Finish a guessing round", "topic": "Capstone", "tier": "Hard",
        "goal": "Build a small round model and implement the entire state transition that clamps navigation and reveals a normalized correct answer.",
        "check": "Run `capstone_hard`. The integration test checks the completed round and a wrong answer that must stay hidden.",
        "hint": "Clamp to `moves.len()`. Normalize both opening names by splitting whitespace, joining with one space, and lowercasing.",
        "reference": "capstone-and-end-to-end-flow",
        "source": '''// NOTE: This final exercise combines move state, normalized answers, and reveal state.\n// TODO: Define the round record and implement the complete state transition below.\npub struct Round {\n    pub moves: Vec<String>,\n    pub ply: usize,\n    pub opening: String,\n    pub revealed: bool,\n}\n\n// TODO: Clamp the ply, compare normalized answers, reveal only on a correct nonempty guess.\npub fn finish_round(round: &mut Round, requested_ply: usize, guess: &str) -> bool {\n    todo!("finish a GTE round from its game data and learner answer")\n}\n''',
        "solution": '''pub struct Round {\n    pub moves: Vec<String>,\n    pub ply: usize,\n    pub opening: String,\n    pub revealed: bool,\n}\n\npub fn finish_round(round: &mut Round, requested_ply: usize, guess: &str) -> bool {\n    round.ply = requested_ply.min(round.moves.len());\n    let normalize = |value: &str| {\n        value.split_whitespace().collect::<Vec<_>>().join(" ").to_ascii_lowercase()\n    };\n    let guess = normalize(guess);\n    let correct = !guess.is_empty() && guess == normalize(&round.opening);\n    if correct {\n        round.revealed = true;\n    }\n    correct\n}\n''',
        "test": '''#[path = "../src/capstone_hard.rs"]\nmod exercise;\n\nfn round() -> exercise::Round {\n    exercise::Round {\n        moves: vec!["e4".into(), "e5".into(), "Nf3".into()],\n        ply: 0,\n        opening: "Italian Game".into(),\n        revealed: false,\n    }\n}\n\n#[test]\nfn finish_round_clamps_navigation_and_reveals_only_a_correct_answer() {\n    let mut correct = round();\n    assert!(exercise::finish_round(&mut correct, 99, " ITALIAN   GAME "));\n    assert_eq!(correct.ply, 3);\n    assert!(correct.revealed);\n\n    let mut incorrect = round();\n    assert!(!exercise::finish_round(&mut incorrect, 1, "Sicilian Defense"));\n    assert_eq!(incorrect.ply, 1);\n    assert!(!incorrect.revealed);\n}\n''',
    },
]


def write_file(path: Path, content: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(textwrap.dedent(content).lstrip(), encoding="utf-8")


def main() -> None:
    for exercise in EXERCISES:
        stem = exercise["id"].replace("-", "_")
        markdown = f'''# {exercise['title']}\n\n## Goal\n\n{exercise['goal']}\n\n## Check\n\n{exercise['check']}\n\n## Hint\n\n{exercise['hint']}\n\n## NOTE\n\nThis is the {exercise['tier'].lower()} version of the {exercise['topic'].lower()} example. Read the NOTE in the Rust source to understand the expected behavior before changing it.\n\n## TODO\n\nFollow the TODO comment in `exercises/src/{stem}.rs`. The exercise tests are independent and can be rerun with the terminal's `R` command.\n\n## Reference\n\nRead [reference.md: {exercise['reference']}](reference.md#{exercise['reference']}) in the codebase walkthrough.\n'''
        write_file(ROOT / "exercises/src" / f"{stem}.rs", exercise["source"])
        write_file(ROOT / "exercises/tests" / f"{stem}.rs", exercise["test"])
        write_file(ROOT / "solutions" / f"{stem}.rs", exercise["solution"])
        write_file(ROOT / "exercises" / f"{stem}.md", markdown)
    print(f"Generated {len(EXERCISES)} tiered exercises and their tests, solutions, and guides")


if __name__ == "__main__":
    main()
