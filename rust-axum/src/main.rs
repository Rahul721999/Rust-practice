mod routes;
use routes::*;
use axum::{
    routing::get,
    Router,
};



#[tokio::main]
async fn main() {
    // create app
    let app = Router::new()
        .route("/",get(health_check))
        .route("/:name", get(greet))
        .route("/users", get(extract_user));

    // run with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().expect("failed to bind address"))
        .serve(app.into_make_service())
        .await
        .expect("failed to start server");
}