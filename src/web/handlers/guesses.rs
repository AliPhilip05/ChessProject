//! Validates submitted guesses and records round feedback.
use crate::chess::{opening_matches, rating_is_close};
use crate::web::http::HttpResponse;
use crate::web::render::{html_escape, render_home};
use crate::web::session::{GameMode, sessions};
use std::collections::HashMap;

pub(in crate::web) fn submit_elo_guess(params: &HashMap<String, String>) -> HttpResponse {
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

pub(in crate::web) fn submit_opening_guess(params: &HashMap<String, String>) -> HttpResponse {
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
