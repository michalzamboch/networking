use std::{net::TcpListener, thread::spawn};

use tungstenite::{
    accept_hdr,
    handshake::server::{Request, Response},
    Message,
};

fn main() {
    env_logger::init();
    let server = TcpListener::bind("127.0.0.1:3012").unwrap();
    for stream in server.incoming() {
        spawn(move || {
            let callback = |req: &Request, mut response: Response| {
                println!("Received a new ws handshake");
                println!("The request's path is: {}", req.uri().path());
                println!("The request's headers are:");
                for (ref header, _value) in req.headers() {
                    println!("* {}", header);
                }

                // Let's add an additional header to our response to the client.
                let headers = response.headers_mut();
                headers.append("MyCustomHeader", ":)".parse().unwrap());
                headers.append("SOME_TUNGSTENITE_HEADER", "header_value".parse().unwrap());

                Ok(response)
            };
            let mut websocket = accept_hdr(stream.unwrap(), callback).unwrap();

            websocket
                .send(Message::Text("Hello from server.".into()))
                .unwrap();

            loop {
                let msg = websocket.read().unwrap();

                if msg.is_binary() {
                    println!(
                        "Received and sending forward: {}",
                        msg.clone().into_text().unwrap_or_default()
                    );
                } else if msg.is_text() {
                    let text = msg.clone().into_text().unwrap_or_default();
                    if text == "get_data" {
                        websocket
                            .send(Message::Text("Data from server.".into()))
                            .unwrap();
                    }
                } else if msg.is_close() {
                    println!("Bye...");
                    break;
                }
            }
        });
    }
}
