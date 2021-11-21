use bytes::Bytes;
use tokio::sync::mpsc;

#[derive(Debug)]
enum Command {
    Get { key: String },
    Set { key: String, val: Bytes },
}

#[tokio::main]
async fn main() -> mini_redis::Result<()> {
    // Create a new channel with a capacity of at most 32.
    let (tx, mut rx) = mpsc::channel(32);

    // The `Sender` handles are moved into the tasks. As there are two
    // tasks, we need a second `Sender`.
    let tx2 = tx.clone();

    // Spawn two tasks, one gets a key, the other sets a key
    let t1 = tokio::spawn(async move {
        let cmd = Command::Get {
            key: "hello".to_string(),
        };

        tx.send(cmd).await.unwrap();
    });

    let t2 = tokio::spawn(async move {
        let cmd = Command::Set {
            key: "foo".to_string(),
            val: "bar".into(),
        };

        tx2.send(cmd).await.unwrap();
    });

    // The `move` keyword is used to **move** ownership of `rx` into the task.
    let manager = tokio::spawn(async move {
        // Establish a connection to the server
        let mut client = mini_redis::client::connect("127.0.0.1:6379").await.unwrap();

        // Start receiving messages
        while let Some(cmd) = rx.recv().await {
            match cmd {
                Command::Get { key } => {
                    client.get(&key).await.unwrap();
                }
                Command::Set { key, val } => {
                    client.set(&key, val).await.unwrap();
                }
            }
        }
    });

    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();

    Ok(())
}
