pub fn route_owner(request_target: &str) -> Option<&'static str> {
    let path = request_target.split('?').next().unwrap_or(request_target);
    match path {
        "/" | "/new" | "/load" | "/play" | "/move" | "/guess-elo" | "/guess-opening" => {
            Some("src/web.rs")
        }
        "/guide" | "/guide/download" | "/guide/functions.pdf" => Some("src/web.rs"),
        path if path.starts_with("/guide/") => Some("src/web.rs"),
        _ => None,
    }
}
