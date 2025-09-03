use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;

pub fn start_server(address: &str) -> std::io::Result<()> {
    let listener = TcpListener::bind(address)?;
    println!("🚀 Development server running on http://{}", address);
    println!("Press Ctrl+C to stop the server");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Err(e) = handle_connection(stream) {
                    eprintln!("Error handling connection: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer)?;

    let request = String::from_utf8_lossy(&buffer);
    let request_line = request.lines().next().unwrap_or("");

    // Parse the request path
    let path = request_line
        .split_whitespace()
        .nth(1)
        .unwrap_or("/")
        .trim_start_matches('/');

    println!("Request: {}", { path });

    let file_path = if path.is_empty() || path == "/" {
        "output/index.html"
    } else {
        &format!("output/{}", path)
    };

    let response = serve_file(file_path);
    stream.write_all(response.as_bytes())?;
    Ok(())
}

fn serve_file(path: &str) -> String {
    match fs::read_to_string(path) {
        Ok(contents) => {
            let content_type = get_content_type(path);
            format!(
                "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
                content_type,
                contents.len(),
                contents
            )
        }
        Err(_) => {
            let not_found = r#"<!DOCTYPE html>
<html>
<head><title>404 Not Found</title></head>
<body>
<h1>404 Not Found</h1>
<p>The requested page could not be found.</p>
<a href="/">← Back to Home</a>
</body>
</html>"#;
            format!(
                "HTTP/1.1 404 NOT FOUND\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
                not_found.len(),
                not_found
            )
        }
    }
}

fn get_content_type(path: &str) -> &'static str {
    let extension = Path::new(path)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");

    match extension {
        "html" => "text/html; charset=utf-8",
        "css" => "text/css",
        "js" => "application/javascript",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "svg" => "image/svg+xml",
        "ico" => "image/x-icon",
        _ => "text/plain",
    }
}
