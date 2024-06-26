use tungstenite::{
    connect,
    protocol::{frame::coding::CloseCode::*, CloseFrame},
    Message,
};
use url::Url;

fn main() {
    env_logger::init();

    let (mut socket, response) =
        connect(Url::parse("ws://localhost:3012/socket").unwrap()).expect("Can't connect");

    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());
    println!("Response contains the following headers:");
    for (ref header, _value) in response.headers() {
        println!("* {}", header);
    }

    socket
        .send(Message::Text("Hello WebSocket".into()))
        .unwrap();

    let msg = socket.read().expect("Error reading message");
    println!("Received: {:?}", msg);
    

    socket.send(Message::Text("get_data".into())).unwrap();

    let msg = socket.read().expect("Error reading message");
    if msg.is_text() {
        println!("Data: {}", msg.into_text().unwrap_or_default());
    }

    socket.send(Message::Text("Bye WebSocket".into())).unwrap();

    socket
        .close(Some(CloseFrame {
            code: Normal,
            reason: "End".into(),
        }))
        .expect("Error while closing connection.");
}
