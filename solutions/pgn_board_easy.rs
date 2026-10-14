pub fn is_san_move(token: &str) -> bool {
    !token.is_empty()
        && !token.starts_with('$')
        && !matches!(token, "1-0" | "0-1" | "1/2-1/2" | "*")
        && !token.chars().all(|character| character.is_ascii_digit() || character == '.')
}
