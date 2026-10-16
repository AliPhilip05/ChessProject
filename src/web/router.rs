//! Maps URL paths to the appropriate browser handler.
use super::book::{book_chapter_index, book_chapters, download_book_markdown, render_book};
use super::handlers::{
    load_game, render_session, submit_elo_guess, submit_opening_guess, update_move,
};
use super::http::HttpResponse;
use super::render::render_home;
use super::request::parse_query;

#[cfg(test)]
mod tests;

pub(super) fn route(target: &str) -> HttpResponse {
    let (path, query) = target.split_once('?').unwrap_or((target, ""));
    let params = parse_query(query);
    match path {
        "/" => HttpResponse::Page(render_home("")),
        "/guide" => HttpResponse::Page(render_book("introduction")),
        "/guide/download" => HttpResponse::Download(download_book_markdown()),
        "/guide/functions.pdf" => {
            HttpResponse::Pdf(include_bytes!("../../output/pdf/GTE-Function-Appendix.pdf").to_vec())
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
