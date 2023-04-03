use diesel::{Queryable, Insertable};
use serde::{Serialize, Deserialize};
// use crate::diesel_db::schema::{todo_item, todo_list};
use crate::schema::{todo_item, todo_list};
#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct TodoItem{
    pub id : i32,
    pub title : String,
    pub checked : bool
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct TodoList{
    pub id : i32,
    pub title : String
}

#[derive(Debug, Insertable)]
#[diesel(table_name = todo_item)]
pub struct NewTodoItem<'a> {
    pub title : &'a str,
    pub checked : bool
}

#[derive(Debug, Insertable)]
#[diesel(table_name = todo_list)]
pub struct NewTodoList<'a>{
    pub title : &'a str
}