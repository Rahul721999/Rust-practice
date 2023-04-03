extern crate diesel;

use actix_web::{HttpServer, App, web};
mod routes;
mod models;
mod schema;
mod run;
pub use routes::{add_task, health_check};
#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let db = run::connect_db();
    let db = web::Data::new(db.clone());
    dbg!("run fn called successfully");
    HttpServer::new(move||{
        App::new()
            .app_data(db.clone())
            .route("/add_task", web::post().to(add_task))
            .route("/health-check", web::get().to(health_check))

    })
    .bind(("127.0.0.1", 7000))?
    .run()
    .await
}
