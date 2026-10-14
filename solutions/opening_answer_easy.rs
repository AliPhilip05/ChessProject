pub fn normalize_opening(answer: &str) -> String {
    answer
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_ascii_lowercase()
}
