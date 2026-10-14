#[path = "../src/archive_api.rs"]
mod exercise;

#[test]
fn newest_archive_urls_are_selected_first_and_limited() {
    let archives = vec!["jan".into(), "feb".into(), "mar".into()];
    assert_eq!(
        exercise::recent_archive_urls(&archives, 2),
        vec!["mar", "feb"]
    );
    assert_eq!(
        exercise::recent_archive_urls(&archives, 0),
        Vec::<String>::new()
    );
}

#[test]
fn username_validation_rejects_empty_or_unsafe_input() {
    assert!(exercise::valid_username("ChessLearner_7"));
    assert!(!exercise::valid_username(""));
    assert!(!exercise::valid_username("name/path"));
}
