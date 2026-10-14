// NOTE: A rating miss is an absolute distance, regardless of which rating was larger.
// TODO: Convert the signed difference to a non-negative value before returning it.
pub fn rating_error(guess: i32, actual: i32) -> i32 {
    let difference = guess - actual;
    difference
}
