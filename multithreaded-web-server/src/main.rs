use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};
fn main() {
    let listen = TcpListener::bind("127.0.0.1:8080").expect("failed to get the address");
    for stream in listen.incoming() {
        let _stream = stream.expect("failed to get the stream");
        std::thread::spawn(||{
            handle_req(_stream);
        });
    }
}

fn handle_req(mut stream: TcpStream) {
    let buf = BufReader::new(&mut stream);
    let http_req = buf.lines().next().expect("failed on first unwrap").expect("failed on sec unwrap");

    let (status, filename) = match &http_req [..]{
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            std::thread::sleep(std::time::Duration::from_secs(15));
            ("HTTP/1.1 200 OK", "hello.html")
        },
        _=> ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };  
    
        let content = fs::read_to_string(filename).expect("failed to get html file");
        let length = content.len();
        let response = format!("{status}\r\nContent-Length: {length}\r\n\r\n{content}");

        stream.write_all(response.as_bytes()).unwrap();
}