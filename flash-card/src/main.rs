//! Static web server for the flash-card app — std only, no dependencies.
//!
//! Serves the `web/` directory plus the generated `data/cards.json` on
//! `http://127.0.0.1:8080`:
//!
//! - `GET /`                 -> `web/index.html`
//! - `GET /styles.css`, etc. -> the matching file in `web/`
//! - `GET /data/cards.json`  -> the generated deck (as `application/json`)
//! - anything else           -> `404 Not Found`
//!
//! Run from the `flash-card/` directory with `cargo run` (generate the deck
//! first with `cargo run --bin generate`).

use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};

const DEFAULT_ADDR: &str = "127.0.0.1:7697";

fn main() {
    // Anchor served paths to this crate's directory so `cargo run` works
    // regardless of the current working directory.
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let web_dir = manifest_dir.join("web");
    let data_dir = manifest_dir.join("data");

    let addr = bind_addr();
    let listener = match TcpListener::bind(&addr) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("error: could not bind {addr}: {e}");
            eprintln!("hint: the port may be in use — try `cargo run -- 7698` or set FLASH_CARD_ADDR.");
            std::process::exit(1);
        }
    };

    println!("Flash-cards served at http://{addr}  (Ctrl+C to stop)");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let web = web_dir.clone();
                let data = data_dir.clone();
                // One thread per connection keeps the std-only server simple.
                std::thread::spawn(move || {
                    if let Err(e) = handle(stream, &web, &data) {
                        eprintln!("connection error: {e}");
                    }
                });
            }
            Err(e) => eprintln!("accept error: {e}"),
        }
    }
}

/// Pick the bind address: a CLI argument, then `FLASH_CARD_ADDR`, then the
/// default `127.0.0.1:8080`. A bare port (e.g. `8090`) binds to localhost.
fn bind_addr() -> String {
    std::env::args()
        .nth(1)
        .or_else(|| std::env::var("FLASH_CARD_ADDR").ok())
        .map(|s| {
            if s.contains(':') {
                s
            } else {
                format!("127.0.0.1:{s}")
            }
        })
        .unwrap_or_else(|| DEFAULT_ADDR.to_string())
}

/// Read one request and write the response.
fn handle(mut stream: TcpStream, web_dir: &Path, data_dir: &Path) -> std::io::Result<()> {
    let mut reader = BufReader::new(stream.try_clone()?);

    let mut request_line = String::new();
    if reader.read_line(&mut request_line)? == 0 {
        return Ok(()); // client hung up before sending anything
    }

    // Drain the remaining request headers so the client sees a clean response.
    let mut header = String::new();
    loop {
        header.clear();
        let n = reader.read_line(&mut header)?;
        if n == 0 || header == "\r\n" || header == "\n" {
            break;
        }
    }

    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap_or("");
    let target = parts.next().unwrap_or("/");

    if method != "GET" {
        return send(
            &mut stream,
            "405 Method Not Allowed",
            "text/plain; charset=utf-8",
            b"405 Method Not Allowed",
        );
    }

    match resolve(target, web_dir, data_dir) {
        Some(path) => match std::fs::read(&path) {
            Ok(body) => send(&mut stream, "200 OK", content_type(&path), &body),
            Err(_) => not_found(&mut stream),
        },
        None => not_found(&mut stream),
    }
}

/// Map a request target to a file on disk, or `None` for a 404.
///
/// `/` serves `web/index.html`; `/data/cards.json` serves the generated deck;
/// everything else is looked up under `web/`. Path traversal is rejected and,
/// as defense in depth, the resolved path is confined to its base directory.
fn resolve(target: &str, web_dir: &Path, data_dir: &Path) -> Option<PathBuf> {
    let path = target.split(['?', '#']).next().unwrap_or(target);
    let rel = path.trim_start_matches('/');
    let rel = if rel.is_empty() { "index.html" } else { rel };

    // Reject empty/`.`/`..` segments outright.
    if rel
        .split('/')
        .any(|seg| seg.is_empty() || seg == "." || seg == "..")
    {
        return None;
    }

    let (base, candidate) = if rel == "data/cards.json" {
        (data_dir, data_dir.join("cards.json"))
    } else {
        (web_dir, web_dir.join(rel))
    };

    // Confirm the resolved file stays within its base directory.
    let real_base = base.canonicalize().ok()?;
    let real_path = candidate.canonicalize().ok()?;
    real_path.starts_with(&real_base).then_some(real_path)
}

/// Guess a content type from the file extension.
fn content_type(path: &Path) -> &'static str {
    match path.extension().and_then(|e| e.to_str()) {
        Some("html") => "text/html; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("js") => "text/javascript; charset=utf-8",
        Some("json") => "application/json; charset=utf-8",
        Some("svg") => "image/svg+xml",
        Some("ico") => "image/x-icon",
        Some("png") => "image/png",
        _ => "application/octet-stream",
    }
}

fn not_found(stream: &mut TcpStream) -> std::io::Result<()> {
    send(
        stream,
        "404 Not Found",
        "text/html; charset=utf-8",
        b"<!doctype html><meta charset=utf-8><h1>404 Not Found</h1>",
    )
}

/// Write a complete HTTP/1.1 response and close the connection.
fn send(
    stream: &mut TcpStream,
    status: &str,
    content_type: &str,
    body: &[u8],
) -> std::io::Result<()> {
    let head = format!(
        "HTTP/1.1 {status}\r\n\
         Content-Type: {content_type}\r\n\
         Content-Length: {len}\r\n\
         Connection: close\r\n\
         \r\n",
        len = body.len(),
    );
    stream.write_all(head.as_bytes())?;
    stream.write_all(body)?;
    stream.flush()
}
