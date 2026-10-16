//! The overview page and game-source forms.
use super::board::mini_board;
use super::html::{html_escape, page};
use crate::chess::ChessGame;

pub(in crate::web) fn render_home(message: &str) -> String {
    let body = format!(
        include_str!("../../../assets/templates/home.html"),
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
