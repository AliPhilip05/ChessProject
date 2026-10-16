//! Browser application facade. Transport, state, handlers, rendering, and the book
//! reader live in focused child modules.
mod book;
mod handlers;
mod http;
mod render;
mod request;
mod router;
mod session;

pub use http::serve;
