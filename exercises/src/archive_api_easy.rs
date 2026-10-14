// NOTE: Chess.com returns archive months oldest first; GTE reviews recent months first.
// TODO: Add each visited URL to `selected` (the exercise is guided one line at a time).
pub fn recent_archives<'a>(urls: &[&'a str], limit: usize) -> Vec<&'a str> {
    let mut selected = Vec::new();
    for _archive in urls.iter().rev().take(limit) {
        // TODO: selected.push(???);
    }
    selected
}
