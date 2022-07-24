use std::{io::Write, net::TcpStream};

fn main() -> std::io::Result<()> {
    let message = std::env::args().nth(1).unwrap();
    let mut stream = TcpStream::connect("127.0.0.1:8192")?;
    stream.write(message.as_bytes())?;
    Ok(())
}
