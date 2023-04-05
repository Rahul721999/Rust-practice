use actix_web::{HttpResponse,web, Responder};
use sqlx::{PgPool};
use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct Task {
    id: i32,
    title: String,
    completed: bool,
}
#[derive(Deserialize)]
pub struct GetTask{
    pub title : String,
}
pub async fn update(db : web::Data<PgPool>, get_task : web::Json<GetTask>)-> impl Responder{
    match sqlx::query!(
        "UPDATE todos SET checked = $1 WHERE title = $2",
        true,
        get_task.title
    ).execute(db.as_ref()).await{
        Ok(_res) => {
            HttpResponse::Ok().finish()
        },
        Err(_) => HttpResponse::InternalServerError().finish(),
    }    
}