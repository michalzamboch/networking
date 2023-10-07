use serde::{Deserialize, Serialize};
use std::io::{self, Read, Write};
use std::net::TcpStream;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug)]
struct Message {
    uuid: Uuid,
    msg: Option<String>,
}

fn get_mock_message() -> Message {
    Message {
        uuid: Uuid::new_v4(),
        msg: Some("Hi from clinet: ".to_owned()),
    }
}

fn main() -> io::Result<()> {
    let mut buffer = [0; 1024];
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;

    let serialized = serde_json::to_vec(&get_mock_message()).unwrap();
    stream.write_all(&serialized)?;

    let bytes_read = stream.read(&mut buffer)?;
    println!(
        "Received from server: {}",
        String::from_utf8_lossy(&buffer[..bytes_read])
    );

    Ok(())
}
