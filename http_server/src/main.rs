use std::{
    fs::File,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    let addr = "127.0.0.1:7878";
    let listner = TcpListener::bind(addr).unwrap();
    println!("start listening on {}", addr);

    for stream in listner.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream)
    }
}

#[allow(clippy::unused_io_amount)]
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    // for debug
    println!("Request: {:?}", buffer); // prints bytes string in the buffer.
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1\r\n";
    let (status_line, file_path) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
    } else {
        ("HTTP/1.1 404 Not found\r\n\r\n", "404.html")
    };

    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    // https://doc.rust-jp.rs/book-ja/ch20-02-multithreaded.html から再開
}
