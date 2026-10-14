#[path = "../src/archive_api_easy.rs"]
mod exercise;

#[test]
fn archives_are_limited_and_returned_newest_first() {
    let urls = ["jan", "feb", "mar", "apr"];
    assert_eq!(exercise::recent_archives(&urls, 2), vec!["apr", "mar"]);
    assert_eq!(exercise::recent_archives(&urls, 0), Vec::<&str>::new());
}
