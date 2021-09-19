use anyhow::Result;
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

/// serve waits for a connection at the specified socket address.
pub fn serve(addr: &str) -> Result<()> {
    let listener = TcpListener::bind(addr)?;
    loop {
        let (stream, _) = listener.accept()?;
        thread::spawn(move || {
            handler(stream).unwrap_or_else(|e| error!("{}", e));
        });
    }
}

/// handler listens for input from the client and returns the same when received.
fn handler(mut stream: TcpStream) -> Result<()> {
    debug!("Handling data from {}", stream.peer_addr()?);
    let mut buffer = [0; 1024];
    loop {
        let nbytes = stream.read(&mut buffer)?;
        if nbytes == 0 {
            debug!("Client {} disconnected", stream.peer_addr()?);
            return Ok(());
        }
        let req = String::from_utf8_lossy(&buffer[..nbytes])
            .trim()
            .to_string();
        debug!("received request: {}", req);
        let res = format!("{}!!!\n", req);
        stream.write_all(res.as_bytes())?;
    }
}
