//! Reads sessions and changes the selected board position.
use crate::web::http::HttpResponse;
use crate::web::render::{html_escape, render_game, render_home};
use crate::web::session::{max_ply, sessions};
use std::collections::HashMap;

pub(in crate::web) fn render_session(params: &HashMap<String, String>) -> HttpResponse {
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

pub(in crate::web) fn update_move(params: &HashMap<String, String>) -> HttpResponse {
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
