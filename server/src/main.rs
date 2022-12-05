use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::{accept, Message};

fn main() {
    let addr = "127.0.0.1:9001";
    let server = match TcpListener::bind(addr) {
        Ok(w) => w,
        Err(e) => panic!("Error in binding TcpListener: {}", e),
    };

    println!("TcpListener bound at address: {}", addr);

    for stream in server.incoming() {
        // spawn a separate thread to handle each connection
        spawn(move || {
            let streamunwrap = stream.unwrap();
            let mut websocket = match accept(&streamunwrap) {
                Ok(o) => o,
                Err(e) => panic!("Failure in accepting stream: {}", e),
            };

            println!("Accepted stream: {}", streamunwrap.peer_addr().unwrap());
            loop {
                let msg = match websocket.read_message() {
                    Ok(o) => o,
                    Err(e) => {
                        println!("{e}");
                        break;
                    }
                };

                let format_string = format!("You've sent this: {}", msg);
                let return_message = Message::Text(format_string);
                if msg.is_binary() || msg.is_text() {
                    websocket.write_message(return_message).unwrap();
                }
            }
        });
    }
}
