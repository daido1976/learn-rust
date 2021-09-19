use anyhow::Result;
use std::{io, net::UdpSocket};

pub fn communicate(addr: &str) -> Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:0")?;
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        socket.send_to(input.as_bytes(), addr)?;

        let mut buffer = [0; 1024];
        socket.recv_from(&mut buffer)?;
        print!("{}", String::from_utf8_lossy(&buffer));
    }
}
