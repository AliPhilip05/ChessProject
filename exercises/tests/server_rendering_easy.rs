#[path = "../src/server_rendering_easy.rs"]
mod exercise;

#[test]
fn move_url_carries_the_session_and_requested_ply() {
    assert_eq!(exercise::move_url("42", 7), "/move?sid=42&ply=7");
}
