// 
pub async fn health_check() -> &'static str{
     "Health-Check Successful ✅"
}


//
//  ------------------------extractor------------------------
use axum::extract::{Path, Json};

/// 
/// this api takes a name in path and returns a String as greeting
pub async fn greet(Path(name) : Path<String>) ->String{
     format!("Hello {name}")
}
/// 
/// this api is the example to extract Json value from header
/// 

use serde::Deserialize;
#[derive(Deserialize)]
pub struct CreateUser {
    pub email: String,
    pub password: String,
}

pub async fn extract_user(Json(user) : Json<CreateUser>) -> String{
     format!("Email: {}, Password: {}", user.email, user.password)
}