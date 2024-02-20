use std::{
    collections::HashMap,
    env,
    io::Error as IoError,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use futures_channel::mpsc::{unbounded, UnboundedReceiver, UnboundedSender};
use futures_util::{
    future, pin_mut,
    stream::{SplitSink, SplitStream, TryStreamExt},
    StreamExt,
};

use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::WebSocketStream;

type Tx = UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;
type ServiceToId = Arc<Mutex<HashMap<String, SocketAddr>>>;

pub async fn start_server() -> Result<(), IoError> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());
    let state = PeerMap::new(Mutex::new(HashMap::new()));

    let service_to_id = ServiceToId::new(Mutex::new(HashMap::new()));

    if let Ok(listener) = TcpListener::bind(&addr).await {
        println!("Listening on: {}", addr);

        // Let's spawn the handling of each connection in a separate task.
        while let Ok((stream, addr)) = listener.accept().await {
            tokio::spawn(handle_connection(
                state.clone(),
                service_to_id.clone(),
                stream,
                addr,
            ));
        }
        Ok(())
    } else {
        println!("Failed to bind address");
        Err(IoError::new(
            std::io::ErrorKind::Other,
            "Failed to bind address",
        ))
    }
}

async fn handle_connection(
    peer_map: PeerMap,
    registry: ServiceToId,
    stream: TcpStream,
    addr: SocketAddr,
) {
    println!("Incoming TCP connection from: {}", addr);

    // Accept the websocket handshake
    if let Ok(ws_stream) = tokio_tungstenite::accept_async(stream).await {
        println!("WebSocket connection established with address: {}", addr);

        // create a channel for our stream, which other sockets will use to send us messages
        let (sender_channel, reciever_channel) = unbounded();
        peer_map.lock().unwrap().insert(addr, sender_channel);

        let (outgoing_ws, incoming_ws) = ws_stream.split();

        handle_incoming_messages(
            incoming_ws,
            reciever_channel,
            outgoing_ws,
            peer_map.clone(),
            registry.clone(),
            addr,
        )
        .await;
    } else {
        println!("Error during the websocket handshake occurred");
    }
}

async fn handle_incoming_messages(
    incoming: SplitStream<WebSocketStream<TcpStream>>,
    reciever: UnboundedReceiver<Message>,
    outgoing_ws: SplitSink<WebSocketStream<TcpStream>, Message>,
    peer_map: PeerMap,
    registry: ServiceToId,
    addr: SocketAddr,
) {
    // handle revcieved messages event
    let broadcast_incoming = incoming.try_for_each(|msg| {
        if let Ok(peers) = peer_map.lock() {
            // TODO: Use peers to handle messages
            handle_message(msg, registry.clone(), addr);
        } else {
            println!("Error in getting the peer map");
        }
        future::ok(())
    });
    let receive_from_others = reciever.map(Ok).forward(outgoing_ws);

    pin_mut!(broadcast_incoming, receive_from_others);
    future::select(broadcast_incoming, receive_from_others).await;

    println!("{} disconnected", &addr);
    peer_map.lock().unwrap().remove(&addr);
}

fn handle_message(payload: Message, registry: ServiceToId, addr: SocketAddr) {
    println!("Received a message from {}: {}", addr, payload.to_text().unwrap()); 
    // if let Ok(payload){
    //     match payload {
    //         PayLoad::Subscribe => {
    //             println!("Subscribe payload");
    //         }
    //         PayLoad::TakerMsg => {
    //             println!("TakerMsg payload");
    //         }
    //         PayLoad::MakerMsg => {
    //             println!("MakerMsg payload");
    //         }
    //         PayLoad::MatchEngineMsg => {
    //             println!("MatchEngineMsg payload");
    //         }
    //     }
    // } else {
    //     println!("payload is not a valid text message");
    // }
}

fn get_reciever(reciever: String, registry: ServiceToId){

}


#[derive(Debug, Clone)]
pub enum PayLoad {
    Subscribe(SubscriptionPayload),
    TakerMsg(TakerPayload),
    MakerMsg(MakerPayload),
    MatchEngineMsg(MatchEnginePayload),
}

#[derive(Debug, Clone)]
pub struct SubscriptionPayload {
    pub service: String,
    pub id: String,
}

#[derive(Debug, Clone)]
pub struct TakerPayload {
    pub id: String,
    pub price: f64,
    pub quantity: f64,
    pub currency: String
}

#[derive(Debug, Clone)]
pub struct MakerPayload {
    pub id: String,
    pub price: f64,
    pub quantity: f64,
    pub currency: String
}

#[derive(Debug, Clone)]
pub struct MatchEnginePayload {
    pub order_status: String,
    pub order_id : String,
    pub price: f64,
    pub quantity: f64,
}
