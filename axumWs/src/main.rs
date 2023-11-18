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
    
    todo!()
}