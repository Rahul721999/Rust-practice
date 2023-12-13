#![allow(unused)]

// Import the event_bus module
pub mod event_bus;
pub use event_bus::*;

pub mod websocket {
    // Import necessary dependencies
    use futures::{channel::mpsc::Sender, SinkExt, StreamExt};
    use reqwasm::websocket::{futures::WebSocket, Message}; // Assuming reqwasm and wasm_bindgen_futures are correctly imported
    use wasm_bindgen_futures::spawn_local;

    // Define the WebSocketService struct
    pub struct WebSocketService {
        // Channel transmitter for sending messages to the WebSocket
        pub tx: Sender<String>,
    }

    // Implement methods for WebSocketService
    impl WebSocketService {
        // Constructor for creating a new WebSocketService
        pub fn new() -> Self {
            // Open a WebSocket connection to "ws://localhost:8080/"
            let ws = WebSocket::open("ws://localhost:8080")
                .expect("failed to create websocket");
            let (mut write, mut read) = ws.split();

            // Create a channel for sending messages to the WebSocket
            let (in_tx, mut in_rx) = futures::channel::mpsc::channel::<String>(1000);

            // Spawn a task to handle sending messages to the WebSocket
            spawn_local(async move {
                while let Some(s) = in_rx.next().await {
                    log::debug!("got event from channel! {}", s);
                    if let Err(err) = write.send(Message::Text(s)).await{
                        log::error!("Error sending message via WebSocket: {:?}", err);
                    }
                }
            });

            // Spawn a task to handle receiving messages from the WebSocket
            spawn_local(async move {
                while let Some(msg) = read.next().await {
                    match msg {
                        Ok(Message::Text(data)) => {
                            log::debug!("from websocket: {}", data);
                        }
                        Ok(Message::Bytes(b)) => {
                            if let Ok(val) = std::str::from_utf8(&b) {
                                log::debug!("from websocket: {}", val);
                            }
                        }
                        Err(e) => {
                            log::error!("ws: {:?}", e);
                        }
                    }
                }
                log::debug!("WebSocket Closed");
            });

            Self { tx: in_tx }
        }
    }
}
