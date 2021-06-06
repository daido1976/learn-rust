use std::net::TcpListener;

fn main() {
    let addr = "127.0.0.1:7878";
    let listner = TcpListener::bind(addr).unwrap();
    println!("start listening on {}", addr);

    for stream in listner.incoming() {
        let stream = stream.unwrap();

        println!("{:?}", stream);
        println!("connection established!")
    }
}
