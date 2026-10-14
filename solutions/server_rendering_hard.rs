pub fn render_notice(title: &str, message: &str) -> String {
    fn escape(text: &str) -> String {
        let mut escaped = String::new();
        for character in text.chars() {
            match character {
                '&' => escaped.push_str("&amp;"),
                '<' => escaped.push_str("&lt;"),
                '>' => escaped.push_str("&gt;"),
                '"' => escaped.push_str("&quot;"),
                '\'' => escaped.push_str("&#39;"),
                other => escaped.push(other),
            }
        }
        escaped
    }
    format!("<aside><strong>{}</strong><p>{}</p></aside>", escape(title), escape(message))
}
