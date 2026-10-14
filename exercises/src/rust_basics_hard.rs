// NOTE: Keep external strings out of the app state by converting them to a closed enum.
// TODO: Implement the complete parser with an exhaustive match and a safe fallback.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameMode {
    GuessElo,
    GuessOpening,
}

pub fn parse_mode(argument: &str) -> Option<GameMode> {
    todo!("parse the supported mode names")
}
