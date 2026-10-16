//! HTTP transport and JSON decoding for Chess.com public data.

use serde_json::Value;

const USER_AGENT: &str = "ChessProject prototype (Rust learning project)";

pub(super) fn get_json(url: &str) -> Result<Value, String> {
    let response = ureq::get(url)
        .set("User-Agent", USER_AGENT)
        .call()
        .map_err(|error| format!("Chess.com API request failed: {error}"))?;
    response
        .into_json::<Value>()
        .map_err(|error| format!("Chess.com returned unreadable JSON: {error}"))
}
