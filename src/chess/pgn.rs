//! PGN headers, mainline tokenization, and move-number handling.

use std::collections::HashMap;

pub fn pgn_headers(pgn: &str) -> HashMap<String, String> {
    let mut result = HashMap::new();
    for line in pgn.lines() {
        let line = line.trim();
        if !line.starts_with('[') || !line.ends_with(']') {
            continue;
        }
        let Some(space) = line.find(char::is_whitespace) else {
            continue;
        };
        let key = &line[1..space];
        let value = line[space..].trim().trim_end_matches(']').trim();
        let value = value
            .strip_prefix('"')
            .and_then(|text| text.strip_suffix('"'))
            .unwrap_or(value)
            .replace("\\\"", "\"");
        result.insert(key.to_owned(), value);
    }
    result
}

pub fn pgn_move_tokens(pgn: &str) -> Vec<String> {
    let mut body = String::new();
    let mut in_braces = false;
    let mut variation_depth = 0usize;
    let mut in_line_comment = false;

    for line in pgn.lines() {
        if line.trim().starts_with('[') {
            body.push(' ');
            continue;
        }
        for ch in line.chars() {
            if in_line_comment {
                continue;
            }
            if in_braces {
                if ch == '}' {
                    in_braces = false;
                    body.push(' ');
                }
                continue;
            }
            match ch {
                ';' if variation_depth == 0 => in_line_comment = true,
                '{' if variation_depth == 0 => in_braces = true,
                '(' => variation_depth += 1,
                ')' if variation_depth > 0 => variation_depth -= 1,
                _ if variation_depth == 0 => body.push(ch),
                _ => {}
            }
        }
        in_line_comment = false;
        body.push(' ');
    }

    body.split_whitespace().map(str::to_owned).collect()
}

pub(super) fn header<'a>(headers: &'a HashMap<String, String>, key: &str) -> Option<&'a str> {
    headers.get(key).map(String::as_str)
}

pub(super) fn parse_rating(text: &str) -> Option<i32> {
    text.parse::<i32>().ok().filter(|rating| *rating > 0)
}

pub(super) fn is_result(token: &str) -> bool {
    matches!(token, "1-0" | "0-1" | "1/2-1/2" | "*")
}

pub(super) fn strip_move_number(token: &str) -> String {
    let mut index = 0;
    for byte in token.bytes() {
        if byte.is_ascii_digit() || byte == b'.' {
            index += 1;
        } else {
            break;
        }
    }
    token[index..].to_owned()
}

#[cfg(test)]
mod tests {
    use super::{pgn_headers, pgn_move_tokens, strip_move_number};

    #[test]
    fn mainline_tokens_exclude_headers_comments_and_nested_variations() {
        let pgn = r#"[White "Player"]
1.e4 {a comment} e5 (1... c5 (2. Nf3)) 2.Nf3 ; ignore this line
Nc6 1-0"#;

        assert_eq!(pgn_move_tokens(pgn), ["1.e4", "e5", "2.Nf3", "Nc6", "1-0"]);
        assert_eq!(strip_move_number("1...e5"), "e5");
    }

    #[test]
    fn headers_decode_escaped_quotes() {
        let headers = pgn_headers(r#"[Event "A \"friendly\" game"]"#);
        assert_eq!(headers.get("Event").unwrap(), "A \"friendly\" game");
    }
}
