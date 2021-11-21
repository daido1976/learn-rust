use bytes::Bytes;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() {
    // Bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    println!("Listening");

    let db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        // Clone the handle to the hash map.
        let db = db.clone();

        println!("Accepted");
        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}

async fn process(socket: TcpStream, db: Db) {
    // Connection, provided by `mini-redis`, handles parsing frames from the socket
    let mut connection = mini_redis::Connection::new(socket);

    // Use `read_frame` to receive a command from the connection.
    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match mini_redis::Command::from_frame(frame).unwrap() {
            mini_redis::Command::Set(cmd) => {
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                mini_redis::Frame::Simple("OK".to_string())
            }
            mini_redis::Command::Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    mini_redis::Frame::Bulk(value.clone())
                } else {
                    mini_redis::Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        // Write the response to the client
        connection.write_frame(&response).await.unwrap();
    }
}
