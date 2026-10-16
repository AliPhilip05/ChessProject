//! Embedded book chapters and generated per-function pages.
use std::sync::OnceLock;

pub(in crate::web) struct BookChapter {
    pub(in crate::web) slug: String,
    pub(in crate::web) title: String,
    pub(in crate::web) source: String,
}

const BASE_BOOK_CHAPTERS: &[(&str, &str, &str)] = &[
    (
        "introduction",
        "Introduction",
        include_str!("../../../docs/book/src/introduction.md"),
    ),
    (
        "question-zero",
        "Question 0: Start with GTE",
        include_str!("../../../docs/book/src/question-zero.md"),
    ),
    (
        "getting-started",
        "Getting Started",
        include_str!("../../../docs/book/src/getting-started.md"),
    ),
    (
        "architecture",
        "Project Architecture",
        include_str!("../../../docs/book/src/architecture.md"),
    ),
    (
        "rust-entry-point",
        "Rust Entry Points and Types",
        include_str!("../../../docs/book/src/rust-entry-point.md"),
    ),
    (
        "public-data",
        "Chess.com Public Data",
        include_str!("../../../docs/book/src/public-data.md"),
    ),
    (
        "board-state",
        "PGN and Board State",
        include_str!("../../../docs/book/src/board-state.md"),
    ),
    (
        "game-modes",
        "The Two Game Modes",
        include_str!("../../../docs/book/src/game-modes.md"),
    ),
    (
        "web-server",
        "The Web Server and Rendered Pages",
        include_str!("../../../docs/book/src/web-server.md"),
    ),
    (
        "terminal-learning",
        "Terminal Lessons",
        include_str!("../../../docs/book/src/terminal-learning.md"),
    ),
    (
        "capstone",
        "Capstone",
        include_str!("../../../docs/book/src/capstone.md"),
    ),
    (
        "reference",
        "Codebase Walkthrough — reference.md",
        include_str!("../../../exercises/reference.md"),
    ),
];

const FUNCTION_PAGE_SOURCE: &str = include_str!("../../../docs/book/src/function-pages.md");

pub(in crate::web) fn book_chapters() -> &'static [BookChapter] {
    static CHAPTERS: OnceLock<Vec<BookChapter>> = OnceLock::new();
    CHAPTERS.get_or_init(|| {
        let mut chapters: Vec<BookChapter> = BASE_BOOK_CHAPTERS
            .iter()
            .map(|(slug, title, source)| BookChapter {
                slug: (*slug).to_owned(),
                title: (*title).to_owned(),
                source: (*source).to_owned(),
            })
            .collect();
        let mut page: Option<(String, String, String)> = None;
        for line in FUNCTION_PAGE_SOURCE.lines() {
            if let Some(metadata) = line
                .strip_prefix("<!-- GTE_FUNCTION_PAGE: ")
                .and_then(|line| line.strip_suffix(" -->"))
            {
                let (slug, title) = metadata.split_once('|').unwrap_or((metadata, metadata));
                page = Some((slug.to_owned(), title.to_owned(), String::new()));
            } else if line == "<!-- END_GTE_FUNCTION_PAGE -->" {
                if let Some((slug, title, source)) = page.take() {
                    chapters.push(BookChapter {
                        slug,
                        title: format!("Function: {title}"),
                        source: source.trim().to_owned(),
                    });
                }
            } else if let Some((_, _, source)) = page.as_mut() {
                source.push_str(line);
                source.push('\n');
            }
        }
        chapters
    })
}

pub(in crate::web) fn book_chapter_index(slug: &str) -> Option<usize> {
    book_chapters()
        .iter()
        .position(|chapter| chapter.slug == slug)
}
