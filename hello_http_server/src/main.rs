use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    let addr = "127.0.0.1:7878";
    let listner = TcpListener::bind(addr).unwrap();
    println!("start listening on {}", addr);

    for stream in listner.incoming() {
        let stream = stream.unwrap();

        println!("{:?}", stream);
        handle_connection(stream)
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    // for debug
    println!("Request: {:?}", buffer); // prints bytes string in the buffer.
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let response = "HTTP/1.1 200 OK\r\n\r\nHello!";

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
