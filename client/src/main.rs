use tungstenite::{connect, Message};
use url::Url;

fn main() {
    let (mut socket, _) =
        connect(Url::parse("ws://localhost:9001").unwrap()).expect("Can't connect to socket");

    println!("Connected to socket");

    socket
        .write_message(Message::Text("Hello dumbass socket".into()))
        .unwrap();

    let msg = socket.read_message().expect("Error reading message");
    println!("Received: {}", msg);
}
