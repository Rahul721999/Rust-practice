#![allow(unused)]
use actix_web::{HttpRequest, HttpResponse, web, Responder, Error};
use diesel::associations::HasTable;
use diesel::{RunQueryDsl, OptionalExtension};
use diesel::prelude::*;
use serde::Deserialize;
use crate::schema::todo_item::dsl::*;
use crate::models::{NewTodoItem, TodoItem};
use crate::run::PostgresPool;
#[derive(Debug, Deserialize)]
pub struct NewTask{
   pub title : String,
}
pub async fn add_task(db : web::Data<PostgresPool>, task : web::Json<NewTask>) -> Result<HttpResponse, Error>{
   println!("fn called");
   Ok(
      web::block(move|| add_task_res(db, task))
      .await
      .map(|task| HttpResponse::Ok().finish())
      .map_err(|_| HttpResponse::InternalServerError().finish()).expect("failed to get the error message")
   )   
}

fn add_task_res(db : web::Data<PostgresPool>, task : web::Json<NewTask>) -> Result<TodoItem, diesel::result::Error> {
   let mut db = db.get().expect("failed to get the db");
   let new_task = NewTodoItem{
      title : &task.title,
      checked : false,
   };
   let res = diesel::insert_into(todo_item).values(new_task).get_result::<TodoItem>(&mut db)?;
   Ok(res)
}

pub async fn health_check() -> HttpResponse{
   HttpResponse::Ok().finish()
}