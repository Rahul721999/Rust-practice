#![allow(unused)]
pub mod event_bus;
pub use event_bus::*;
pub mod websocket{
     use futures::{channel::mpsc::Sender, SinkExt, StreamExt};
     use reqwasm::websocket::{futures::WebSocket, Message};

     use wasm_bindgen_futures::spawn_local;

     pub struct WebSocketService{
          // channel transmitter
          pub tx : Sender<String>
     }

     impl WebSocketService{
          pub fn new() -> Self{
               let ws = WebSocket::open("ws://127.0.0.1:8080").unwrap();
               let (mut write, mut read) = ws.split();
               let (in_tx,mut in_rx) = futures::channel::mpsc::channel::<String>(1000);

               
               spawn_local(async move{
                    while let Some(s) = in_rx.next().await{
                         log::debug!("got event from channel! {}",s);
                         write.send(Message::Text(s)).await.unwrap();
                    }
               });


               // handle diff types of msges
               spawn_local(async move{
                    while let Some(msg) = read.next().await{
                         match msg{
                              Ok(Message::Text(data)) =>{
                                   log::debug!("from websocket: {}",data);
                              }
                              Ok(Message::Bytes(b)) => {
                                   let decoded = std::str::from_utf8(&b);
                                   if let Ok(val) = decoded {
                                       log::debug!("from websocket: {}", val);
                                   }
                               }
                               Err(e) => {
                                   log::error!("ws: {:?}", e)
                               }
                         }
                    }
                    log::debug!("WebSocket CLosed");
               });
               Self{tx: in_tx}
          }
     }
}