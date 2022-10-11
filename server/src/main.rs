use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::accept;

fn main() {
    let server = match TcpListener::bind("127.0.0.1:9001") {
        Ok(w) => w,
        Err(e) => panic!("Error in binding TcpListener: {}", e),
    };

    for stream in server.incoming() {
        // spawn a separate thread to handle each connection
        spawn(move || {
            let mut websocket = match accept(stream.unwrap()) {
                Ok(o) => o,
                Err(e) => panic!("Failure in accepting stream: {}", e),
            };

            loop {
                let msg = websocket.read_message().unwrap();

                if msg.is_binary() || msg.is_text() {
                    websocket.write_message(msg).unwrap();
                }
            }
        });
    }
}
