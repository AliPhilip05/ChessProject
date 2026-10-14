use crate::api;
use crate::chess::{ChessGame, opening_matches, rating_is_close};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, OnceLock};

#[derive(Clone, Copy, PartialEq, Eq)]
enum GameMode {
    Elo,
    Opening,
}

struct Session {
    game: ChessGame,
    mode: GameMode,
    difficulty: String,
    target: String,
    ply: usize,
    note: String,
    feedback: String,
    revealed: bool,
}

static SESSIONS: OnceLock<Mutex<HashMap<String, Session>>> = OnceLock::new();
static NEXT_SESSION: AtomicU64 = AtomicU64::new(1);

fn sessions() -> &'static Mutex<HashMap<String, Session>> {
    SESSIONS.get_or_init(|| Mutex::new(HashMap::new()))
}

pub fn serve() {
    let host = std::env::var("BIND_ADDRESS").unwrap_or_else(|_| "127.0.0.1".to_owned());
    let port = std::env::var("PORT").unwrap_or_else(|_| "7878".to_owned());
    let address = format!("{host}:{port}");
    let listener = match TcpListener::bind(&address) {
        Ok(listener) => listener,
        Err(error) => {
            eprintln!("Could not start the ChessProject web app: {error}");
            return;
        }
    };
    println!("ChessProject is listening at http://{address}");
    println!("Guidebook: http://{address}/guide");

    for incoming in listener.incoming() {
        match incoming {
            Ok(stream) => handle_connection(stream),
            Err(error) => eprintln!("Browser connection failed: {error}"),
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let _ = stream.set_read_timeout(Some(std::time::Duration::from_secs(3)));
    let mut buffer = [0u8; 16_384];
    let count = match stream.read(&mut buffer) {
        Ok(count) if count > 0 => count,
        _ => return,
    };
    let request = String::from_utf8_lossy(&buffer[..count]);
    let first_line = request.lines().next().unwrap_or_default();
    let mut parts = first_line.split_whitespace();
    let method = parts.next().unwrap_or("GET");
    let target = parts.next().unwrap_or("/");
    if method != "GET" {
        write_response(
            &mut stream,
            405,
            "text/plain; charset=utf-8",
            "Use a browser GET form for this prototype.",
        );
        return;
    }
    let response = route(target);
    match response {
        HttpResponse::Page(body) => {
            write_response(&mut stream, 200, "text/html; charset=utf-8", &body)
        }
        HttpResponse::Download(body) => write_download(&mut stream, "GTE-Guidebook.md", &body),
        HttpResponse::Pdf(body) => write_pdf(&mut stream, &body),
        HttpResponse::Redirect(location) => {
            let head = format!(
                "HTTP/1.1 303 See Other\r\nLocation: {location}\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
            );
            let _ = stream.write_all(head.as_bytes());
        }
        HttpResponse::NotFound => write_response(
            &mut stream,
            404,
            "text/plain; charset=utf-8",
            "Page not found.",
        ),
    }
}

enum HttpResponse {
    Page(String),
    Download(String),
    Pdf(Vec<u8>),
    Redirect(String),
    NotFound,
}

fn route(target: &str) -> HttpResponse {
    let (path, query) = target.split_once('?').unwrap_or((target, ""));
    let params = parse_query(query);
    match path {
        "/" => HttpResponse::Page(render_home("")),
        "/guide" => HttpResponse::Page(render_book("introduction")),
        "/guide/download" => HttpResponse::Download(download_book_markdown()),
        "/guide/functions.pdf" => {
            HttpResponse::Pdf(include_bytes!("../output/pdf/GTE-Function-Appendix.pdf").to_vec())
        }
        "/load" => load_game(&params),
        "/play" => render_session(&params),
        "/move" => update_move(&params),
        "/guess-elo" => submit_elo_guess(&params),
        "/guess-opening" => submit_opening_guess(&params),
        "/new" => HttpResponse::Page(render_home("Choose a mode to start another round.")),
        _ => path
            .strip_prefix("/guide/")
            .and_then(book_chapter_index)
            .map(|index| HttpResponse::Page(render_book(&book_chapters()[index].slug)))
            .unwrap_or(HttpResponse::NotFound),
    }
}

fn load_game(params: &HashMap<String, String>) -> HttpResponse {
    let mode = if params.get("mode").map(String::as_str) == Some("opening") {
        GameMode::Opening
    } else {
        GameMode::Elo
    };
    let difficulty = params
        .get("difficulty")
        .map(|value| normalize_difficulty(value))
        .unwrap_or_else(|| "medium".to_owned());
    let target = params
        .get("target")
        .map(|value| normalize_target(value))
        .unwrap_or_else(|| "both".to_owned());
    let username = params.get("username").map(String::as_str).unwrap_or("erik");
    let source = params.get("source").map(String::as_str).unwrap_or("demo");

    let (game, note) = match source {
        "random" => match api::random_public_game(username) {
            Ok(game) => (game, String::new()),
            Err(error) => (
                ChessGame::demo(),
                format!("API fallback: {error} The included sample game is shown instead."),
            ),
        },
        "link" => {
            let url = params.get("url").map(String::as_str).unwrap_or_default();
            match api::game_from_archive_link(username, url) {
                Ok(game) => (game, String::new()),
                Err(error) => (
                    ChessGame::demo(),
                    format!(
                        "Game link unavailable: {error} Demo data is shown so you can still explore the board."
                    ),
                ),
            }
        }
        _ => (
            ChessGame::demo(),
            "Demo data · this built-in game works without a network connection.".to_owned(),
        ),
    };
    if game.ply_count() == 0 {
        let demo = ChessGame::demo();
        let note = if note.is_empty() {
            "The loaded PGN had no readable moves, so the demo game is shown.".to_owned()
        } else {
            note
        };
        return create_session(demo, mode, difficulty, target, note);
    }
    create_session(game, mode, difficulty, target, note)
}

fn create_session(
    game: ChessGame,
    mode: GameMode,
    difficulty: String,
    target: String,
    note: String,
) -> HttpResponse {
    let id = NEXT_SESSION.fetch_add(1, Ordering::Relaxed).to_string();
    let ply = if mode == GameMode::Opening {
        8.min(game.ply_count())
    } else {
        0
    };
    let session = Session {
        game,
        mode,
        difficulty,
        target,
        ply,
        note,
        feedback: String::new(),
        revealed: false,
    };
    if let Ok(mut store) = sessions().lock() {
        store.insert(id.clone(), session);
    }
    HttpResponse::Redirect(format!("/play?sid={id}"))
}

fn render_session(params: &HashMap<String, String>) -> HttpResponse {
    let Some(id) = params.get("sid") else {
        return HttpResponse::Page(render_home(
            "This game session has expired. Start a new round.",
        ));
    };
    match sessions().lock() {
        Ok(store) => match store.get(id) {
            Some(session) => HttpResponse::Page(render_game(session, id)),
            None => HttpResponse::Page(render_home(
                "This game session has expired. Start a new round.",
            )),
        },
        Err(_) => HttpResponse::Page(render_home(
            "The local game session could not be read. Start a new round.",
        )),
    }
}

fn update_move(params: &HashMap<String, String>) -> HttpResponse {
    let Some(id) = params.get("sid") else {
        return HttpResponse::Page(render_home(
            "This game session has expired. Start a new round.",
        ));
    };
    if let Ok(mut store) = sessions().lock() {
        if let Some(session) = store.get_mut(id) {
            let requested = params
                .get("ply")
                .and_then(|value| value.parse::<usize>().ok())
                .unwrap_or(session.ply);
            session.ply = requested.min(max_ply(session));
            session.feedback.clear();
        }
    }
    HttpResponse::Redirect(format!("/play?sid={}", html_escape(id)))
}

fn submit_elo_guess(params: &HashMap<String, String>) -> HttpResponse {
    let Some(id) = params.get("sid") else {
        return HttpResponse::Page(render_home(
            "This game session has expired. Start a new round.",
        ));
    };
    let mut store = match sessions().lock() {
        Ok(store) => store,
        Err(_) => {
            return HttpResponse::Page(render_home("The local game session could not be read."));
        }
    };
    let Some(session) = store.get_mut(id) else {
        return HttpResponse::Page(render_home(
            "This game session has expired. Start a new round.",
        ));
    };
    if session.mode != GameMode::Elo {
        return HttpResponse::Redirect(format!("/play?sid={}", html_escape(id)));
    }

    let mut lines = Vec::new();
    if session.target != "black" {
        score_one(
            &mut lines,
            "White",
            params.get("white").map(String::as_str).unwrap_or_default(),
            session.game.white_rating,
            &session.difficulty,
        );
    }
    if session.target != "white" {
        score_one(
            &mut lines,
            "Black",
            params.get("black").map(String::as_str).unwrap_or_default(),
            session.game.black_rating,
            &session.difficulty,
        );
    }
    session.feedback = lines.join("<br>");
    session.revealed = true;
    HttpResponse::Redirect(format!("/play?sid={}", html_escape(id)))
}

fn score_one(
    lines: &mut Vec<String>,
    color: &str,
    raw_guess: &str,
    actual: Option<i32>,
    difficulty: &str,
) {
    let Some(guess) = raw_guess
        .trim()
        .parse::<i32>()
        .ok()
        .filter(|value| (0..=4000).contains(value))
    else {
        lines.push(format!("{color}: enter a numeric rating guess."));
        return;
    };
    let Some(actual) = actual else {
        lines.push(format!(
            "{color}: no rating was included with this game record."
        ));
        return;
    };
    let error = (guess - actual).abs();
    let outcome = if rating_is_close(guess, actual, difficulty) {
        "inside tolerance"
    } else {
        "outside tolerance"
    };
    lines.push(format!(
        "{color}: you guessed {guess}; actual rating {actual}; off by {error} · {outcome}."
    ));
}

fn submit_opening_guess(params: &HashMap<String, String>) -> HttpResponse {
    let Some(id) = params.get("sid") else {
        return HttpResponse::Page(render_home(
            "This game session has expired. Start a new round.",
        ));
    };
    let mut store = match sessions().lock() {
        Ok(store) => store,
        Err(_) => {
            return HttpResponse::Page(render_home("The local game session could not be read."));
        }
    };
    let Some(session) = store.get_mut(id) else {
        return HttpResponse::Page(render_home(
            "This game session has expired. Start a new round.",
        ));
    };
    if session.mode != GameMode::Opening {
        return HttpResponse::Redirect(format!("/play?sid={}", html_escape(id)));
    }
    let guess = params.get("guess").map(String::as_str).unwrap_or_default();
    session.feedback = if opening_matches(guess, &session.game.opening) {
        format!(
            "Correct — the opening is {}.",
            html_escape(&session.game.opening)
        )
    } else if guess.trim().is_empty() {
        "Enter an opening name before submitting.".to_owned()
    } else {
        format!(
            "Not quite. The opening is {}.",
            html_escape(&session.game.opening)
        )
    };
    session.revealed = true;
    HttpResponse::Redirect(format!("/play?sid={}", html_escape(id)))
}

fn render_home(message: &str) -> String {
    let body = format!(
        r###"
<header class="topbar"><a class="brand" href="/"><span class="brand-mark">GTE</span><span>CHESS<span class="green">PROJECT</span></span></a><nav><a href="#elo">Guess the Elo</a><a href="#opening">Guess the Opening</a><a href="/guide">Guidebook</a></nav><a class="nav-cta" href="#elo">Start a round <span>↗</span></a></header>
<main>
  <section class="hero" id="overview">
    <div class="hero-copy"><div class="eyebrow"><span class="pulse"></span> PLAY THE POSITION. READ THE PLAYER.</div><h1>Can you spot<br>the <em>rating?</em></h1><p class="lede">A game can tell you a lot. Move by move, put your chess instincts to the test — then see how close you came.</p><div class="hero-actions"><a class="button button-green" href="#elo">Choose a challenge <span>→</span></a><a class="text-link" href="https://www.youtube.com/results?search_query=GothamChess+Guess+the+Elo" target="_blank" rel="noreferrer">Watch GothamChess's Guess the Elo <span>↗</span></a></div></div>
    <div class="hero-board-wrap"><div class="hero-board" aria-hidden="true">{}</div><div class="board-caption"><span>OPENING STUDY</span><span>01 / 02</span></div></div>
    <div class="hero-stamp"><span>02</span><small>WAYS TO<br>PLAY</small></div>
  </section>
  <div class="inspiration"><span class="inspiration-icon">▶</span><p><strong>Inspired by GothamChess's Guess the Elo series.</strong> Watch a game, read the moves, and see if your rating instincts hold up.</p><a href="https://www.youtube.com/results?search_query=GothamChess+Guess+the+Elo" target="_blank" rel="noreferrer">Explore the series ↗</a></div>
  <section class="intro-row"><div><div class="eyebrow">THE PROJECT</div><h2>Learn to read<br>the game.</h2></div><p>ChessProject is a chess practice and programming-learning prototype. Review public Chess.com games with the ratings hidden, guess each player's Elo or name the opening, then explore how the Rust app works in its companion Guidebook and terminal lessons.</p><a class="button button-outline" href="/guide">Read the Guidebook <span>→</span></a></section>
  <section class="mode-section" id="elo"><div class="section-head"><div><div class="eyebrow">MODE 01 · PLAYER SENSE</div><h2>Guess the Elo</h2></div><p>Watch the moves. Trust your read.<br>Reveal the ratings when you're ready.</p></div>
    <div class="mode-card elo-card"><div class="card-number">01</div><div class="mode-copy"><span class="mode-icon">♟</span><h3>How strong<br>are they?</h3><p>Choose a game, review the board at your pace, and call the rating. You decide how close counts.</p><div class="tag-row"><span>MOVE REVIEW</span><span>3 DIFFICULTIES</span><span>WHITE / BLACK / BOTH</span></div></div><div class="mode-form"><h4>Set up your round</h4><form action="/load" method="get"><input type="hidden" name="mode" value="elo"><input type="hidden" name="source" value="random"><label>Chess.com username <span>public archive</span><input name="username" value="erik" required maxlength="25" autocomplete="off"></label><div class="form-row"><label>Difficulty<select name="difficulty"><option value="easy">Easy · ±200</option><option value="medium" selected>Medium · ±100</option><option value="hard">Hard · ±50</option></select></label><label>Guess for<select name="target"><option value="both">Both players</option><option value="white">White only</option><option value="black">Black only</option></select></label></div><button class="button button-green full" type="submit">Pick a random archive game <span>→</span></button></form><div class="form-divider"><span>OR USE A GAME LINK</span></div><form action="/load" method="get"><input type="hidden" name="mode" value="elo"><input type="hidden" name="source" value="link"><label>Chess.com game URL<input name="url" placeholder="https://www.chess.com/game/live/…" required></label><label>Player archive username<input name="username" value="erik" required maxlength="25" autocomplete="off"></label><div class="form-row"><label>Difficulty<select name="difficulty"><option value="easy">Easy · ±200</option><option value="medium" selected>Medium · ±100</option><option value="hard">Hard · ±50</option></select></label><label>Guess for<select name="target"><option value="both">Both players</option><option value="white">White only</option><option value="black">Black only</option></select></label></div><button class="button button-outline full" type="submit">Find this game <span>→</span></button></form><a class="demo-link" href="/load?mode=elo&source=demo">Try the built-in demo game <span>↗</span></a></div></div>
    <p class="api-note"><span>i</span> Random picks come from the selected player's public Chess.com archive. The PubAPI does not provide a global random-game feed.</p>
  </section>
  <section class="mode-section opening-section" id="opening"><div class="section-head"><div><div class="eyebrow">MODE 02 · OPENING RECALL</div><h2>Guess the Opening</h2></div><p>See the first few moves.<br>Can you name the opening?</p></div><div class="mode-card opening-card"><div class="opening-preview"><div class="preview-label"><span class="pulse"></span> YOUR NEXT CHALLENGE</div><div class="mini-board">{}</div><div class="preview-moves"><b>1.</b> e4 e5 <b>2.</b> Nf3 Nc6 <b>3.</b> Bc4 Bc5</div></div><div class="opening-form"><span class="mode-icon">⌁</span><h3>Find the name<br>behind the moves.</h3><p>A short opening sequence, one answer box, and immediate feedback. Uppercase or lowercase both work.</p><form action="/load" method="get"><input type="hidden" name="mode" value="opening"><input type="hidden" name="source" value="random"><label>Chess.com username <span>public archive</span><input name="username" value="erik" required maxlength="25" autocomplete="off"></label><button class="button button-green full" type="submit">Start opening challenge <span>→</span></button></form><a class="demo-link" href="/load?mode=opening&source=demo">Try the Italian Game demo <span>↗</span></a></div></div></section>
  <section class="build-note"><div class="build-icon">⌘</div><div><div class="eyebrow">BUILT TO BE LEARNED</div><h2>Curious how it works?</h2><p>Open the Rustlings-style terminal path to explore the API, PGN parser, scoring rules, and page renderer one focused lesson at a time.</p></div><div class="build-actions"><code>cargo run</code><a class="button button-outline" href="/guide">Open the Guidebook <span>→</span></a></div></section>
  <footer><a class="brand" href="/"><span class="brand-mark">GTE</span><span>CHESS<span class="green">PROJECT</span></span></a><span>Rust-built prototype · Chess.com public game data</span><a href="/guide">Guidebook ↗</a></footer>
</main>"###,
        mini_board(ChessGame::demo().position_at(8)),
        mini_board(ChessGame::demo().position_at(8))
    );
    let notice = if message.is_empty() {
        String::new()
    } else {
        format!("<div class=\"toast\">{}</div>", html_escape(message))
    };
    page("GTE · Read the Game", &notice, &body)
}

fn render_game(session: &Session, id: &str) -> String {
    let game = &session.game;
    let mode_label = if session.mode == GameMode::Elo {
        "GUESS THE ELO"
    } else {
        "GUESS THE OPENING"
    };
    let limit = max_ply(session);
    let move_list = render_move_list(game, id, session.ply, limit);
    let board = render_board(game.position_at(session.ply));
    let note = if session.note.is_empty() {
        String::new()
    } else {
        format!("<div class=\"toast\">{}</div>", html_escape(&session.note))
    };
    let game_link = if game.source_url.starts_with("https://") {
        format!(
            "<a class=\"source-link\" href=\"{}\" target=\"_blank\" rel=\"noreferrer\">Open on Chess.com ↗</a>",
            html_escape(&game.source_url)
        )
    } else {
        String::new()
    };
    let feedback = if session.feedback.is_empty() {
        String::new()
    } else {
        format!("<div class=\"result-box\">{}</div>", session.feedback)
    };
    let sidebar = if session.mode == GameMode::Elo {
        render_elo_panel(session, id, &feedback)
    } else {
        render_opening_panel(session, id, &feedback)
    };
    let title = if session.mode == GameMode::Elo {
        "Guess the Elo"
    } else {
        "Guess the Opening"
    };
    let body = format!(
        r###"
<header class="topbar"><a class="brand" href="/"><span class="brand-mark">GTE</span><span>CHESS<span class="green">PROJECT</span></span></a><nav><a class="active" href="/">Game lab</a><a href="/guide">Guidebook</a></nav><a class="nav-cta" href="/new">New round <span>↗</span></a></header>
<main class="play-main"><div class="play-heading"><div><a class="back-link" href="/">← All challenges</a><div class="eyebrow">{mode_label} · {}</div><h1>{}</h1></div><div class="round-badge"><span class="pulse"></span> YOUR ROUND</div></div>
<div class="play-grid"><section class="board-panel">{}<div class="board-area">{}</div>{}<div class="move-controls"><a href="/move?sid={id}&ply=0" aria-label="First position">|◀</a><a href="/move?sid={id}&ply={}" aria-label="Previous move">←</a><span>MOVE {} / {}</span><a href="/move?sid={id}&ply={}" aria-label="Next move">→</a><a href="/move?sid={id}&ply={}" aria-label="Last position">▶|</a></div><div class="move-list"><div class="move-list-head"><span>GAME MOVES</span><span>{} plies</span></div>{}</div></section><aside class="guess-panel">{}</aside></div>
<div class="game-meta"><span><small>RESULT</small><b>{}</b></span><span><small>TIME CONTROL</small><b>{}</b></span><span><small>DATE</small><b>{}</b></span><span><small>ECO</small><b>{}</b></span><span>{}</span></div>
<footer><a class="brand" href="/"><span class="brand-mark">GTE</span><span>CHESS<span class="green">PROJECT</span></span></a><span>Public archive game · rating revealed only after your guess</span><a href="/guide">How it works ↗</a></footer></main>"###,
        if game.demo {
            "DEMO DATA"
        } else {
            "CHESS.COM ARCHIVE"
        },
        html_escape(title),
        render_player(
            "black",
            &game.black_name,
            game.black_rating,
            session.revealed && session.target != "white"
        ),
        board,
        render_player(
            "white",
            &game.white_name,
            game.white_rating,
            session.revealed && session.target != "black"
        ),
        session.ply.saturating_sub(1),
        session.ply,
        limit,
        session.ply.saturating_add(1).min(limit),
        limit,
        limit,
        move_list,
        sidebar,
        html_escape(&game.result),
        html_escape(&game.time_control),
        html_escape(&game.date),
        if game.eco.is_empty() {
            "—".to_owned()
        } else {
            html_escape(&game.eco)
        },
        game_link
    );
    page(title, &note, &body)
}

fn render_elo_panel(session: &Session, id: &str, feedback: &str) -> String {
    let white_input = if session.target == "black" {
        String::new()
    } else {
        format!(
            "<label>White · {}<input type=number min=0 max=4000 name=white placeholder=\"Your guess\" required></label>",
            html_escape(&session.game.white_name)
        )
    };
    let black_input = if session.target == "white" {
        String::new()
    } else {
        format!(
            "<label>Black · {}<input type=number min=0 max=4000 name=black placeholder=\"Your guess\" required></label>",
            html_escape(&session.game.black_name)
        )
    };
    let answer_area = if session.revealed {
        feedback.to_owned()
    } else {
        String::new()
    };
    format!(
        r###"<div class="panel-kicker"><span class="mode-icon small-icon">♟</span><span>MAKE YOUR CALL</span></div><h2>What are their<br>ratings?</h2><p class="panel-description">Pick a number for each player you want to guess. Your difficulty window is <strong>±{}</strong>.</p><div class="difficulty-pill">{} DIFFICULTY <span>±{}</span></div><form class="guess-form" action="/guess-elo" method="get"><input type="hidden" name="sid" value="{}">{}{}<button class="button button-green full" type="submit">Reveal the ratings <span>→</span></button></form>{}<div class="mode-selector-label">YOUR SETTINGS</div><div class="settings-chips"><span>{}</span><span>{}</span></div><a class="demo-link" href="/new">Try another game <span>↗</span></a>"###,
        crate::chess::rating_tolerance(&session.difficulty),
        session.difficulty.to_ascii_uppercase(),
        crate::chess::rating_tolerance(&session.difficulty),
        id,
        white_input,
        black_input,
        answer_area,
        session.target.to_ascii_uppercase(),
        session.difficulty.to_ascii_uppercase()
    )
}

fn render_opening_panel(session: &Session, id: &str, feedback: &str) -> String {
    let feedback = if session.revealed {
        feedback.to_owned()
    } else {
        String::new()
    };
    format!(
        r###"<div class="panel-kicker"><span class="mode-icon small-icon">⌁</span><span>NAME THAT OPENING</span></div><h2>What opening<br>is this?</h2><p class="panel-description">The board is paused after the first few moves. Enter the opening name you recognize.</p><div class="sequence-card"><span>POSITION AFTER</span><strong>{} plies</strong><div>{}</div></div><form class="guess-form" action="/guess-opening" method="get"><input type="hidden" name="sid" value="{}"><label>Your opening guess<input name="guess" placeholder="e.g. Italian Game" autocomplete="off" required></label><button class="button button-green full" type="submit">Check my answer <span>→</span></button></form>{}<div class="opening-tip">Capitalization and extra spaces don't matter.</div><a class="demo-link" href="/new">Try another game <span>↗</span></a>"###,
        session.ply,
        html_escape(&opening_sequence(&session.game)),
        id,
        feedback
    )
}

fn render_player(color: &str, name: &str, rating: Option<i32>, revealed: bool) -> String {
    let (icon, class, label) = if color == "white" {
        ("♙", "white-dot", "White")
    } else {
        ("♟", "black-dot", "Black")
    };
    let rating_text = if revealed {
        rating
            .map(|value| format!("Rating · {value}"))
            .unwrap_or_else(|| "Rating unavailable".to_owned())
    } else {
        "Rating hidden".to_owned()
    };
    let visible_rating = if revealed {
        String::new()
    } else {
        "•••".to_owned()
    };
    format!(
        "<div class=\"player-line\"><span class=\"piece-dot {class}\">{icon}</span><div><strong>{}</strong><small>{label} · {}</small></div><span class=\"rating-lock\">{visible_rating}</span></div>",
        html_escape(name),
        html_escape(&rating_text)
    )
}

fn opening_sequence(game: &ChessGame) -> String {
    let mut sequence = String::new();
    for (index, san) in game.moves.iter().take(8).enumerate() {
        if index % 2 == 0 {
            sequence.push_str(&format!("{}.", index / 2 + 1));
        }
        sequence.push(' ');
        sequence.push_str(san);
    }
    sequence.trim().to_owned()
}

fn max_ply(session: &Session) -> usize {
    let game_count = session.game.ply_count();
    if session.mode == GameMode::Opening {
        game_count.min(8)
    } else {
        game_count
    }
}

fn render_move_list(
    game: &ChessGame,
    id: &str,
    selected_ply: usize,
    visible_plies: usize,
) -> String {
    if game.moves.is_empty() {
        return "<p class=\"muted\">No readable moves in this PGN.</p>".to_owned();
    }
    let mut output = String::from("<div class=\"move-grid\">");
    for (index, san) in game.moves.iter().take(visible_plies).enumerate() {
        let ply = index + 1;
        let move_number = (index / 2) + 1;
        let class = if ply == selected_ply {
            "move-token selected"
        } else {
            "move-token"
        };
        if index % 2 == 0 {
            output.push_str(&format!(
                "<span class=\"move-number\">{move_number}.</span>"
            ));
        }
        output.push_str(&format!(
            "<a class=\"{class}\" href=\"/move?sid={id}&amp;ply={ply}\">{}</a>",
            html_escape(san)
        ));
    }
    output.push_str("</div>");
    output
}

fn render_board(position: &[char; 64]) -> String {
    let mut output =
        String::from("<div class=\"board\" role=\"img\" aria-label=\"Chess position\">");
    for row in 0..8 {
        for column in 0..8 {
            let piece = position[row * 8 + column];
            let square = if (row + column) % 2 == 0 {
                "light-square"
            } else {
                "dark-square"
            };
            let symbol = chess_symbol(piece);
            let label = if piece == '.' {
                "empty".to_owned()
            } else {
                format!(
                    "{} on {}{}",
                    piece_name(piece),
                    (b'a' + column as u8) as char,
                    8 - row
                )
            };
            output.push_str(&format!(
                "<div class=\"square {square}\" aria-label=\"{}\">{symbol}</div>",
                html_escape(&label)
            ));
        }
    }
    output.push_str("</div>");
    output
}

fn mini_board(position: &[char; 64]) -> String {
    let mut output = String::from("<div class=\"mini-board-grid\">");
    for row in 0..8 {
        for column in 0..8 {
            let piece = position[row * 8 + column];
            let square = if (row + column) % 2 == 0 {
                "light-square"
            } else {
                "dark-square"
            };
            output.push_str(&format!(
                "<div class=\"square {square}\">{}</div>",
                chess_symbol(piece)
            ));
        }
    }
    output.push_str("</div>");
    output
}

fn chess_symbol(piece: char) -> &'static str {
    match piece {
        'K' => "♔",
        'Q' => "♕",
        'R' => "♖",
        'B' => "♗",
        'N' => "♘",
        'P' => "♙",
        'k' => "♚",
        'q' => "♛",
        'r' => "♜",
        'b' => "♝",
        'n' => "♞",
        'p' => "♟",
        _ => "",
    }
}

fn piece_name(piece: char) -> &'static str {
    match piece.to_ascii_uppercase() {
        'K' => "king",
        'Q' => "queen",
        'R' => "rook",
        'B' => "bishop",
        'N' => "knight",
        _ => "pawn",
    }
}

struct BookChapter {
    slug: String,
    title: String,
    source: String,
}

const BASE_BOOK_CHAPTERS: &[(&str, &str, &str)] = &[
    (
        "introduction",
        "Introduction",
        include_str!("../docs/book/src/introduction.md"),
    ),
    (
        "question-zero",
        "Question 0: Start with GTE",
        include_str!("../docs/book/src/question-zero.md"),
    ),
    (
        "getting-started",
        "Getting Started",
        include_str!("../docs/book/src/getting-started.md"),
    ),
    (
        "architecture",
        "Project Architecture",
        include_str!("../docs/book/src/architecture.md"),
    ),
    (
        "rust-entry-point",
        "Rust Entry Points and Types",
        include_str!("../docs/book/src/rust-entry-point.md"),
    ),
    (
        "public-data",
        "Chess.com Public Data",
        include_str!("../docs/book/src/public-data.md"),
    ),
    (
        "board-state",
        "PGN and Board State",
        include_str!("../docs/book/src/board-state.md"),
    ),
    (
        "game-modes",
        "The Two Game Modes",
        include_str!("../docs/book/src/game-modes.md"),
    ),
    (
        "web-server",
        "The Web Server and Rendered Pages",
        include_str!("../docs/book/src/web-server.md"),
    ),
    (
        "terminal-learning",
        "Terminal Lessons",
        include_str!("../docs/book/src/terminal-learning.md"),
    ),
    (
        "capstone",
        "Capstone",
        include_str!("../docs/book/src/capstone.md"),
    ),
    (
        "reference",
        "Codebase Walkthrough — reference.md",
        include_str!("../exercises/reference.md"),
    ),
];

const FUNCTION_PAGE_SOURCE: &str = include_str!("../docs/book/src/function-pages.md");

fn book_chapters() -> &'static [BookChapter] {
    static CHAPTERS: OnceLock<Vec<BookChapter>> = OnceLock::new();
    CHAPTERS.get_or_init(|| {
        let mut chapters: Vec<BookChapter> = BASE_BOOK_CHAPTERS
            .iter()
            .map(|(slug, title, source)| BookChapter {
                slug: (*slug).to_owned(),
                title: (*title).to_owned(),
                source: (*source).to_owned(),
            })
            .collect();
        let mut page: Option<(String, String, String)> = None;
        for line in FUNCTION_PAGE_SOURCE.lines() {
            if let Some(metadata) = line
                .strip_prefix("<!-- GTE_FUNCTION_PAGE: ")
                .and_then(|line| line.strip_suffix(" -->"))
            {
                let (slug, title) = metadata.split_once('|').unwrap_or((metadata, metadata));
                page = Some((slug.to_owned(), title.to_owned(), String::new()));
            } else if line == "<!-- END_GTE_FUNCTION_PAGE -->" {
                if let Some((slug, title, source)) = page.take() {
                    chapters.push(BookChapter {
                        slug,
                        title: format!("Function: {title}"),
                        source: source.trim().to_owned(),
                    });
                }
            } else if let Some((_, _, source)) = page.as_mut() {
                source.push_str(line);
                source.push('\n');
            }
        }
        chapters
    })
}

fn book_chapter_index(slug: &str) -> Option<usize> {
    book_chapters()
        .iter()
        .position(|chapter| chapter.slug == slug)
}

fn render_book(slug: &str) -> String {
    let Some(index) = book_chapter_index(slug) else {
        return String::new();
    };
    let chapters = book_chapters();
    let chapter = &chapters[index];
    let mut sidebar = String::new();
    for (chapter_index, item) in chapters.iter().enumerate() {
        let active = if chapter_index == index {
            " class=\"active\" aria-current=\"page\""
        } else {
            ""
        };
        sidebar.push_str(&format!(
            "<a{active} href=\"/guide/{}\"><span class=\"chapter-number\">{:02}</span>{}</a>",
            item.slug,
            chapter_index + 1,
            html_escape(&item.title)
        ));
    }
    let previous = index.checked_sub(1).map(|previous_index| {
        let item = &chapters[previous_index];
        format!(
            "<a class=\"chapter-card previous\" href=\"/guide/{}\"><small>PREVIOUS</small><span>← {}</span></a>",
            item.slug,
            html_escape(&item.title)
        )
    }).unwrap_or_else(|| "<span></span>".to_owned());
    let next = chapters.get(index + 1).map(|item| {
        format!(
            "<a class=\"chapter-card next\" href=\"/guide/{}\"><small>NEXT</small><span>{} →</span></a>",
            item.slug,
            html_escape(&item.title)
        )
    }).unwrap_or_default();
    let body = format!(
        "<header class=\"book-header\"><a class=\"book-brand\" href=\"/guide\"><span class=\"book-mark\">GTE</span><span><strong>The GTE Book</strong><small>Rust app walkthrough</small></span></a><div class=\"book-header-title\">{}</div><a class=\"book-download pdf-download\" href=\"/guide/functions.pdf\" download>Download Function Appendix PDF ↓</a><a class=\"book-download\" href=\"/guide/download\" download>Download Markdown ↓</a><a class=\"book-app-link\" href=\"/\">Open the app <span>↗</span></a></header><div class=\"book-shell\"><aside class=\"book-sidebar\"><div class=\"sidebar-heading\">THE GTE BOOK</div><div class=\"sidebar-chapters\">{sidebar}</div><a class=\"sidebar-reference\" href=\"/guide/reference\">Open reference.md <span>↗</span></a></aside><main class=\"book-main\"><div class=\"book-breadcrumb\"><a href=\"/guide\">The GTE Book</a><span>/</span><span>{}</span></div><article class=\"book-content\">{}</article><nav class=\"chapter-navigation\" aria-label=\"Chapter navigation\">{}{}</nav><footer class=\"book-footer\"><span>GTE · A Rust chess app and learning project</span><a href=\"/\">Return to the app</a></footer></main></div>",
        html_escape(&chapter.title),
        html_escape(&chapter.title),
        render_markdown(&chapter.source),
        previous,
        next
    );
    format!(
        "<!doctype html><html lang=\"en\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1\"><meta name=\"theme-color\" content=\"#f7f7f5\"><title>{} · The GTE Book</title><style>{BOOK_STYLE}</style></head><body class=\"book-page\">{body}</body></html>",
        html_escape(&chapter.title)
    )
}

fn download_book_markdown() -> String {
    let mut markdown =
        String::from("# The GTE Book\n\nA Rust chess app and terminal learning project.\n");
    for chapter in book_chapters() {
        markdown.push_str("\n\n---\n\n");
        markdown.push_str(chapter.source.trim());
        markdown.push('\n');
    }
    markdown
}

fn render_markdown(source: &str) -> String {
    let mut output = String::new();
    let mut paragraph = Vec::new();
    let mut list_kind: Option<bool> = None;
    let mut code_language: Option<String> = None;

    for line in source.lines() {
        if code_language.is_some() {
            if line.starts_with("```") {
                output.push_str("</code></pre>");
                code_language = None;
            } else {
                output.push_str(&html_escape(line));
                output.push('\n');
            }
            continue;
        }
        if line.starts_with("```") {
            flush_markdown_paragraph(&mut paragraph, &mut output);
            close_markdown_list(&mut list_kind, &mut output);
            let language = line.trim_start_matches('`').trim();
            let language_class = if language.is_empty() {
                String::new()
            } else {
                format!(" class=\"language-{}\"", html_escape(language))
            };
            output.push_str(&format!("<pre><code{language_class}>"));
            code_language = Some(language.to_owned());
            continue;
        }
        if line.trim().is_empty() {
            flush_markdown_paragraph(&mut paragraph, &mut output);
            close_markdown_list(&mut list_kind, &mut output);
            continue;
        }
        if let Some((level, heading)) = markdown_heading(line) {
            flush_markdown_paragraph(&mut paragraph, &mut output);
            close_markdown_list(&mut list_kind, &mut output);
            let id = heading_slug(heading);
            output.push_str(&format!(
                "<h{level} id=\"{id}\">{}</h{level}>",
                inline_markdown(heading)
            ));
            continue;
        }
        if line.trim() == "---" || line.trim() == "***" {
            flush_markdown_paragraph(&mut paragraph, &mut output);
            close_markdown_list(&mut list_kind, &mut output);
            output.push_str("<hr>");
            continue;
        }
        if let Some(quote) = line.strip_prefix("> ") {
            flush_markdown_paragraph(&mut paragraph, &mut output);
            close_markdown_list(&mut list_kind, &mut output);
            output.push_str(&format!(
                "<blockquote><p>{}</p></blockquote>",
                inline_markdown(quote)
            ));
            continue;
        }
        if let Some(item) = line.strip_prefix("- ").or_else(|| line.strip_prefix("* ")) {
            flush_markdown_paragraph(&mut paragraph, &mut output);
            open_markdown_list(false, &mut list_kind, &mut output);
            output.push_str(&format!("<li>{}</li>", inline_markdown(item)));
            continue;
        }
        if let Some((_, item)) = line.split_once(". ").filter(|(number, _)| {
            !number.is_empty() && number.chars().all(|character| character.is_ascii_digit())
        }) {
            flush_markdown_paragraph(&mut paragraph, &mut output);
            open_markdown_list(true, &mut list_kind, &mut output);
            output.push_str(&format!("<li>{}</li>", inline_markdown(item)));
            continue;
        }
        close_markdown_list(&mut list_kind, &mut output);
        paragraph.push(line.trim().to_owned());
    }
    flush_markdown_paragraph(&mut paragraph, &mut output);
    close_markdown_list(&mut list_kind, &mut output);
    if code_language.is_some() {
        output.push_str("</code></pre>");
    }
    output
}

fn flush_markdown_paragraph(paragraph: &mut Vec<String>, output: &mut String) {
    if paragraph.is_empty() {
        return;
    }
    let text = paragraph.join(" ");
    output.push_str(&format!("<p>{}</p>", inline_markdown(&text)));
    paragraph.clear();
}

fn open_markdown_list(ordered: bool, current: &mut Option<bool>, output: &mut String) {
    if *current == Some(ordered) {
        return;
    }
    close_markdown_list(current, output);
    output.push_str(if ordered { "<ol>" } else { "<ul>" });
    *current = Some(ordered);
}

fn close_markdown_list(current: &mut Option<bool>, output: &mut String) {
    match current.take() {
        Some(true) => output.push_str("</ol>"),
        Some(false) => output.push_str("</ul>"),
        None => {}
    }
}

fn markdown_heading(line: &str) -> Option<(usize, &str)> {
    let hashes = line
        .chars()
        .take_while(|character| *character == '#')
        .count();
    (1..=6)
        .contains(&hashes)
        .then(|| line.get(hashes..).unwrap_or_default())
        .and_then(|heading| heading.strip_prefix(' ').map(|text| (hashes, text)))
}

fn heading_slug(heading: &str) -> String {
    let mut slug = String::new();
    let mut separator = false;
    for character in heading
        .chars()
        .flat_map(|character| character.to_lowercase())
    {
        if character.is_ascii_alphanumeric() {
            if separator && !slug.is_empty() {
                slug.push('-');
            }
            slug.push(character);
            separator = false;
        } else if character.is_whitespace() || character == '-' {
            separator = true;
        }
    }
    slug
}

fn inline_markdown(text: &str) -> String {
    let mut output = String::new();
    let mut index = 0;
    while index < text.len() {
        let tail = &text[index..];
        if let Some(code) = tail.strip_prefix('`') {
            if let Some(end) = code.find('`') {
                output.push_str(&format!("<code>{}</code>", html_escape(&code[..end])));
                index += end + 2;
                continue;
            }
        }
        if let Some(strong) = tail.strip_prefix("**") {
            if let Some(end) = strong.find("**") {
                output.push_str(&format!(
                    "<strong>{}</strong>",
                    inline_markdown(&strong[..end])
                ));
                index += end + 4;
                continue;
            }
        }
        if let Some(link) = tail.strip_prefix('[') {
            if let Some(label_end) = link.find("](") {
                let target_start = label_end + 2;
                if let Some(target_end) = link[target_start..].find(')') {
                    let label = &link[..label_end];
                    let target = &link[target_start..target_start + target_end];
                    if let Some(href) = safe_markdown_href(target) {
                        output.push_str(&format!(
                            "<a href=\"{}\">{}</a>",
                            html_escape(&href),
                            inline_markdown(label)
                        ));
                    } else {
                        output.push_str(&inline_markdown(label));
                    }
                    index += target_start + target_end + 2;
                    continue;
                }
            }
        }
        let character = tail.chars().next().expect("index is before the end");
        output.push_str(&html_escape(&character.to_string()));
        index += character.len_utf8();
    }
    output
}

fn safe_markdown_href(target: &str) -> Option<String> {
    if target.starts_with("https://")
        || target.starts_with("http://")
        || target.starts_with("/guide")
        || target.starts_with('#')
    {
        Some(target.to_owned())
    } else if let Some(fragment) = target.strip_prefix("reference.md") {
        Some(format!("/guide/reference{fragment}"))
    } else {
        None
    }
}

fn page(title: &str, notice: &str, body: &str) -> String {
    format!(
        r###"<!doctype html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1"><meta name="theme-color" content="#171815"><title>{}</title><style>{}</style></head><body>{}{}</body></html>"###,
        html_escape(title),
        STYLE,
        notice,
        body
    )
}

fn write_response(stream: &mut TcpStream, status: u16, content_type: &str, body: &str) {
    let reason = match status {
        200 => "OK",
        404 => "Not Found",
        405 => "Method Not Allowed",
        _ => "Error",
    };
    let head = format!(
        "HTTP/1.1 {status} {reason}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\nX-Content-Type-Options: nosniff\r\n\r\n",
        body.len()
    );
    let _ = stream.write_all(head.as_bytes());
    let _ = stream.write_all(body.as_bytes());
}

fn write_download(stream: &mut TcpStream, filename: &str, body: &str) {
    let head = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/markdown; charset=utf-8\r\nContent-Disposition: attachment; filename=\"{filename}\"\r\nContent-Length: {}\r\nConnection: close\r\nX-Content-Type-Options: nosniff\r\n\r\n",
        body.len()
    );
    let _ = stream.write_all(head.as_bytes());
    let _ = stream.write_all(body.as_bytes());
}

fn write_pdf(stream: &mut TcpStream, body: &[u8]) {
    let head = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/pdf\r\nContent-Disposition: attachment; filename=\"GTE-Function-Appendix.pdf\"\r\nContent-Length: {}\r\nConnection: close\r\nX-Content-Type-Options: nosniff\r\n\r\n",
        body.len()
    );
    let _ = stream.write_all(head.as_bytes());
    let _ = stream.write_all(body);
}

fn parse_query(query: &str) -> HashMap<String, String> {
    query
        .split('&')
        .filter(|part| !part.is_empty())
        .filter_map(|part| {
            let (key, value) = part.split_once('=').unwrap_or((part, ""));
            Some((url_decode(key), url_decode(value)))
        })
        .collect()
}

fn url_decode(value: &str) -> String {
    let bytes = value.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        match bytes[index] {
            b'+' => {
                decoded.push(b' ');
                index += 1;
            }
            b'%' if index + 2 < bytes.len() => {
                let high = hex_value(bytes[index + 1]);
                let low = hex_value(bytes[index + 2]);
                if let (Some(high), Some(low)) = (high, low) {
                    decoded.push(high * 16 + low);
                    index += 3;
                } else {
                    decoded.push(bytes[index]);
                    index += 1;
                }
            }
            byte => {
                decoded.push(byte);
                index += 1;
            }
        }
    }
    String::from_utf8_lossy(&decoded).into_owned()
}

fn hex_value(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

fn html_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

fn normalize_difficulty(value: &str) -> String {
    match value.to_ascii_lowercase().as_str() {
        "easy" => "easy".to_owned(),
        "hard" | "harm" => "hard".to_owned(),
        _ => "medium".to_owned(),
    }
}

fn normalize_target(value: &str) -> String {
    match value.to_ascii_lowercase().as_str() {
        "white" => "white".to_owned(),
        "black" => "black".to_owned(),
        _ => "both".to_owned(),
    }
}

const BOOK_STYLE: &str = r#"
:root{color-scheme:light;--ink:#22242a;--muted:#666b74;--line:#d7dadd;--paper:#fff;--canvas:#f5f6f7;--accent:#b7410e;--link:#0969a2;--code:#f1f3f5}*{box-sizing:border-box}html{scroll-behavior:smooth;scroll-padding-top:82px}body{margin:0;background:var(--paper);color:var(--ink);font:16px/1.7 -apple-system,BlinkMacSystemFont,"Segoe UI",sans-serif}.book-page a{color:var(--link);text-decoration:none}.book-page a:hover{text-decoration:underline}.book-header{height:62px;position:sticky;top:0;z-index:4;background:#fff;border-bottom:1px solid var(--line);display:flex;align-items:center;padding:0 25px;gap:24px}.book-brand{display:flex;align-items:center;gap:11px;min-width:260px;color:var(--ink)!important}.book-brand:hover{text-decoration:none!important}.book-mark{display:grid;place-items:center;width:34px;height:34px;border-radius:7px;background:#81b64c;color:#172012;font-size:11px;font-weight:900;letter-spacing:-.04em}.book-brand strong,.book-brand small{display:block;line-height:1.35}.book-brand strong{font-size:13px}.book-brand small{font-size:10px;color:var(--muted)}.book-header-title{font-family:Georgia,"Times New Roman",serif;font-size:16px;font-weight:600;color:#474b52}.book-app-link{margin-left:auto;font-size:12px;font-weight:650}.book-app-link span{color:var(--accent);margin-left:5px}.book-shell{display:grid;grid-template-columns:276px minmax(0,1fr);min-height:calc(100vh - 62px)}.book-sidebar{position:sticky;top:62px;height:calc(100vh - 62px);overflow:auto;background:var(--canvas);border-right:1px solid var(--line);padding:28px 15px 25px 22px}.sidebar-heading{padding:0 10px 13px;color:#777c84;font-size:10px;font-weight:800;letter-spacing:.13em}.sidebar-chapters{display:grid;gap:2px}.sidebar-chapters a{display:grid;grid-template-columns:25px 1fr;align-items:start;gap:5px;border-radius:5px;padding:8px 9px;color:#3b4048;font-size:12px;line-height:1.45}.sidebar-chapters a.active{background:#e8ebed;color:#17191d;font-weight:700;border-left:3px solid var(--accent);padding-left:6px}.chapter-number{font-family:ui-monospace,SFMono-Regular,Menlo,monospace;color:#8a9097;font-size:10px;padding-top:1px}.sidebar-reference{display:block;margin:21px 9px 0;padding:11px 0;border-top:1px solid var(--line);font-size:11px;font-weight:700}.sidebar-reference span{color:var(--accent);padding-left:4px}.book-main{width:min(100%,1050px);padding:30px clamp(24px,6vw,82px) 38px;margin:0 auto}.book-breadcrumb{display:flex;gap:10px;align-items:center;color:#8a8e95;font-size:11px;margin:0 0 25px}.book-breadcrumb a{color:#6b727a}.book-content{max-width:790px}.book-content h1,.book-content h2,.book-content h3,.book-content h4{font-family:Georgia,"Times New Roman",serif;line-height:1.25;color:#1c1f24}.book-content h1{font-size:36px;letter-spacing:-.025em;margin:0 0 24px;padding-bottom:14px;border-bottom:1px solid var(--line)}.book-content h2{font-size:26px;margin:39px 0 12px}.book-content h3{font-size:20px;margin:30px 0 10px}.book-content h4{font-size:16px;margin:25px 0 8px}.book-content p{margin:0 0 17px}.book-content strong{font-weight:700}.book-content a{color:var(--link);text-decoration:underline;text-decoration-color:#a8c5d6;text-underline-offset:2px}.book-content code{font: .88em ui-monospace,SFMono-Regular,Menlo,Consolas,monospace;background:var(--code);border:1px solid #e3e5e8;border-radius:3px;padding:2px 5px}.book-content pre{overflow:auto;background:#f6f7f8;border:1px solid #d9dde1;border-radius:5px;padding:15px 17px;margin:20px 0 24px;line-height:1.55}.book-content pre code{background:transparent;border:0;padding:0;font-size:13px;color:#282c34}.book-content ul,.book-content ol{padding-left:26px;margin:0 0 19px}.book-content li{padding:2px 0}.book-content blockquote{border-left:3px solid #c6cbd0;margin:18px 0;padding:2px 16px;color:#555b63}.book-content hr{border:0;border-top:1px solid var(--line);margin:30px 0}.book-content table{border-collapse:collapse;width:100%;margin:19px 0 24px;font-size:14px}.book-content th,.book-content td{text-align:left;padding:8px 10px;border:1px solid var(--line)}.book-content th{background:#f4f5f6}.chapter-navigation{display:grid;grid-template-columns:1fr 1fr;gap:16px;max-width:790px;margin:48px 0 32px}.chapter-card{border:1px solid var(--line);border-radius:5px;padding:12px 14px;display:grid;gap:5px;color:#343941!important}.chapter-card:hover{background:#fafafa;text-decoration:none!important;border-color:#aeb4bb}.chapter-card small{font-size:9px;letter-spacing:.1em;color:#777d85}.chapter-card span{font-size:13px;font-weight:650}.chapter-card.next{text-align:right}.book-footer{max-width:790px;border-top:1px solid var(--line);padding-top:17px;display:flex;justify-content:space-between;gap:15px;color:#777d85;font-size:10px}.book-footer a{color:#6b727a}.book-footer:empty{display:none}
@media(max-width:760px){.book-header{padding:0 15px;gap:13px}.book-brand{min-width:0}.book-brand small{display:none}.book-header-title{font-size:13px}.book-app-link{font-size:11px}.book-shell{display:block}.book-sidebar{position:static;height:auto;max-height:250px;padding:12px 12px;border-right:0;border-bottom:1px solid var(--line)}.sidebar-heading{padding-bottom:7px}.sidebar-chapters{grid-template-columns:1fr 1fr;gap:0 4px}.sidebar-chapters a{font-size:10px;padding:6px}.sidebar-reference{margin-top:8px;padding-top:9px}.book-main{padding:24px 19px 30px}.book-content h1{font-size:30px}.book-content h2{font-size:23px}.book-content table{font-size:12px}}
.book-download{font-size:11px;font-weight:700;border:1px solid var(--line);border-radius:4px;padding:6px 9px;white-space:nowrap}.book-download:hover{background:var(--canvas);text-decoration:none!important}
@media(max-width:520px){.book-header-title{display:none}.book-download{font-size:9px;padding:5px 6px}}
"#;

const STYLE: &str = r#"
:root{color-scheme:dark;--bg:#171815;--panel:#20211e;--panel2:#252622;--line:#373a33;--muted:#a2a69a;--text:#f4f3ed;--green:#81b64c;--green2:#9ccc65;--cream:#eeeed2;--boarddark:#769656;--gold:#d6b15e;--red:#d27b6f}*{box-sizing:border-box}html{scroll-behavior:smooth}body{margin:0;background:var(--bg);color:var(--text);font-family:Inter,-apple-system,BlinkMacSystemFont,"Segoe UI",sans-serif;line-height:1.55}a{color:inherit;text-decoration:none}button,input,select{font:inherit}.topbar{height:76px;padding:0 clamp(20px,5vw,78px);display:flex;align-items:center;justify-content:space-between;border-bottom:1px solid #ffffff12;background:#171815eF;position:sticky;top:0;z-index:3;backdrop-filter:blur(16px)}.brand{display:inline-flex;align-items:center;gap:10px;font-size:13px;font-weight:900;letter-spacing:.08em}.brand-mark{width:34px;height:34px;display:grid;place-items:center;background:var(--green);border-radius:9px;color:#182015;font-size:12px;font-weight:950;letter-spacing:-.05em}.green{color:var(--green2)}nav{display:flex;gap:34px;font-size:13px;color:#c2c4ba}nav a:hover,nav a.active{color:var(--green2)}.nav-cta{font-size:12px;font-weight:750;border:1px solid #ffffff25;border-radius:8px;padding:10px 14px}.nav-cta span{color:var(--green2);margin-left:8px}main{max-width:1240px;margin:auto;padding:0 42px}.hero{position:relative;display:grid;grid-template-columns:1fr .82fr;align-items:center;gap:50px;padding:92px 0 70px;min-height:535px}.eyebrow{font-size:10px;font-weight:800;letter-spacing:.16em;color:var(--green2)}.pulse{width:7px;height:7px;display:inline-block;border-radius:50%;background:var(--green);box-shadow:0 0 11px #81b64c9c;margin-right:7px}.hero h1,.guide-hero h1{font-size:clamp(54px,7vw,86px);letter-spacing:-.075em;line-height:.98;margin:18px 0 22px;font-weight:790}.hero h1 em,.guide-hero h1 em{color:var(--green2);font-style:normal}.lede{font-size:17px;color:#b0b3a8;max-width:470px;line-height:1.7}.hero-actions{display:flex;align-items:center;gap:22px;flex-wrap:wrap;margin-top:30px}.button{border:0;border-radius:8px;padding:13px 18px;font-weight:760;display:inline-flex;align-items:center;justify-content:space-between;gap:22px;font-size:13px;cursor:pointer;transition:.18s ease}.button span{font-size:18px;line-height:12px}.button-green{background:var(--green);color:#172012}.button-green:hover{background:var(--green2);transform:translateY(-1px)}.button-outline{border:1px solid #ffffff35;background:transparent;color:var(--text)}.button-outline:hover{border-color:var(--green);color:var(--green2)}.text-link,.demo-link{font-size:12px;font-weight:680;color:#d8dacd}.text-link span,.demo-link span{color:var(--green2);margin-left:5px}.hero-board-wrap{max-width:410px;justify-self:center;width:100%;padding:15px;background:#242520;border:1px solid #ffffff17;border-radius:14px;box-shadow:0 28px 80px #0006;transform:rotate(1.5deg)}.hero-board{aspect-ratio:1;overflow:hidden;border-radius:5px}.board-caption{display:flex;justify-content:space-between;color:#898d80;font-size:9px;letter-spacing:.13em;padding:12px 3px 1px}.hero-stamp{position:absolute;right:0;bottom:58px;width:78px;height:78px;border-radius:50%;background:var(--green);color:#1e2a16;display:flex;align-items:center;justify-content:center;gap:5px;transform:rotate(9deg)}.hero-stamp span{font-weight:900;font-size:24px}.hero-stamp small{font-size:8px;line-height:1.2;font-weight:800}.inspiration{display:flex;align-items:center;gap:17px;background:#21231e;border:1px solid #34382f;border-radius:10px;padding:16px 20px;margin:0 0 78px}.inspiration-icon{background:#34392e;border-radius:8px;width:35px;height:35px;display:grid;place-items:center;color:var(--green2);font-size:12px;flex:0 0 auto}.inspiration p{color:#b9bcb1;font-size:12px;margin:0;flex:1}.inspiration p strong{color:#f0f1e8}.inspiration>a{color:var(--green2);font-size:11px;font-weight:700;white-space:nowrap}.intro-row{display:grid;grid-template-columns:.82fr 1.25fr auto;align-items:end;gap:42px;padding:0 2px 82px;border-bottom:1px solid #ffffff1e}.intro-row h2,.section-head h2{font-size:42px;letter-spacing:-.06em;line-height:1.05;margin:12px 0 0}.intro-row>p{color:#b0b3a8;font-size:14px;line-height:1.8;margin:0;max-width:570px}.mode-section{padding:82px 0 0;scroll-margin-top:95px}.section-head{display:flex;align-items:end;justify-content:space-between;margin-bottom:26px}.section-head p{color:var(--muted);text-align:right;font-size:13px}.mode-card{border:1px solid #393c34;border-radius:13px;background:linear-gradient(130deg,#22231f,#1d1e1b);display:grid;position:relative;overflow:hidden}.elo-card{grid-template-columns:.9fr 1.1fr}.card-number{position:absolute;left:21px;top:18px;font-size:11px;color:#6f7467;font-weight:700;letter-spacing:.15em}.mode-copy{padding:72px 42px 38px;border-right:1px solid #ffffff16;position:relative;overflow:hidden}.mode-copy:after{content:"♞";position:absolute;right:-18px;bottom:-76px;font-size:230px;color:#ffffff05;transform:rotate(-14deg)}.mode-icon{width:40px;height:40px;border-radius:10px;background:#30392a;color:var(--green2);display:grid;place-items:center;font-size:23px}.mode-copy h3,.opening-form h3{font-size:39px;line-height:1.02;letter-spacing:-.055em;margin:22px 0 15px}.mode-copy p,.opening-form>p{font-size:13px;color:#aeb1a5;line-height:1.75;max-width:340px}.tag-row{display:flex;gap:7px;flex-wrap:wrap;margin-top:26px}.tag-row span{font-size:8px;letter-spacing:.09em;border:1px solid #ffffff1c;border-radius:999px;padding:6px 8px;color:#b4b7ab}.mode-form{padding:30px 34px}.mode-form h4{font-size:14px;margin:0 0 20px}.mode-form label,.opening-form label,.guess-form label{display:block;font-size:10px;font-weight:750;color:#d5d7cb;margin-bottom:12px}.mode-form label>span{font-weight:500;color:#858b7c;margin-left:6px}.mode-form input,.mode-form select,.opening-form input,.guess-form input{display:block;width:100%;margin-top:7px;border:1px solid #41443c;background:#191a17;border-radius:7px;padding:11px 12px;color:var(--text);outline:none;font-size:12px}.mode-form input:focus,.mode-form select:focus,.opening-form input:focus,.guess-form input:focus{border-color:var(--green)}.form-row{display:grid;grid-template-columns:1fr 1fr;gap:12px}.full{width:100%;margin-top:5px}.form-divider{display:flex;align-items:center;gap:12px;color:#777c70;font-size:8px;letter-spacing:.13em;margin:20px 0 16px}.form-divider:before,.form-divider:after{content:"";height:1px;background:#383b33;flex:1}.demo-link{display:inline-block;margin-top:18px;font-size:11px}.api-note{display:flex;gap:10px;color:#8f9388;font-size:11px;align-items:center;margin:15px 0 0}.api-note span{border:1px solid #63684f;color:var(--green2);width:17px;height:17px;border-radius:50%;text-align:center;line-height:15px;flex:0 0 auto}.opening-section{padding-bottom:65px}.opening-card{grid-template-columns:1.05fr .95fr}.opening-preview{padding:32px;background:#1b1c19;border-right:1px solid #ffffff15}.preview-label{font-size:9px;letter-spacing:.15em;color:#adb1a3;margin-bottom:20px}.mini-board{max-width:330px;margin:auto;border:7px solid #2c2e28;border-radius:4px;box-shadow:0 15px 38px #0005}.mini-board-grid{display:grid;grid-template-columns:repeat(8,1fr);aspect-ratio:1}.mini-board-grid .square{font-size:clamp(14px,2.5vw,26px)}.preview-moves{max-width:330px;margin:14px auto 0;color:#bdc1b3;font-size:11px;word-spacing:5px}.preview-moves b{color:var(--green2)}.opening-form{padding:38px}.opening-form h3{font-size:34px}.opening-form form{margin-top:23px}.opening-form>p{margin-bottom:2px}.build-note{display:flex;align-items:center;gap:22px;border-top:1px solid #ffffff1f;padding:36px 0;margin-bottom:30px}.build-icon{width:48px;height:48px;background:#292b25;border:1px solid #41443a;border-radius:10px;display:grid;place-items:center;color:var(--green2);font-size:22px;flex:0 0 auto}.build-note h2{font-size:19px;margin:5px 0}.build-note p{color:#a7aa9f;font-size:12px;max-width:520px;margin:0}.build-actions{margin-left:auto;display:flex;align-items:center;gap:16px}.build-actions code,.code-card code{color:var(--green2);font-size:11px;background:#151612;border:1px solid #33362e;border-radius:6px;padding:9px 11px}footer{display:flex;align-items:center;justify-content:space-between;border-top:1px solid #ffffff1b;padding:25px 0 34px;color:#878b7f;font-size:10px}footer .brand{color:var(--text);font-size:11px}footer .brand-mark{height:28px;width:28px;font-size:11px}footer>a:last-child{color:#c2c5b9}.toast{margin:22px 0;padding:13px 16px;border-radius:8px;background:#2a3024;border:1px solid #52653c;color:#cfe7ac;font-size:12px}.play-main{padding-top:25px}.play-heading{display:flex;align-items:end;justify-content:space-between;margin:26px 0 24px}.back-link{color:#adb1a4;font-size:11px}.play-heading .eyebrow{margin-top:17px}.play-heading h1{font-size:36px;letter-spacing:-.055em;margin:6px 0 0}.round-badge{border:1px solid #42453d;border-radius:999px;padding:8px 11px;color:#c7cabc;font-size:8px;letter-spacing:.11em}.play-grid{display:grid;grid-template-columns:minmax(390px,1.13fr) minmax(300px,.87fr);gap:18px;align-items:start}.board-panel,.guess-panel{background:var(--panel);border:1px solid #383b33;border-radius:11px;padding:18px}.player-line{display:flex;align-items:center;gap:11px;padding:3px 2px 14px}.piece-dot{font-size:25px;width:31px;text-align:center}.white-dot{color:#eeeed2;text-shadow:0 1px 2px #000}.black-dot{color:#1b1d18;text-shadow:0 0 1px #fff}.player-line strong{display:block;font-size:12px}.player-line small{display:block;color:#909489;font-size:9px;margin-top:1px}.rating-lock{margin-left:auto;letter-spacing:3px;color:#8e9386;font-size:10px}.board-area{padding:8px;background:#2b2d27;border-radius:5px}.board{display:grid;grid-template-columns:repeat(8,1fr);aspect-ratio:1;overflow:hidden;border-radius:2px}.square{display:grid;place-items:center;aspect-ratio:1;line-height:1;font-family:"Arial Unicode MS", "DejaVu Sans",serif}.board .square{font-size:clamp(25px,4.7vw,54px)}.light-square{background:var(--cream);color:#25251f}.dark-square{background:var(--boarddark);color:#20261b}.move-controls{display:flex;justify-content:space-between;align-items:center;padding:12px 5px;color:#b6bbac}.move-controls a{width:32px;height:30px;display:grid;place-items:center;background:#30322c;border-radius:5px;font-size:15px}.move-controls a:hover{background:#3b4532;color:var(--green2)}.move-controls span{font-size:9px;letter-spacing:.13em;color:#9ea396}.move-list{border-top:1px solid #393b35;margin-top:2px;padding-top:13px}.move-list-head{display:flex;justify-content:space-between;color:#868b7f;font-size:8px;letter-spacing:.14em;margin-bottom:10px}.move-grid{display:grid;grid-template-columns:32px 1fr 1fr;gap:2px 5px;max-height:140px;overflow:auto}.move-number{color:#73796d;font-size:10px;padding:4px}.move-token{font-size:11px;padding:4px 7px;border-radius:4px;color:#d4d7ca}.move-token:hover,.move-token.selected{background:#3d4b31;color:#c9efa0}.guess-panel{padding:24px 23px}.panel-kicker{display:flex;align-items:center;gap:10px;color:var(--green2);font-size:9px;font-weight:800;letter-spacing:.14em}.small-icon{width:30px;height:30px;font-size:18px}.guess-panel h2{font-size:31px;line-height:1.04;letter-spacing:-.055em;margin:18px 0 11px}.panel-description{font-size:11px;color:#a6aa9e;line-height:1.7}.panel-description strong{color:var(--green2)}.difficulty-pill{display:inline-flex;gap:10px;align-items:center;border:1px solid #3b4034;border-radius:999px;color:#adb2a2;font-size:8px;letter-spacing:.1em;padding:7px 10px;margin:5px 0 17px}.difficulty-pill span{color:var(--green2);font-weight:800;font-size:10px}.guess-form label{margin:0 0 11px}.guess-form button{margin-top:8px}.result-box{margin-top:16px;border:1px solid #566b43;background:#2a3024;color:#d1e8b3;border-radius:7px;padding:12px;font-size:11px;line-height:1.8}.mode-selector-label{border-top:1px solid #383a34;padding-top:16px;margin-top:19px;color:#83887b;font-size:8px;letter-spacing:.13em}.settings-chips{display:flex;gap:7px;margin-top:8px}.settings-chips span{font-size:8px;color:#c0c4b7;background:#30322c;border-radius:99px;padding:6px 8px}.sequence-card{background:#191a17;border:1px solid #373a32;border-radius:7px;padding:12px;margin:20px 0}.sequence-card>span{font-size:8px;letter-spacing:.1em;color:#868b80}.sequence-card>strong{float:right;font-size:9px;color:var(--green2)}.sequence-card>div{margin-top:7px;color:#b7bdab;font-size:12px;font-weight:800}.opening-tip{font-size:9px;color:#8d9286;margin-top:12px}.game-meta{display:flex;align-items:center;gap:34px;padding:18px 6px;margin:5px 0 25px;border-bottom:1px solid #ffffff18}.game-meta>span{display:grid;gap:3px}.game-meta small{font-size:8px;letter-spacing:.1em;color:#81867a}.game-meta b{font-size:10px;font-weight:650;color:#c9ccbf}.game-meta .source-link{margin-left:auto;color:var(--green2);font-size:10px}.muted{color:#969a8e;font-size:11px}.guide-main{display:grid;grid-template-columns:240px 1fr;gap:65px;padding-top:70px}.guide-hero{grid-column:1/-1;background:linear-gradient(105deg,#23251f,#191a17);border:1px solid #373a31;border-radius:13px;padding:48px 54px}.guide-hero h1{font-size:62px;margin:14px 0}.guide-hero p{color:#aeb2a5;max-width:550px;font-size:14px}.guide-hero .tag-row{margin-top:22px}.toc{position:sticky;top:104px;align-self:start;padding:15px 0}.toc b{font-size:9px;letter-spacing:.13em;color:#8a8f82;display:block;margin-bottom:13px}.toc a{display:block;padding:9px 0;color:#b7bbaf;font-size:11px;border-bottom:1px solid #ffffff0d}.toc a:hover{color:var(--green2)}.guide-content section{display:grid;grid-template-columns:45px 1fr;gap:18px;padding:4px 0 35px;margin-bottom:24px;border-bottom:1px solid #ffffff1a;scroll-margin-top:100px}.chapter{font-size:10px;color:var(--green2);font-weight:800;padding-top:8px}.guide-content h2{font-size:25px;letter-spacing:-.04em;margin:0 0 13px}.guide-content h3{font-size:15px;margin:22px 0 4px}.guide-content p{font-size:12px;line-height:1.85;color:#adb1a5}.guide-content p a{color:var(--green2);text-decoration:underline;text-underline-offset:3px}.guide-content code{color:#c4e69b;background:#262921;padding:2px 4px;border-radius:3px;font-size:11px}.code-card{padding:14px;background:#11120f;border:1px solid #34372f;border-radius:7px;margin:14px 0}.code-card code{border:0;background:transparent;padding:0}.guide-content kbd{background:#2b2d27;border:1px solid #45483f;border-radius:4px;padding:2px 6px;color:#e8ebdf;font-size:10px}.guide-content .button{margin-top:10px}.inspiration,.mode-section,.intro-row{scroll-margin-top:90px}
@media(max-width:900px){main{padding:0 25px}.hero{gap:22px;padding-top:65px}.hero-board-wrap{max-width:340px}.intro-row{grid-template-columns:.8fr 1.2fr}.intro-row .button{grid-column:2;justify-self:start}.elo-card{grid-template-columns:.8fr 1.2fr}.mode-copy{padding-left:27px;padding-right:25px}.mode-form{padding:27px 23px}.play-grid{grid-template-columns:minmax(300px,1fr) minmax(250px,.8fr);gap:12px}.board-panel,.guess-panel{padding:14px}.guide-main{grid-template-columns:190px 1fr;gap:35px}}
@media(max-width:680px){.topbar{height:64px;padding:0 17px}.topbar nav{gap:12px;font-size:10px}.topbar nav a:nth-child(2){display:none}.nav-cta{font-size:0;padding:8px 11px}.nav-cta span{font-size:15px;margin:0}main{padding:0 17px}.hero{display:flex;flex-direction:column;align-items:start;padding:52px 0 45px;gap:30px}.hero h1{font-size:61px}.lede{font-size:14px}.hero-board-wrap{width:min(84%,330px);align-self:center}.hero-stamp{right:3px;bottom:5px;width:65px;height:65px}.hero-actions{gap:15px}.inspiration{align-items:flex-start;flex-wrap:wrap;margin-bottom:56px}.inspiration p{flex-basis:calc(100% - 55px)}.inspiration>a{margin-left:51px}.intro-row{display:block;padding-bottom:52px}.intro-row h2{font-size:37px;margin-bottom:18px}.intro-row>p{margin:0 0 20px}.section-head{align-items:flex-start;gap:12px}.section-head h2{font-size:35px}.section-head p{font-size:10px}.mode-section{padding-top:55px}.elo-card,.opening-card{grid-template-columns:1fr}.mode-copy{padding:54px 25px 27px;border-right:0;border-bottom:1px solid #ffffff16}.mode-copy h3{font-size:34px}.mode-form,.opening-form{padding:25px 20px}.opening-preview{padding:23px;border-right:0;border-bottom:1px solid #ffffff16}.build-note{align-items:flex-start;flex-wrap:wrap}.build-note>div:nth-child(2){flex:1}.build-actions{margin-left:70px;flex-wrap:wrap}.build-note p{font-size:11px}footer{gap:12px;flex-wrap:wrap}footer>span{order:3;width:100%}.play-main{padding-top:12px}.play-heading{margin:20px 0}.play-heading h1{font-size:28px}.round-badge{font-size:7px}.play-grid{grid-template-columns:1fr}.board-panel,.guess-panel{padding:14px}.board .square{font-size:clamp(31px,10vw,52px)}.guess-panel{padding:22px}.game-meta{gap:15px;flex-wrap:wrap}.game-meta>span{min-width:27%}.game-meta .source-link{margin-left:0}.guide-main{display:block;padding-top:35px}.guide-hero{padding:29px 23px}.guide-hero h1{font-size:46px}.toc{position:static;margin:25px 0}.guide-content section{grid-template-columns:29px 1fr;gap:11px;padding-bottom:24px}.guide-content h2{font-size:22px}}
"#;
