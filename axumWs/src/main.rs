use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
};
use axum::TypedHeader; 

use std::borrow::Cow;
use std::ops::ControlFlow;
use std::{net::SocketAddr, path::PathBuf};
use tower_http::{
    services::ServeDir,
    trace::{DefaultMakeSpan, TraceLayer},
};

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

//allows to extract the IP of connecting user
use axum::extract::connect_info::ConnectInfo;
use axum::extract::ws::CloseFrame;

//allows to split the websocket stream into separate TX and RX branches
use futures::{sink::SinkExt, stream::StreamExt};

#[tokio::main]
async fn main() {
    // configure tracing_subscriber
    tracing_subscriber::registry()
        .with( // with layer
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_websocket=debug, tower_http=debug".into()),
        ).with(tracing_subscriber::fmt::layer())
        .init();

        let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");

        // App route
        let app = Router::new()
            .fallback_service(ServeDir::new(assets_dir).append_index_html_on_directories(true))
            .route("/ws", get(websocket_handler))
            .layer(TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true))
            );

        let listener = tokio::net::TcpListener::bind("localhost:3000")
            .await
            .unwrap();

        tracing::debug!("🚀 listening on: {}", listener.local_addr().unwrap());
        axum::Server::bind(&listener.local_addr().unwrap()).serve(app.into_make_service()).await.unwrap();
}

async fn websocket_handler(
    ws: WebSocketUpgrade,
    user_agent : Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(socket_addr) : ConnectInfo<SocketAddr>,
)-> impl IntoResponse{
    // get the browser info
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent{
        user_agent.to_string()
    }else{
        String::from("Unknown browser")
    };

    println!("🛜`{user_agent}` at: {socket_addr} connected");

    ws.on_upgrade(move |socket| handle_socket(socket, socket_addr))
}

async fn handle_socket(
    mut socket : WebSocket,
    who : SocketAddr
){

    // send msg
    if socket.send(Message::Ping(vec![1,2,3])).await.is_ok(){
    }else{
        println!("❌Could not send ping to: {who} ");
        return;
    }

    // recieving msg
    if let Some(msg) = socket.recv().await{
        if let Ok(msg) = msg{
            // TODO: do something with the msg
            if process_msg(msg, who).is_break(){
                return;
            }
        }else{
            println!("❌Client:{who} got disconnected");
            return
        }
    }
    
    // Sending random messages
    for i in 1..5 {
        if socket
            .send(Message::Text(format!("Hi {i} times!")))
            .await
            .is_err()
        {
            println!("client {who} abruptly disconnected");
            return;
        }
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }

    // To send and recieve the messages at the same time. Split the websocket channel to sender and reciever
    let (mut sender, mut reciever) = socket.split();


    // Spawn a task that will push messages to the client
    let mut send_task = tokio::spawn(async move{
        let n_msg = 10;
        for i in 0..n_msg{
            if sender.send(Message::Text(format!("Server message: {i}...")))
                .await
                .is_err(){
                    return i;
                }
                tokio::time::sleep(std::time::Duration::from_millis(300)).await;
        }

        // sending CLOSE req to websocket
        println!("🚀Sending close to {who}...");
        if let Err(e) = sender
            .send(Message::Close(Some(CloseFrame { 
                code: axum::extract::ws::close_code::NORMAL,
                reason: Cow::from("GoodBye !!!") 
            }
        ))).await{
            println!("Could not send CLOSE req, Reason : {e}");
        }
        n_msg
    });
    

    // Spawn a task that will recieve the messages and print them on the server console.
    let mut rcv_task = tokio::spawn(async move{
        let mut cnt = 0;
        while let Some(Ok(msg)) = reciever.next().await{
            cnt +=1;
            if process_msg(msg, who).is_break(){
                break;
            }
        }
        cnt
    });

    // If any one of the task EXITS, abort the other.
    tokio::select! {
        rv_a = (&mut send_task) =>{
            match rv_a{
                Ok(a) => println!("{a} Messages sent to {who}"),
                Err(a) => println!("Error sending mesasages, Reason: {a}"),
            }
            rcv_task.abort();
        }

        rv_b = (&mut rcv_task) =>{
            match rv_b{
                Ok(b) => println!("Recieved {b} messages"),
                Err(b) => println!("Error recieving messages, Reason: {b}"),
            }
            send_task.abort()
        }
    }

    println!("❌ Websocket context {who} destroyed");
}

// handler fn to process the ricieved msg
fn process_msg(msg: Message, who : SocketAddr) -> ControlFlow<(),()>{
    // TODO: process the recieved msg
    match msg{
        Message::Ping(v) =>{
            println!(" 🚩 {who} sent a ping with {v:?}");
        }
        Message::Pong(v)=>{
            println!("🚩 {who} sent a pong with: {v:?}");
        }
        Message::Text(t) =>{
            println!("🚩 {who} sent a pong with: {t:?}");
        }
        Message::Binary(d) =>{
            println!("🚩 {who} sent {} bytes: {:?}", d.len(),d);
        }
        Message::Close(c) =>{
            if let Some(cf) = c {
                println!("{} send close with code {} and reason {}", who, cf.code, cf.reason);
            }else{
                println!("{} somehow sent clsoe message without CLoseFrame", who);
            }
            return ControlFlow::Break(());
        }
    }
    ControlFlow::Continue(())
}