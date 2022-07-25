use std::{
    env::args,
    io::{Result, Write},
    net::TcpStream,
};

fn main() -> Result<()> {
    let args: Vec<String> = args().collect();
    let mut stream = TcpStream::connect(&args[1])?;
    stream.write(&args[2].as_bytes())?;
    Ok(())
}
