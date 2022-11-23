use anyhow::Result;
use std::net::UdpSocket;

/// serve responses at the specified socket address.
pub fn serve(addr: &str) -> Result<()> {
    let server_socket = UdpSocket::bind(addr)?;
    loop {
        let mut buffer = [0; 1024];
        let (size, src) = server_socket.recv_from(&mut buffer)?;
        debug!("received {} bytes from {}", size, src);
        let req = String::from_utf8_lossy(&buffer[..size]).trim().to_string();
        debug!("received request: {}", req);

        let res = format!("{}!!!\n", req);
        server_socket.send_to(res.as_bytes(), src)?;
    }
}
