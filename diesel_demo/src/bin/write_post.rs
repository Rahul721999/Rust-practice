use diesel_demo::*;
use std::io::{stdin, Read};

#[cfg(not(windows))]
const EOF : &str = "CTRL+D";
#[cfg(windows)]
const EOF : &str = "CTRL+Z";

fn main() {
    let connection = &mut connect_db();

    let mut title = String::new();
    let mut body = String::new();

    println!("Enter a Title: ");
    stdin().read_line(&mut title).expect("Failed to get the title from CLI");
    let title = title.trim();

    println!(
        "\nOk! Let's write {} (Press {} when finished)\n",
        title, EOF
    );
    stdin().read_to_string(&mut body).unwrap();
    
    let post = create_post(connection, title, &body);

    println!("Saved draft {} with id {}",title, post.id);

}