#[path = "../src/intro_0.rs"]
mod exercise;

#[test]
fn project_has_a_gte_mark_and_two_entry_points() {
    assert_eq!(exercise::product_mark(), "GTE");
    assert_eq!(exercise::entrypoint(false), "cli::run");
    assert_eq!(exercise::entrypoint(true), "web::serve");
}

#[test]
fn codebase_map_points_to_the_main_components() {
    assert_eq!(exercise::COMPONENTS.len(), 6);
    assert_eq!(
        exercise::component("src/chess.rs"),
        Some(&exercise::COMPONENTS[4])
    );
    assert_eq!(
        exercise::component("src/web.rs").unwrap().purpose,
        "serves the game app and the GTE Book"
    );
    assert_eq!(exercise::component("src/missing.rs"), None);
}
