#[path = "../src/codebase_hard.rs"]
mod exercise;

#[test]
fn route_owner_handles_query_strings_and_guide_chapters() {
    assert_eq!(exercise::route_owner("/play?sid=12"), Some("src/web.rs"));
    assert_eq!(exercise::route_owner("/guide/function-web-route"), Some("src/web.rs"));
    assert_eq!(exercise::route_owner("/unknown"), None);
}
