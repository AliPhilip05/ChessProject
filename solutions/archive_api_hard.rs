pub fn newest_month_with_games(archives: &[Vec<String>]) -> Option<&[String]> {
    archives
        .iter()
        .rev()
        .find(|games| !games.is_empty())
        .map(Vec::as_slice)
}
