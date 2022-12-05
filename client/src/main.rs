use tungstenite::{connect, Message};
use url::Url;

fn main() {
    let (mut socket, _) =
        connect(Url::parse("ws://localhost:9001").unwrap()).expect("Can't connect to socket");

    println!("Connected to socket");

    let message = "Hello dumbass socket";

    println!("Sending message \"{}\"", message);
    socket
        .write_message(Message::Text(message.to_string()))
        .unwrap();

    let msg = socket.read_message().expect("Error reading message");
    println!("Received: \"{}\"", msg);

    match socket.close(None) {
        Ok(_) => (),                 //ok
        Err(e) => println!("{}", e), // quite unusual I'd assume?
    };
}
