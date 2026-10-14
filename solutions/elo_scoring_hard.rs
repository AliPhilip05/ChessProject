pub fn evaluate_guess(guess: Option<i32>, actual: Option<i32>, tolerance: i32) -> Option<bool> {
    let guess = guess?;
    let actual = actual?;
    if tolerance < 0 {
        return None;
    }
    Some((guess - actual).abs() <= tolerance)
}
