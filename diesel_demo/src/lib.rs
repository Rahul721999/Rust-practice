pub mod model;
pub mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn connect_db() ->PgConnection{
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("failed to get the db url");
    if let Ok(db) = PgConnection::establish(&db_url){
        return db
    }else {
        panic!("Error Connecting to {}", db_url);
    }
}


use self::model::{NewPost, Post};

pub fn create_post(conn : &mut PgConnection, title : &str, body: &str) -> Post{
    use crate::schema::posts;

    let new_post = NewPost {title, body};

    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error Saving new post")
}