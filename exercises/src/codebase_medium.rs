// NOTE: Map each explicit game mode to the module that owns its behavior.
// TODO: Write the match arms in `owner` and keep both variants distinct.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameMode {
    GuessElo,
    GuessOpening,
}

pub fn owner(mode: GameMode) -> &'static str {
    todo!("map each mode to its application module")
}
