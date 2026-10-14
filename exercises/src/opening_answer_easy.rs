// NOTE: Opening names are compared as words, not by the user's capitalization or spacing.
// TODO: Normalize the joined words to lowercase before returning the value.
pub fn normalize_opening(answer: &str) -> String {
    let words: Vec<&str> = answer.split_whitespace().collect();
    words.join(" ")
}
