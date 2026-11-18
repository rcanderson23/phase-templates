use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;

fn respond(mut stream: TcpStream) -> std::io::Result<()> {
    stream.set_read_timeout(Some(Duration::from_secs(5)))?;
    stream.set_write_timeout(Some(Duration::from_secs(5)))?;
    let mut request = [0_u8; 4096];
    if stream.read(&mut request)? == 0 {
        return Ok(());
    }

    let body = "Hello from a compiled Rust application Phase!\n";
    write!(
        stream,
        "HTTP/1.1 200 OK\r\nContent-Type: text/plain; charset=utf-8\r\n\
         Content-Length: {}\r\nConnection: close\r\n\r\n{}",
        body.len(),
        body
    )
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8080")?;
    println!("Listening on 0.0.0.0:8080");
    for connection in listener.incoming() {
        match connection.and_then(respond) {
            Ok(()) => {}
            Err(error) => eprintln!("Request failed: {error}"),
        }
    }
    Ok(())
}
