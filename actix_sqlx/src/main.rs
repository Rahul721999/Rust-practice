#![allow(unused)]
use actix_web::{HttpServer, web::{self, Data}, App};
use dotenv::dotenv;
use lib::{add, read, update};
use lib::run::connect_db;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db = connect_db().await;
    let db = Data::new(db);
    HttpServer::new(move ||{
        App::new()
            .service(
                web::scope("/todo")
                    .app_data(db.clone())
                    .route("add-task", web::post().to(add))
                    .route("read-task", web::get().to(read))
                    .route("update-task", web::get().to(update))
            )
    })
    .bind(("127.0.0.1", 7000))?
    .run()
    .await
}
