use actix_web::{HttpResponse,web, Responder};
use sqlx::{FromRow, PgPool};

use serde::Serialize;

#[derive(Debug, FromRow, Serialize)]
struct Record {
    id: i32,
    title: String,
    checked : bool,
}

pub async fn read(db: web::Data<PgPool>)-> impl Responder{
    let result = sqlx::query_as!(
        Record,
        r#"SELECT id, title, checked FROM todos"#
    )
    .fetch_all(db.as_ref())
    .await;

    match result {
        Ok(rows) => HttpResponse::Ok().json(rows),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}   