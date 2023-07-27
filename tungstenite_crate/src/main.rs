use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::{accept, Message};

/// A WebSocket echo server
fn main() {
    let server = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in server.incoming() {
        spawn(move || {
            let mut websocket = accept(stream.unwrap()).unwrap();
            loop {
                let msg = websocket.read().unwrap();
                // We do not want to send back ping/pong messages.
                println!("{}",msg);
                if msg.is_binary() || msg.is_text() {
                    if msg.to_string().trim() == "hello" {
                        println!("triggered");
                        websocket.send(Message::Text("how are you".to_string())).unwrap();
                    } else {
                        websocket.send(msg).unwrap();
                    }
                }
            }
        });
    }
}
