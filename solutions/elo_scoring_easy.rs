pub fn rating_error(guess: i32, actual: i32) -> i32 {
    (guess - actual).abs()
}
