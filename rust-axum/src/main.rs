mod routes;
use std::sync::Arc;

use routes::*;
use axum::{
    routing::get,
    Router,
};

#[derive(Clone)]
pub struct AppState{
    pub field1 : String,
}


#[tokio::main]
async fn main() {
    let state = Arc::new(AppState{
        field1 : "Hellow".to_string(),
    });
    let tuple = (state, Some("Rahul".to_string()));
    // create app
    let app = Router::new()
    .route("/",get(health_check))
    .route("/:name", get(greet))
    .route("/users", get(extract_user))
    .route("/tuple", get(extract_app_state_as_tuple))
    .with_state(tuple.clone());

    // run with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:8080".parse().expect("failed to bind address"))
        .serve(app.into_make_service())
        .await
        .expect("failed to start server");
}