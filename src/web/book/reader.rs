//! The Guidebook reader shell, chapter navigation, and Markdown download.
use super::catalog::{book_chapter_index, book_chapters};
use super::markdown::render_markdown;
use crate::web::render::html_escape;

const BOOK_STYLE: &str = include_str!("../../../assets/styles/book.css");

pub(in crate::web) fn render_book(slug: &str) -> String {
    let Some(index) = book_chapter_index(slug) else {
        return String::new();
    };
    let chapters = book_chapters();
    let chapter = &chapters[index];
    let mut sidebar = String::new();
    for (chapter_index, item) in chapters.iter().enumerate() {
        let active = if chapter_index == index {
            " class=\"active\" aria-current=\"page\""
        } else {
            ""
        };
        sidebar.push_str(&format!(
            "<a{active} href=\"/guide/{}\"><span class=\"chapter-number\">{:02}</span>{}</a>",
            item.slug,
            chapter_index + 1,
            html_escape(&item.title)
        ));
    }
    let previous = index.checked_sub(1).map(|previous_index| {
        let item = &chapters[previous_index];
        format!(
            "<a class=\"chapter-card previous\" href=\"/guide/{}\"><small>PREVIOUS</small><span>← {}</span></a>",
            item.slug,
            html_escape(&item.title)
        )
    }).unwrap_or_else(|| "<span></span>".to_owned());
    let next = chapters.get(index + 1).map(|item| {
        format!(
            "<a class=\"chapter-card next\" href=\"/guide/{}\"><small>NEXT</small><span>{} →</span></a>",
            item.slug,
            html_escape(&item.title)
        )
    }).unwrap_or_default();
    let body = format!(
        include_str!("../../../assets/templates/book.html"),
        html_escape(&chapter.title),
        html_escape(&chapter.title),
        render_markdown(&chapter.source),
        previous,
        next,
        sidebar = sidebar,
    );
    format!(
        include_str!("../../../assets/templates/book-page.html"),
        html_escape(&chapter.title),
        BOOK_STYLE = BOOK_STYLE,
        body = body,
    )
}

pub(in crate::web) fn download_book_markdown() -> String {
    let mut markdown =
        String::from("# The GTE Book\n\nA Rust chess app and terminal learning project.\n");
    for chapter in book_chapters() {
        markdown.push_str("\n\n---\n\n");
        markdown.push_str(chapter.source.trim());
        markdown.push('\n');
    }
    markdown
}
