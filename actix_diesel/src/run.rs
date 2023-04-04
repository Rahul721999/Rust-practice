use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
use std::{env};
use r2d2::Pool;

pub type PostgresPool = Pool<ConnectionManager<PgConnection>>;

pub fn connect_db() -> PostgresPool{
    dotenv().ok();
    dbg!("run fn called");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    dbg!("got db url");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    dbg!("trying to connect");
    r2d2::Pool::builder()
        .build(manager).expect("failed to get the db pool")
    
}