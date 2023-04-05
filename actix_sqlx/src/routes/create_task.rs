use actix_web::{HttpResponse,web, Responder};
use sqlx::{PgPool};
use serde::{Serialize, Deserialize};

#[derive(Debug,Serialize, Deserialize)]
pub struct NewTask{
    pub title : String,
}

pub async fn add(db: web::Data<PgPool>,task : web::Json<NewTask>)-> impl Responder{
    let result = sqlx::query!(
        "INSERT INTO todos (title) VALUES ($1) RETURNING id",
        task.title,
    )
    .fetch_one(db.as_ref())
    .await;

    match result {
        Ok(row) => HttpResponse::Created().json(row.id),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}