// NOTE: Represent the app's two modes as named alternatives that tests can compare.
// TODO: Derive PartialEq and Eq so assert_eq! can compare modes, then finish label below.
#[derive(Clone, Copy, Debug)]
pub enum GameMode {
    GuessElo,
    GuessOpening,
}

// NOTE: Return the visible title for the selected game mode.
// TODO: Replace the placeholder with a match over both enum variants.
pub fn label(mode: GameMode) -> &'static str {
    todo!("return the display name for each game mode")
}
