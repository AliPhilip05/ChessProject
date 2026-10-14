#[path = "../src/archive_api_hard.rs"]
mod exercise;

#[test]
fn search_skips_empty_months_and_stops_at_the_newest_usable_one() {
    let archives = vec![vec!["old".into()], vec![], vec!["new".into()], vec![]];
    assert_eq!(exercise::newest_month_with_games(&archives), Some(["new".to_owned()].as_slice()));
    assert_eq!(exercise::newest_month_with_games(&[vec![], vec![]]), None);
}
