// NOTE: Each mode has a fixed label that the interface can display.
// TODO: Replace the two placeholder strings with the correct labels.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameMode {
    GuessElo,
    GuessOpening,
}

pub fn label(mode: GameMode) -> &'static str {
    match mode {
        GameMode::GuessElo => "???",
        GameMode::GuessOpening => "???",
    }
}
