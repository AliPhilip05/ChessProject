use super::{write_pdf, write_response};

#[test]
fn response_lengths_use_bytes_and_pdf_body_is_not_text_encoded() {
    let mut response = Vec::new();
    write_response(&mut response, 200, "text/plain; charset=utf-8", "♞");
    assert!(
        String::from_utf8(response)
            .unwrap()
            .contains("Content-Length: 3\r\n")
    );

    let payload = b"%PDF-\x00\xff";
    let mut response = Vec::new();
    write_pdf(&mut response, payload);
    let body_start = response
        .windows(4)
        .position(|part| part == b"\r\n\r\n")
        .unwrap()
        + 4;
    assert_eq!(&response[body_start..], payload);
    let headers = std::str::from_utf8(&response[..body_start]).unwrap();
    assert!(headers.contains("Content-Type: application/pdf\r\n"));
    assert!(headers.contains("Content-Disposition: attachment;"));
}
