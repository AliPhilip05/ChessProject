pub fn recent_archives<'a>(urls: &[&'a str], limit: usize) -> Vec<&'a str> {
    let mut selected = Vec::new();
    for archive in urls.iter().rev().take(limit) {
        selected.push(*archive);
    }
    selected
}
