use super::render_markdown;

#[test]
fn markdown_escapes_markup_and_keeps_code_literal() {
    let html = render_markdown("# API <tag>\n\n```rust\nlet raw = \"<script>\";\n```\n");
    assert!(html.contains("<h1 id=\"api-tag\">API &lt;tag&gt;</h1>"));
    assert!(html.contains("&lt;script&gt;"));
    assert!(!html.contains("<script>"));
    assert!(html.ends_with("</code></pre>"));
}

#[test]
fn documentation_links_are_rewritten_and_unsafe_schemes_are_removed() {
    let html = render_markdown(
        "[Walkthrough](reference.md#elo-scoring) and [unsafe](javascript:alert) and **bold**.",
    );
    assert!(html.contains("href=\"/guide/reference#elo-scoring\""));
    assert!(!html.contains("javascript:"));
    assert!(html.contains("<strong>bold</strong>"));
}
