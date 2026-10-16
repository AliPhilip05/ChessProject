//! Inline Markdown formatting and validated documentation links.
use crate::web::render::html_escape;

pub(super) fn inline_markdown(text: &str) -> String {
    let mut output = String::new();
    let mut index = 0;
    while index < text.len() {
        let tail = &text[index..];
        if let Some(code) = tail.strip_prefix('`') {
            if let Some(end) = code.find('`') {
                output.push_str(&format!("<code>{}</code>", html_escape(&code[..end])));
                index += end + 2;
                continue;
            }
        }
        if let Some(strong) = tail.strip_prefix("**") {
            if let Some(end) = strong.find("**") {
                output.push_str(&format!(
                    "<strong>{}</strong>",
                    inline_markdown(&strong[..end])
                ));
                index += end + 4;
                continue;
            }
        }
        if let Some(link) = tail.strip_prefix('[') {
            if let Some(label_end) = link.find("](") {
                let target_start = label_end + 2;
                if let Some(target_end) = link[target_start..].find(')') {
                    let label = &link[..label_end];
                    let target = &link[target_start..target_start + target_end];
                    if let Some(href) = safe_markdown_href(target) {
                        output.push_str(&format!(
                            "<a href=\"{}\">{}</a>",
                            html_escape(&href),
                            inline_markdown(label)
                        ));
                    } else {
                        output.push_str(&inline_markdown(label));
                    }
                    index += target_start + target_end + 2;
                    continue;
                }
            }
        }
        let character = tail.chars().next().expect("index is before the end");
        output.push_str(&html_escape(&character.to_string()));
        index += character.len_utf8();
    }
    output
}

fn safe_markdown_href(target: &str) -> Option<String> {
    if target.starts_with("https://")
        || target.starts_with("http://")
        || target.starts_with("/guide")
        || target.starts_with('#')
    {
        Some(target.to_owned())
    } else if let Some(fragment) = target.strip_prefix("reference.md") {
        Some(format!("/guide/reference{fragment}"))
    } else {
        None
    }
}
