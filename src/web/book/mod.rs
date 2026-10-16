//! The Guidebook catalog, reader page, and safe Markdown subset.
mod catalog;
mod inline;
mod markdown;
mod reader;

pub(super) use catalog::{book_chapter_index, book_chapters};
pub(super) use reader::{download_book_markdown, render_book};
