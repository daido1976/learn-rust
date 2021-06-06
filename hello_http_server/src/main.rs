use std::{
    io::Read,
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
    println!("Request: {:?}", buffer); // prints bytes string in the buffer.
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}
