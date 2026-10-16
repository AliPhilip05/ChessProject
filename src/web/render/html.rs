//! Shared app document shell and escaping for text inserted into HTML.
const STYLE: &str = include_str!("../../../assets/styles/app.css");

pub(in crate::web) fn page(title: &str, notice: &str, body: &str) -> String {
    format!(
        include_str!("../../../assets/templates/page.html"),
        html_escape(title),
        STYLE,
        notice,
        body
    )
}

pub(in crate::web) fn html_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
