mod model;
use std::io::Error as IoError;

use model::*;

#[tokio::main]
async fn main() -> Result<(), IoError> {
    match start_server().await {
        Ok(_) => println!("Server started successfully"),
        Err(e) => println!("Error starting the server: {}", e),
    };
    Ok(())
}

// taker / maker / mactchEngine should have a id.
// will have to create an enum to handle the different types of messages
// message from taker will go to the matchEngine
// message from maker will go to the matchEngine
// message from matchEngine will go to the taker or maker

/*
    Subscribe payload{
        msg_type : "subscribe",
        name_of_sender : "Taker" / "maker" / "matchEngine",
        id : "id",
    }
*/
