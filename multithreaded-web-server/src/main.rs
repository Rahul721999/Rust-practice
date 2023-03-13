use std::{net::{TcpListener, TcpStream}, io::{BufReader, BufRead, Write}, fs};
fn main() {
    let listen = TcpListener::bind("127.0.0.1:8080").expect("failed to get the address");
    for stream in listen.incoming(){
        let _stream = stream.expect("failed to get the stream");
        handle_req(_stream);
    }
}

fn handle_req(mut stream: TcpStream){
    let buf = BufReader::new(&mut stream);
    let http_req : Vec<_>  = buf.lines().map(|l|l.unwrap())
                    .take_while(|line| !line.is_empty())
                    .collect();
    let status = "HTTP/1.1 200 OK";
    let content = fs::read_to_string("hello.html").expect("failed to get html file");
    let length = content.len();
    let response =
        format!("{status}\r\nContent-Length: {length}\r\n\r\n{content}");

    stream.write_all(response.as_bytes()).unwrap();
}