use self::model::*;
use diesel::prelude::*; 
use diesel_demo::*;

fn main(){
    use self::schema::posts::dsl::*;

    let connection = &mut connect_db();
    let result = posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(connection)
        .expect("Error loading posts");


    println!("Displaying {} posts", result.len());

    for post in result{
        println!("{}", post.title);
        println!("-------------\n");
        println!("{}", post.body);
    }

}