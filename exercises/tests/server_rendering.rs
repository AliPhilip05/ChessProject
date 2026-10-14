#[path = "../src/server_rendering.rs"]
mod exercise;

#[test]
fn move_links_point_to_a_session_and_ply() {
    assert_eq!(exercise::move_url("42", 7), "/move?sid=42&ply=7");
}

#[test]
fn user_text_is_escaped_before_it_enters_html() {
    assert_eq!(
        exercise::escape_html("<a x=\"&\">'"),
        "&lt;a x=&quot;&amp;&quot;&gt;&#39;"
    );
}
