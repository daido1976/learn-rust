use anyhow::Result;
use std::{
    io::{self, BufRead, BufReader, Write},
    net::TcpStream,
};

/// connect makes a TCP connection to the specified IP address and port number.
pub fn connect(addr: &str) -> Result<()> {
    let mut stream = TcpStream::connect(addr)?;
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        stream.write_all(input.as_bytes())?;

        let mut reader = BufReader::new(&stream);
        let mut buffer = Vec::new();
        reader.read_until(b'\n', &mut buffer)?;
        print!("{}", String::from_utf8_lossy(&buffer));
    }
}
