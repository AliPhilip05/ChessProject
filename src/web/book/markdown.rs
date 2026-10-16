//! Block parsing for the Guidebook's supported Markdown syntax.
use super::inline::inline_markdown;
use crate::web::render::html_escape;

#[cfg(test)]
mod tests;

pub(super) fn render_markdown(source: &str) -> String {
    let mut output = String::new();
    let mut paragraph = Vec::new();
    let mut list_kind: Option<bool> = None;
    let mut code_language: Option<String> = None;

    for line in source.lines() {
        if code_language.is_some() {
            if line.starts_with("```") {
                output.push_str("</code></pre>");
                code_language = None;
            } else {
                output.push_str(&html_escape(line));
                output.push('\n');
            }
            continue;
        }
        if line.starts_with("```") {
            flush_markdown_paragraph(&mut paragraph, &mut output);
            close_markdown_list(&mut list_kind, &mut output);
            let language = line.trim_start_matches('`').trim();
            let language_class = if language.is_empty() {
                String::new()
            } else {
                format!(" class=\"language-{}\"", html_escape(language))
            };
            output.push_str(&format!("<pre><code{language_class}>"));
            code_language = Some(language.to_owned());
            continue;
        }
        if line.trim().is_empty() {
            flush_markdown_paragraph(&mut paragraph, &mut output);
            close_markdown_list(&mut list_kind, &mut output);
            continue;
        }
        if let Some((level, heading)) = markdown_heading(line) {
            flush_markdown_paragraph(&mut paragraph, &mut output);
            close_markdown_list(&mut list_kind, &mut output);
            let id = heading_slug(heading);
            output.push_str(&format!(
                "<h{level} id=\"{id}\">{}</h{level}>",
                inline_markdown(heading)
            ));
            continue;
        }
        if line.trim() == "---" || line.trim() == "***" {
            flush_markdown_paragraph(&mut paragraph, &mut output);
            close_markdown_list(&mut list_kind, &mut output);
            output.push_str("<hr>");
            continue;
        }
        if let Some(quote) = line.strip_prefix("> ") {
            flush_markdown_paragraph(&mut paragraph, &mut output);
            close_markdown_list(&mut list_kind, &mut output);
            output.push_str(&format!(
                "<blockquote><p>{}</p></blockquote>",
                inline_markdown(quote)
            ));
            continue;
        }
        if let Some(item) = line.strip_prefix("- ").or_else(|| line.strip_prefix("* ")) {
            flush_markdown_paragraph(&mut paragraph, &mut output);
            open_markdown_list(false, &mut list_kind, &mut output);
            output.push_str(&format!("<li>{}</li>", inline_markdown(item)));
            continue;
        }
        if let Some((_, item)) = line.split_once(". ").filter(|(number, _)| {
            !number.is_empty() && number.chars().all(|character| character.is_ascii_digit())
        }) {
            flush_markdown_paragraph(&mut paragraph, &mut output);
            open_markdown_list(true, &mut list_kind, &mut output);
            output.push_str(&format!("<li>{}</li>", inline_markdown(item)));
            continue;
        }
        close_markdown_list(&mut list_kind, &mut output);
        paragraph.push(line.trim().to_owned());
    }
    flush_markdown_paragraph(&mut paragraph, &mut output);
    close_markdown_list(&mut list_kind, &mut output);
    if code_language.is_some() {
        output.push_str("</code></pre>");
    }
    output
}

fn flush_markdown_paragraph(paragraph: &mut Vec<String>, output: &mut String) {
    if paragraph.is_empty() {
        return;
    }
    let text = paragraph.join(" ");
    output.push_str(&format!("<p>{}</p>", inline_markdown(&text)));
    paragraph.clear();
}

fn open_markdown_list(ordered: bool, current: &mut Option<bool>, output: &mut String) {
    if *current == Some(ordered) {
        return;
    }
    close_markdown_list(current, output);
    output.push_str(if ordered { "<ol>" } else { "<ul>" });
    *current = Some(ordered);
}

fn close_markdown_list(current: &mut Option<bool>, output: &mut String) {
    match current.take() {
        Some(true) => output.push_str("</ol>"),
        Some(false) => output.push_str("</ul>"),
        None => {}
    }
}

fn markdown_heading(line: &str) -> Option<(usize, &str)> {
    let hashes = line
        .chars()
        .take_while(|character| *character == '#')
        .count();
    (1..=6)
        .contains(&hashes)
        .then(|| line.get(hashes..).unwrap_or_default())
        .and_then(|heading| heading.strip_prefix(' ').map(|text| (hashes, text)))
}

fn heading_slug(heading: &str) -> String {
    let mut slug = String::new();
    let mut separator = false;
    for character in heading
        .chars()
        .flat_map(|character| character.to_lowercase())
    {
        if character.is_ascii_alphanumeric() {
            if separator && !slug.is_empty() {
                slug.push('-');
            }
            slug.push(character);
            separator = false;
        } else if character.is_whitespace() || character == '-' {
            separator = true;
        }
    }
    slug
}
