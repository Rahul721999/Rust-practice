use diesel::prelude::*;
use diesel_demo::*;
use std::env::args;


fn main(){
    use self::schema::posts::dsl::*;

    let target = args().nth(1).expect("Expected a target to match against");
    let pattern = format!("%{}%",target);

    let connection = &mut connect_db();
    let num_deleted = diesel::delete(posts.filter(title.like(pattern)))
        .execute(connection)
        .expect("Error deleting post");

    println!("Deleted {} posts", num_deleted)
}