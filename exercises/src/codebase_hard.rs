// NOTE: The Rust web module owns route selection for game and Guidebook requests.
// TODO: Implement this whole function: remove the query string and select the owner.
pub fn route_owner(request_target: &str) -> Option<&'static str> {
    todo!("map supported GTE request paths to their source module")
}
