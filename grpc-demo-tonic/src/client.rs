


// importing generated gRPC code
use hello_grpc::*;
// importing types for messages
use hello::*;
mod hello;
mod hello_grpc;


use grpc::ClientStubExt;
use futures::executor;
fn main() {

    let name = "Username";
    let port =50051;
    let client_conf = Default::default();
// create a client
    let client=SayClient::new_plain("::1", port, client_conf).unwrap();
// create request
    let mut req = SayRequest::new();
    req.set_name(name.to_string());
// send the request
    let resp = client
        .send(grpc::RequestOptions::new(), req)
        .join_metadata_result();
// wait for response
    println!("{:?}", executor::block_on(resp));
}
