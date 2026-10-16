use std::io::{self, Write};

/// Raw mode disables the terminal's usual LF-to-CRLF conversion. Restore it
/// for rendered text so diagnostics and prompts always begin at column one.
pub(super) struct CrLfWriter<W> {
    inner: W,
    last_was_carriage_return: bool,
}

impl<W> CrLfWriter<W> {
    pub(super) fn new(inner: W) -> Self {
        Self {
            inner,
            last_was_carriage_return: false,
        }
    }
}

impl<W: Write> Write for CrLfWriter<W> {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        let mut normalized = Vec::with_capacity(bytes.len() + 8);
        for &byte in bytes {
            if byte == b'\n' && !self.last_was_carriage_return {
                normalized.push(b'\r');
            }
            normalized.push(byte);
            self.last_was_carriage_return = byte == b'\r';
        }
        self.inner.write_all(&normalized)?;
        Ok(bytes.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

#[cfg(test)]
mod tests {
    use super::CrLfWriter;
    use std::io::Write;

    #[test]
    fn normalizes_lines_without_duplicating_carriage_returns() {
        let mut writer = CrLfWriter::new(Vec::new());
        writer.write_all(b"first\nsecond\r\nthird").unwrap();
        assert_eq!(writer.inner, b"first\r\nsecond\r\nthird");
    }

    #[test]
    fn preserves_crlf_when_split_across_writes() {
        let mut writer = CrLfWriter::new(Vec::new());
        writer.write_all(b"diagnostic\r").unwrap();
        writer.write_all(b"\nnext\n").unwrap();
        assert_eq!(writer.inner, b"diagnostic\r\nnext\r\n");
    }
}
