use std::io::{self, Read, Write};
use std::net::TcpStream;

fn main() -> io::Result<()> {
    let mut buffer = [0; 1024];

    let mut stream = TcpStream::connect("127.0.0.1:8080")?;

    let message = b"Hello, server!";
    stream.write_all(message)?;

    let bytes_read = stream.read(&mut buffer)?;
    println!(
        "Received from server: {}",
        String::from_utf8_lossy(&buffer[..bytes_read])
    );

    Ok(())
}