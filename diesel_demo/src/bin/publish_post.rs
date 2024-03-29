use self::model::Post;
use diesel::prelude::*;
use diesel_demo::*;
use std::env::args;

fn main(){
    use self::schema::posts::dsl::{posts, published};

    let id = args()
        .nth(1)
        .expect("publish_post requires a post id")
        .parse::<i32>()
        .expect("Invalid ID");
    let connection = &mut connect_db();

    let post = diesel::update(posts.find(id))
        .set(published.eq(true))
        .get_result::<Post>(connection)
        .unwrap();
    println!("Published post {}", post.title);

}