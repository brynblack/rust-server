use std::{io::Write, net::TcpStream};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let mut stream = TcpStream::connect(&args[1])?;
    stream.write(&args[2].as_bytes())?;
    Ok(())
}
