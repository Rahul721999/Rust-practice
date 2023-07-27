use redis::RedisResult;
// use std::time::Duration;
fn main() -> RedisResult<()> {
    let client = redis::Client::open("redis://localhost:6379").unwrap();
    let mut con = client.get_connection()?;
    con.set_read_timeout(None)?;
    let res: () = redis::cmd("set").arg("my_key").arg(8).query(&mut con)?;
    let res: i32= redis::cmd("get").arg("my_key").query(&mut con)?;
    println!("{}",redis::Connection::check_connection(con));
    Ok(())
}
