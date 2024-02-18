mod model;
use std::{
    collections::HashMap,
    env,
    io::Error as IoError,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use futures_channel::mpsc::{unbounded, UnboundedSender};
use futures_util::{future, pin_mut, stream::TryStreamExt, StreamExt};

use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::protocol::Message;

type Tx = UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;

async fn handle_connection(peer_map: PeerMap, raw_stream: TcpStream, addr: SocketAddr) {
    println!("Incoming TCP connection from: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake occurred");
    println!("WebSocket connection established with address: {}", addr);

    // Create a channel for our stream, which other sockets will use to send us messages.
    let (sender, reciever) = unbounded();
    peer_map.lock().unwrap().insert(addr, sender);

    let (outgoing, incoming) = ws_stream.split();

    // handle revcieved messages event
    let broadcast_incoming = incoming.try_for_each(|msg| {
        println!("Received a message from {}: {}", addr, msg.to_text().unwrap());
        let peers = peer_map.lock().unwrap();

        // We want to broadcast the message to everyone except ourselves.
        let broadcast_recipients =
            peers.iter().filter(|(peer_addr, _)| peer_addr != &&addr).map(|(_, ws_sink)| ws_sink);

        // TODO: broadcasting the message to all the peers now  
        // ! But we need to send the message to the matchEngine
        for recp in broadcast_recipients {
            println!("Sending message to {:?}", recp.clone());
            recp.unbounded_send(msg.clone()).unwrap();
        }

        future::ok(())


        // ! Remember we don't need the functionality to send message
    });

    let receive_from_others = reciever.map(Ok).forward(outgoing);

    pin_mut!(broadcast_incoming, receive_from_others);
    future::select(broadcast_incoming, receive_from_others).await;

    println!("{} disconnected", &addr);
    peer_map.lock().unwrap().remove(&addr);
}

#[tokio::main]
async fn main() -> Result<(), IoError> {
    let addr = env::args().nth(1).unwrap_or_else(|| "127.0.0.1:8080".to_string());

    let state = PeerMap::new(Mutex::new(HashMap::new()));

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", addr);

    // Let's spawn the handling of each connection in a separate task.
    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(state.clone(), stream, addr));
    }

    Ok(())
}

// taker / maker / mactchEngine should have a id.
// will have to create an enum to handle the different types of messages
// message from taker will go to the matchEngine
// message from maker will go to the matchEngine
// message from matchEngine will go to the taker or maker

#[derive(Debug, Clone, Copy)]
pub enum PayLoad{
    Subscribe,
    TakerMsg,
    MakerMsg,
    MatchEngineMsg
}

/*
    Subscribe payload{
        msg_type : "subscribe",
        name_of_sender : "Taker" / "maker" / "matchEngine",
        id : "id",
    }
*/


fn handle_message(payload: PayLoad, msg: String){
    match payload{
        PayLoad::Subscribe => {
            // TODO : Store the ip address   
            println!("Subscribe payload");
        },
        PayLoad::TakerMsg => {
            println!("TakerMsg payload");
        },
        PayLoad::MakerMsg => {
            println!("MakerMsg payload");
        },
        PayLoad::MatchEngineMsg => {
            println!("MatchEngineMsg payload");
        }
    }
}