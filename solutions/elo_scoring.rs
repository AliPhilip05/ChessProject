pub fn tolerance(difficulty: &str) -> i32 {
    match difficulty.to_ascii_lowercase().as_str() {
        "easy" => 200,
        "hard" => 50,
        _ => 100,
    }
}

pub fn within_tolerance(guess: i32, actual: i32, difficulty: &str) -> bool {
    (guess - actual).abs() <= tolerance(difficulty)
}
