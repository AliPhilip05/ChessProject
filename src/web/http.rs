//! TCP listener and HTTP response serialization.
use super::router::route;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

#[cfg(test)]
mod tests;

pub(super) enum HttpResponse {
    Page(String),
    Download(String),
    Pdf(Vec<u8>),
    Redirect(String),
    NotFound,
}

pub fn serve() {
    let host = std::env::var("BIND_ADDRESS").unwrap_or_else(|_| "127.0.0.1".to_owned());
    let port = std::env::var("PORT").unwrap_or_else(|_| "7878".to_owned());
    let address = format!("{host}:{port}");
    let listener = match TcpListener::bind(&address) {
        Ok(listener) => listener,
        Err(error) => {
            eprintln!("Could not start the ChessProject web app: {error}");
            return;
        }
    };
    println!("ChessProject is listening at http://{address}");
    println!("Guidebook: http://{address}/guide");

    for incoming in listener.incoming() {
        match incoming {
            Ok(stream) => handle_connection(stream),
            Err(error) => eprintln!("Browser connection failed: {error}"),
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let _ = stream.set_read_timeout(Some(std::time::Duration::from_secs(3)));
    let mut buffer = [0u8; 16_384];
    let count = match stream.read(&mut buffer) {
        Ok(count) if count > 0 => count,
        _ => return,
    };
    let request = String::from_utf8_lossy(&buffer[..count]);
    let first_line = request.lines().next().unwrap_or_default();
    let mut parts = first_line.split_whitespace();
    let method = parts.next().unwrap_or("GET");
    let target = parts.next().unwrap_or("/");
    if method != "GET" {
        write_response(
            &mut stream,
            405,
            "text/plain; charset=utf-8",
            "Use a browser GET form for this prototype.",
        );
        return;
    }
    let response = route(target);
    match response {
        HttpResponse::Page(body) => {
            write_response(&mut stream, 200, "text/html; charset=utf-8", &body)
        }
        HttpResponse::Download(body) => write_download(&mut stream, "GTE-Guidebook.md", &body),
        HttpResponse::Pdf(body) => write_pdf(&mut stream, &body),
        HttpResponse::Redirect(location) => {
            let head = format!(
                "HTTP/1.1 303 See Other\r\nLocation: {location}\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
            );
            let _ = stream.write_all(head.as_bytes());
        }
        HttpResponse::NotFound => write_response(
            &mut stream,
            404,
            "text/plain; charset=utf-8",
            "Page not found.",
        ),
    }
}

fn write_response<W: Write>(stream: &mut W, status: u16, content_type: &str, body: &str) {
    let reason = match status {
        200 => "OK",
        404 => "Not Found",
        405 => "Method Not Allowed",
        _ => "Error",
    };
    let head = format!(
        "HTTP/1.1 {status} {reason}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\nX-Content-Type-Options: nosniff\r\n\r\n",
        body.len()
    );
    let _ = stream.write_all(head.as_bytes());
    let _ = stream.write_all(body.as_bytes());
}

fn write_download<W: Write>(stream: &mut W, filename: &str, body: &str) {
    let head = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/markdown; charset=utf-8\r\nContent-Disposition: attachment; filename=\"{filename}\"\r\nContent-Length: {}\r\nConnection: close\r\nX-Content-Type-Options: nosniff\r\n\r\n",
        body.len()
    );
    let _ = stream.write_all(head.as_bytes());
    let _ = stream.write_all(body.as_bytes());
}

fn write_pdf<W: Write>(stream: &mut W, body: &[u8]) {
    let head = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/pdf\r\nContent-Disposition: attachment; filename=\"GTE-Function-Appendix.pdf\"\r\nContent-Length: {}\r\nConnection: close\r\nX-Content-Type-Options: nosniff\r\n\r\n",
        body.len()
    );
    let _ = stream.write_all(head.as_bytes());
    let _ = stream.write_all(body);
}
