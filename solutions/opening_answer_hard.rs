pub fn matches_opening(guess: &str, accepted_names: &[&str]) -> bool {
    let normalize = |value: &str| {
        value.split_whitespace().collect::<Vec<_>>().join(" ").to_ascii_lowercase()
    };
    let guess = normalize(guess);
    !guess.is_empty() && accepted_names.iter().any(|name| normalize(name) == guess)
}
