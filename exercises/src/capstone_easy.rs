// NOTE: A game with N plies has N + 1 reviewable positions, starting at ply zero.
// TODO: Change the exclusive range so the final ply is included.
pub fn reviewable_plies(move_count: usize) -> Vec<usize> {
    let final_ply = move_count;
    (0..final_ply).collect()
}
